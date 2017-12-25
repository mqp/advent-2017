#![feature(io)]

use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Action {
    pub mov: i32,
    pub write: i32,
    pub next: char,
}

#[derive(Debug, Clone)]
struct State {
    pub actions: HashMap<i32, Action>
}

#[derive(Debug, Clone)]
struct Tape {
    pub data: HashMap<i32, i32>,
    pub head: i32,
}

impl Tape where {
    fn new() -> Self {
        Self { data: HashMap::new(), head: 0 }
    }
    fn read(&self) -> i32 {
        self.data.get(&self.head).map(|&x| x).unwrap_or_default()
    }
    fn write(&mut self, val: i32) {
        self.data.insert(self.head, val);
    }
    fn mov(&mut self, offset: i32) {
        self.head += offset;
    }
    fn checksum(&self) -> i32 {
        self.data.values().sum()
    }
}

#[derive(Debug, Clone)]
struct Machine {
    pub states: HashMap<char, State>,
    pub tape: Tape,
    pub steps: u64,
}

fn step(machine: &mut Machine, state_id: char) -> char {
    let state = machine.states.get(&state_id).expect(&format!("Invalid state specified!"));
    let val = machine.tape.read();
    let action = state.actions.get(&val).expect(&format!("No action for state value {}!", val));
    machine.tape.write(action.write);
    machine.tape.mov(action.mov);
    machine.steps += 1;
    action.next
}

fn main() {
    let mut machine = Machine {
        steps: 0,
        tape: Tape::new(),
        states: HashMap::new(),
    };
    let mut state_a = State { actions: HashMap::new() };
    state_a.actions.insert(0, Action { write: 1, mov: 1, next: 'B' });
    state_a.actions.insert(1, Action { write: 0, mov: -1, next: 'E' });
    let mut state_b = State { actions: HashMap::new() };
    state_b.actions.insert(0, Action { write: 1, mov: -1, next: 'C' });
    state_b.actions.insert(1, Action { write: 0, mov: 1, next: 'A' });
    let mut state_c = State { actions: HashMap::new() };
    state_c.actions.insert(0, Action { write: 1, mov: -1, next: 'D' });
    state_c.actions.insert(1, Action { write: 0, mov: 1, next: 'C' });
    let mut state_d = State { actions: HashMap::new() };
    state_d.actions.insert(0, Action { write: 1, mov: -1, next: 'E' });
    state_d.actions.insert(1, Action { write: 0, mov: -1, next: 'F' });
    let mut state_e = State { actions: HashMap::new() };
    state_e.actions.insert(0, Action { write: 1, mov: -1, next: 'A' });
    state_e.actions.insert(1, Action { write: 1, mov: -1, next: 'C' });
    let mut state_f = State { actions: HashMap::new() };
    state_f.actions.insert(0, Action { write: 1, mov: -1, next: 'E' });
    state_f.actions.insert(1, Action { write: 1, mov: 1, next: 'A' });
    machine.states.insert('A', state_a);
    machine.states.insert('B', state_b);
    machine.states.insert('C', state_c);
    machine.states.insert('D', state_d);
    machine.states.insert('E', state_e);
    machine.states.insert('F', state_f);
    
    let mut curr_state = 'A';
    for _ in 0..12386363 {
        curr_state = step(&mut machine, curr_state);
    }
    println!("Checksum is: {}", machine.tape.checksum());
}
