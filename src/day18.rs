use std::collections::HashMap;

type Register = char;
type Number = isize;

#[derive(Clone)]
enum Value {
    Register(Register),
    Integer(Number),
}

#[derive(Clone)]
enum Instructions {
    Send(Value),
    Set(Register, Value),
    Add(Register, Value),
    Mul(Register, Value),
    Mod(Register, Value),
    // recovers frequency of last sound, but only when value is non-zero
    Recover(Value),
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
    fn operate<F>(&mut self, r: Register, v: Value, operand: F) where F: Fn(Number, Number) -> Number {
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
                    let v = self.value(v);
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
        while let Ok(output) = self.interpret_step() {
            if self.recovered_sound.is_some() {
                break
            }
        }
        self.recovered_sound
    }
}

fn parse_input(i: &str) -> Vec<Instructions> {
    let mut ret = vec![];
    for line in i.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let result = match parts[0] {
            "set" => {
                Instructions::Set(parse_to_register(parts[1]), parse_to_value(parts[2]))
            }
            "add" => {
                Instructions::Add(parse_to_register(parts[1]), parse_to_value(parts[2]))
            }
            "mul" => {
                Instructions::Mul(parse_to_register(parts[1]), parse_to_value(parts[2]))
            }
            "mod" => {
                Instructions::Mod(parse_to_register(parts[1]), parse_to_value(parts[2]))
            }
            "snd" => {
                Instructions::Send(parse_to_value(parts[1]))
            }
            "rcv" => {
                Instructions::Recover(parse_to_value(parts[1]))
            }
            "jgz" => {
                Instructions::Jump(parse_to_value(parts[1]), parse_to_value(parts[2]))
            }
            _ => unimplemented!()
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
    0
}

#[test]
fn test_part1() {
    assert_eq!(part1("set a 1\nadd a 2\nmul a a\nmod a 5\nsnd a\nset a 0\nrcv a\njgz a -1\nset a 1\njgz a -2"), Some(4));
}

#[test]
fn test_parsed_part1() {
    use Instructions::*;
    use Value::*;
    assert_eq!(parsed_part1(&vec![
        Set('a', Integer(1)),
        Add('a', Integer(2)),
        Mul('a', Register('a')),
        Mod('a', Integer(5)),
        Send(Register('a')),
        Set('a', Integer(0)),
        Recover(Register('a')),
        Jump(Register('a'), Integer(-1)),
        Set('a', Integer(1)),
        Jump(Register('a'), Integer(-2)),
    ]), Some(4));
}