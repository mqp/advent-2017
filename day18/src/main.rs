#![feature(io)]
#![feature(entry_and_modify)]

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::error::Error;
use std::str::FromStr;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RegisterId(pub char);

impl FromStr for RegisterId {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RegisterId(s.chars().next().ok_or("Empty register provided!")?))
    }
}

#[derive(Debug, Clone, Copy)]
enum Value {
    Register(RegisterId),
    Integer(i64),
}

impl FromStr for Value {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(n) = s.parse::<i64>() {
            Ok(Value::Integer(n))
        } else {
            Ok(Value::Register(s.parse()?))
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Snd(Value),
    Rcv(RegisterId),
    Set(RegisterId, Value),
    Add(RegisterId, Value),
    Mul(RegisterId, Value),
    Mod(RegisterId, Value),
    Jgz(Value, Value),
}

impl FromStr for Instruction {
    type Err = Box<Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let kind = parts.next().ok_or("No instruction provided!")?;
        let arg = parts.next().ok_or("No argument provided!")?;
        match kind {
            "snd" => Ok(Instruction::Snd(arg.parse()?)),
            "rcv" => Ok(Instruction::Rcv(arg.parse()?)),
            "set" => Ok(Instruction::Set(arg.parse()?, parts.next().expect("No value provided!").parse()?)),
            "add" => Ok(Instruction::Add(arg.parse()?, parts.next().expect("No value provided!").parse()?)),
            "mul" => Ok(Instruction::Mul(arg.parse()?, parts.next().expect("No value provided!").parse()?)),
            "mod" => Ok(Instruction::Mod(arg.parse()?, parts.next().expect("No value provided!").parse()?)),
            "jgz" => Ok(Instruction::Jgz(arg.parse()?, parts.next().expect("No value provided!").parse()?)),
            _ => Err(From::from("Unrecognized instruction provided.")),
        }
    }
}

#[derive(Debug, Clone)]
struct ProgramState {
    pub pc: i64,
    pub pid: i64,
    pub terminated: bool,
    pub waiting: bool,
    pub registers: HashMap<RegisterId, i64>,
    pub outputs: VecDeque<i64>,
    pub total_outputs: u64,
}

impl ProgramState {
    fn new(pid: i64) -> Self {
        let register_names = "abcdefghijklmnopqrstuvwxyz".chars();
        let mut registers: HashMap<RegisterId, i64> = register_names.map(|n| (RegisterId(n), 0)).collect();
        registers.insert(RegisterId('p'), pid);
        Self {
            pc: 0, 
            pid: pid,
            terminated: false,
            waiting: false,            
            registers: registers,
            outputs: VecDeque::new(),
            total_outputs: 0
        }
    }

    fn running(&self) -> bool {
        !self.terminated && !self.waiting
    }

    fn produce(&mut self, val: i64) {
        self.outputs.push_back(val);
        self.total_outputs += 1;
    }

    fn consume(&mut self) -> Option<i64> {
        self.outputs.pop_front()
    }
}

fn value_of(registers: &HashMap<RegisterId, i64>, val: Value) -> i64 {
    match val {
        Value::Integer(n) => n,
        Value::Register(id) => *registers.get(&id).unwrap_or(&0),
    }
}

fn execute(myself: &mut ProgramState, other: &mut ProgramState, instr: Instruction) {
    match instr {
        Instruction::Set(reg, val) => { 
            let n = value_of(&myself.registers, val);
            myself.registers.entry(reg).and_modify(|x| { *x = n; });
            myself.pc += 1;
        }
        Instruction::Add(reg, val) => { 
            let n = value_of(&myself.registers, val);
            myself.registers.entry(reg).and_modify(|x| { *x += n; });
            myself.pc += 1;
        }
        Instruction::Mul(reg, val) => {
            let n = value_of(&myself.registers, val);
            myself.registers.entry(reg).and_modify(|x| { *x *= n; });
            myself.pc += 1;
        }
        Instruction::Mod(reg, val) => {
            let n = value_of(&myself.registers, val);
            myself.registers.entry(reg).and_modify(|x| { *x %= n; });
            myself.pc += 1;
        }
        Instruction::Snd(val) => {
            let n = value_of(&myself.registers, val);
            myself.produce(n);
            other.waiting = false;
            myself.pc += 1;
        }
        Instruction::Rcv(reg) => {
            match other.consume() {
                None => {
                    myself.waiting = true;
                },
                Some(n) => { 
                    myself.registers.entry(reg).and_modify(|x| { *x = n; });
                    myself.pc += 1;
                }
            }
        }
        Instruction::Jgz(test, offset) => {
            if value_of(&myself.registers, test) > 0 {
                myself.pc += value_of(&myself.registers, offset);            
            } else {
                myself.pc += 1;
            }
        }
    }
}

fn main() {
    let reader = BufReader::new(File::open("input").expect("Couldn't read input file."));
    let lines = reader.lines().map(|x| x.expect("Couldn't read line."));
    let instructions: Vec<Instruction> = lines.map(|x| x.trim().parse().expect("Couldn't parse instruction.")).collect();

    let mut x_state = ProgramState::new(0);
    let mut y_state = ProgramState::new(1);
    while x_state.running() || y_state.running() {
        while x_state.running() {
            let instr = instructions.get(x_state.pc as usize).unwrap();
            execute(&mut x_state, &mut y_state, *instr);
            if x_state.pc < 0 || (x_state.pc as usize) >= instructions.len() {
                x_state.terminated = true;
            }
        }
        while y_state.running() {
            let instr = instructions.get(y_state.pc as usize).unwrap();
            execute(&mut y_state, &mut x_state, *instr);
            if y_state.pc < 0 || (y_state.pc as usize) >= instructions.len() {
                y_state.terminated = true;
            }
        }
    }
    
    println!("State at termination: {:?} {:?}", x_state, y_state);
}
