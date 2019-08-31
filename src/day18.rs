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
    Recover(Value),
    Jump(Value, Value),
}

struct Interpreter {
    registers: HashMap<Register, Number>,
    instructions: Vec<Instructions>,
    program_counter: isize,
    last_sound: Option<Number>,
}

impl Interpreter {
    fn new(instructions: &[Instructions]) -> Interpreter {
        Interpreter {
            registers: HashMap::new(),
            instructions: instructions.to_vec(),
            program_counter: 0,
            last_sound: None,
        }
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
                    Ok(Some(self.value(v)))
                }
                Jump(val, offset) => {
                    let v = self.value(val);
                    if v == 0 {
                        self.step();
                        Ok(None)
                    } else {
                        self.program_counter += self.value(offset);
                        Ok(None)
                    }
                }
                _ => unimplemented!()
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
        None
    }
}

fn parsed_part1(i: &[Instructions]) -> Option<Number> {
    None
}
pub fn part1(i: &str) -> Option<Number> {
    None
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