//! Advent of Code 2017 Day 5

// FIXME: Allow unused stuff while we are working.
#![allow(dead_code)]

/// The evaluation state.
#[derive(Debug, Default)]
struct State {
    /// Our current position in the program.
    exec_ptr: i32,
    /// The number of jumps in our program.
    base: usize,
    /// The program that we are evaluating.
    program: Vec<i32>,
    /// The number of steps that we have progressed.
    num_steps: usize,
}

impl State {
    fn new(path: &str) -> Self {
        let s = std::fs::read_to_string(path).expect("valid path");
        let s = s.trim();
        let base: usize = s.lines().count();
        let program: Vec<i32> = s.lines().map(|l| l.parse::<i32>().unwrap()).collect();
        dbg!(&program);
        Self {
            exec_ptr: 0,
            base,
            program,
            num_steps: 0,
        }
    }

    fn tick(&mut self) {
        self.num_steps += 1;
        let instr = self
            .program
            .get_mut(self.exec_ptr as usize)
            .expect("exec_ptr in bounds");
        self.exec_ptr += *instr;
        *instr += 1;
    }

    pub fn evaluate(mut self) -> usize {
        while self.exec_ptr >= 0 && self.exec_ptr < self.base as i32 {
            // dbg!(&self);
            self.tick();
        }
        self.num_steps
    }
}

pub fn foo(path: &str) -> u32 {
    let state = State::new(path);
    state.evaluate() as u32
}
