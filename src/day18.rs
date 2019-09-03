use std::collections::HashMap;

type Register = char;
type Number = isize;

#[derive(Clone, Debug)]
enum Value {
    Register(Register),
    Integer(Number),
}

#[derive(Clone, Debug)]
enum Instructions {
    Send(Value),
    Set(Register, Value),
    Add(Register, Value),
    Mul(Register, Value),
    Mod(Register, Value),
    // recovers frequency of last sound, but only when value is non-zero
    Recover(Register),
    // jumps only if x is greater than zero
    Jump(Value, Value),
}

struct Interpreter {
    registers: HashMap<Register, Number>,
    instructions: Vec<Instructions>,
    program_counter: isize,
    last_sound: Option<Number>,
    recovered_sound: Option<Number>,
}

#[derive(Debug)]
struct Partner {
    registers: HashMap<Register, Number>,
    instructions: Vec<Instructions>,
    program_counter: isize,
    network_q: Vec<Number>,
}

impl Interpreter {
    fn new(instructions: &[Instructions]) -> Interpreter {
        Interpreter {
            registers: HashMap::new(),
            instructions: instructions.to_vec(),
            program_counter: 0,
            last_sound: None,
            recovered_sound: None,
        }
    }
    fn operate<F>(&mut self, r: Register, v: Value, operand: F)
    where
        F: Fn(Number, Number) -> Number,
    {
        let v = self.value(v);
        let entry = self.registers.entry(r).or_insert(0);
        *entry = operand(*entry, v);
    }
    fn interpret_step(&mut self) -> Result<Option<Number>, ()> {
        use Instructions::*;
        if self.program_counter < 0 || self.program_counter >= self.instructions.len() as isize {
            Err(())
        } else {
            let instr = self.instructions[self.program_counter as usize].clone();
            match instr {
                Send(v) => {
                    self.step();
                    let v = self.value(v);
                    self.last_sound = Some(v);
                    Ok(Some(v))
                }
                Set(r, v) => {
                    let v = self.value(v);
                    self.registers.insert(r, v);
                    self.step();
                    println!("{:?}", self.registers);
                    Ok(None)
                }
                Add(r, v) => {
                    self.step();
                    self.operate(r, v, |a, b| a + b);
                    println!("{:?}", self.registers);
                    Ok(None)
                }
                Mul(r, v) => {
                    self.step();
                    self.operate(r, v, |a, b| a * b);
                    println!("{:?}", self.registers);
                    Ok(None)
                }
                Mod(r, v) => {
                    self.step();
                    self.operate(r, v, |a, b| a % b);
                    println!("{:?}", self.registers);
                    Ok(None)
                }
                Recover(v) => {
                    let v = *self.registers.get(&v).unwrap_or(&0);
                    self.step();
                    // if not zero, set recovered_sound to last_sound
                    if v != 0 {
                        self.recovered_sound = self.last_sound;
                        println!("set recovered sound to be {:?}", self.recovered_sound);
                        Ok(None)
                    } else {
                        Ok(None)
                    }
                }
                Jump(val, offset) => {
                    let v = self.value(val);
                    if v <= 0 {
                        self.step();
                        Ok(None)
                    } else {
                        self.program_counter += self.value(offset);
                        Ok(None)
                    }
                }
            }
        }
    }
    fn step(&mut self) {
        self.program_counter += 1;
    }
    fn value(&mut self, v: Value) -> Number {
        match v {
            Value::Integer(i) => i,
            Value::Register(r) => {
                let entry = self.registers.entry(r).or_insert(0);
                *entry
            }
        }
    }
    fn first_recover(&mut self) -> Option<Number> {
        while let Ok(_output) = self.interpret_step() {
            if self.recovered_sound.is_some() {
                break;
            }
        }
        self.recovered_sound
    }
}

#[derive(Debug)]
enum PartnerOutput {
    Send(Number),
    Waiting,
    OutOfRange,
    Continue,
}

impl PartnerOutput {
    fn is_out_of_range(&self) -> bool {
        match self {
            PartnerOutput::OutOfRange => true,
            _ => false,
        }
    }
    fn is_waiting(&self) -> bool {
        match self {
            PartnerOutput::Waiting => true,
            _ => false,
        }
    }
}

impl Partner {
    fn new(instructions: Vec<Instructions>, registers: HashMap<Register, Number>) -> Partner {
        Partner {
            registers,
            instructions,
            program_counter: 0,
            network_q: vec![],
        }
    }

    fn operate<F>(&mut self, r: Register, v: &Value, operand: F)
    where
        F: Fn(Number, Number) -> Number,
    {
        let v = self.value(v);
        let entry = self.registers.entry(r).or_insert(0);
        *entry = operand(*entry, v);
    }
    fn value(&mut self, v: &Value) -> Number {
        match v {
            Value::Integer(i) => *i,
            Value::Register(r) => {
                let entry = self.registers.entry(*r).or_insert(0);
                *entry
            }
        }
    }
    fn step(&mut self) {
        self.program_counter += 1;
    }
    fn interpret_step(&mut self) -> PartnerOutput {
        use Instructions::*;
        if self.program_counter < 0 || self.program_counter > self.instructions.len() as isize {
            PartnerOutput::OutOfRange
        } else {
            let instr = self.instructions[self.program_counter as usize].clone();
            print!("{:?} ", instr);
            match instr {
                Send(v) => {
                    self.step();
                    PartnerOutput::Send(self.value(&v))
                }
                Set(r, v) => {
                    let v = self.value(&v);
                    self.registers.insert(r, v);
                    // print!("({} => {}) ", r, v);
                    self.step();
                    PartnerOutput::Continue
                }
                Add(r, v) => {
                    self.step();
                    self.operate(r, &v, |a, b| a + b);
                    // print!("({}: {}) ", r, self.registers[&r]);
                    PartnerOutput::Continue
                }
                Mul(r, v) => {
                    self.step();
                    self.operate(r, &v, |a, b| a * b);
                    // print!("({}: {}) ", r, self.registers[&r]);
                    PartnerOutput::Continue
                }
                Mod(r, v) => {
                    self.step();
                    self.operate(r, &v, |a, b| a % b);
                    // print!("({}: {}) ", r, self.registers[&r]);
                    PartnerOutput::Continue
                }
                Recover(r) => {
                    // only step if there is something inside of whatever
                    // print!("({:?}) ", self.network_q);
                    if self.network_q.len() > 0 {
                        let v = self.network_q.remove(0);
                        self.registers.insert(r, v);
                        self.step();
                        PartnerOutput::Continue
                    } else {
                        PartnerOutput::Waiting
                    }
                }
                Jump(x, y) => {
                    let offset = if self.value(&x) != 0 {
                        self.value(&y)
                    } else {
                        1
                    };
                    self.program_counter += offset;
                    // print!("({}) ", self.program_counter);
                    PartnerOutput::Continue
                }
            }
        }
    }
    fn receive_value(&mut self, v: Number) {
        // thought supposed to step, not really though! since continuing!
        self.network_q.push(v);
    }
}

fn parse_input(i: &str) -> Vec<Instructions> {
    let mut ret = vec![];
    for line in i.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let result = match parts[0] {
            "set" => Instructions::Set(parse_to_register(parts[1]), parse_to_value(parts[2])),
            "add" => Instructions::Add(parse_to_register(parts[1]), parse_to_value(parts[2])),
            "mul" => Instructions::Mul(parse_to_register(parts[1]), parse_to_value(parts[2])),
            "mod" => Instructions::Mod(parse_to_register(parts[1]), parse_to_value(parts[2])),
            "snd" => Instructions::Send(parse_to_value(parts[1])),
            "rcv" => Instructions::Recover(parse_to_register(parts[1])),
            "jgz" => Instructions::Jump(parse_to_value(parts[1]), parse_to_value(parts[2])),
            _ => unimplemented!(),
        };
        ret.push(result);
    }
    ret
}

fn parse_to_register(i: &str) -> Register {
    assert!(i.len() == 1);
    i.chars().nth(0).unwrap()
}
fn parse_to_value(i: &str) -> Value {
    if let Ok(v) = i.parse::<Number>() {
        Value::Integer(v)
    } else {
        Value::Register(parse_to_register(i))
    }
}

fn parsed_part1(i: &[Instructions]) -> Option<Number> {
    let mut interpreter = Interpreter::new(i);
    interpreter.first_recover()
}
pub fn part1(i: &str) -> Option<Number> {
    let instructions = parse_input(i);
    parsed_part1(&instructions)
}

pub fn part2(i: &str) -> usize {
    let instructions = parse_input(i);
    let mut ret = 0;
    let mut iteration = 0;
    let mut part0 = Partner::new(instructions.clone(), vec![('p', 0)].into_iter().collect());
    let mut part1 = Partner::new(instructions.clone(), vec![('p', 1)].into_iter().collect());
    let parts = [&mut part0, &mut part1];
    let mut parts_index = 0;

    let result0 = parts[0].interpret_step();
    // print!("{:?}\t", result0);
    let result1 = parts[1].interpret_step();
    // println!("{:?}", result1);
    let mut results = [result0, result1];

    // how many times program1 sent a value
    // so as soon as program1 is done, can stop
    loop {
        match results[parts_index] {
            PartnerOutput::OutOfRange => {
                if results[1 - parts_index].is_out_of_range() {
                    break;
                } else {
                    results[parts_index] = PartnerOutput::OutOfRange;
                    parts_index = 1 - parts_index;
                }
            }
            PartnerOutput::Waiting => {
                if parts[parts_index].network_q.is_empty() {
                    if results[1 - parts_index].is_waiting() && parts[1 - parts_index].network_q.is_empty() {
                        break;
                    }
                    else {
                        results[parts_index] = PartnerOutput::Waiting;
                        parts_index = 1 - parts_index;
                    }
                } else {
                    results[parts_index] = parts[parts_index].interpret_step();
                }
            }
            PartnerOutput::Send(v) => {
                parts[1 - parts_index].receive_value(v);
                if parts_index == 1 {
                    ret += 1;
                }
                results[parts_index] = parts[parts_index].interpret_step();
            }
            PartnerOutput::Continue => results[parts_index] = parts[parts_index].interpret_step(),
        }
    }
    ret
}

#[test]
fn test_part1() {
    assert_eq!(part1("set a 1\nadd a 2\nmul a a\nmod a 5\nsnd a\nset a 0\nrcv a\njgz a -1\nset a 1\njgz a -2"), Some(4));
}

#[test]
fn test_parsed_part1() {
    use Instructions::*;
    use Value::*;
    assert_eq!(
        parsed_part1(&vec![
            Set('a', Integer(1)),
            Add('a', Integer(2)),
            Mul('a', Register('a')),
            Mod('a', Integer(5)),
            Send(Register('a')),
            Set('a', Integer(0)),
            Recover('a'),
            Jump(Register('a'), Integer(-1)),
            Set('a', Integer(1)),
            Jump(Register('a'), Integer(-2)),
        ]),
        Some(4)
    );
}

#[test]
fn test_part2() {
    assert_eq!(part2("snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d"), 3);
}

#[test]
fn test_similar_example() {

}