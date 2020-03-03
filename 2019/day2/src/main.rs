use std::fs;
use std::vec::Vec;

#[derive(Debug, PartialEq)]
enum OpCode {
    ADD,
    MULT,
    HALT,
}

#[derive(Debug)]
struct IntcodeProgram {
    pc: usize,
    instructions: Vec<usize>,
}

impl IntcodeProgram {
    fn new(instr: Vec<usize>) -> Self {
        Self {
            instructions: instr,
            pc: 0,
        }
    }

    fn evaluate_one_step(&mut self) -> OpCode {
        let op = int_to_opcode(*self.instructions.get(self.pc).unwrap());

        if op == OpCode::HALT {
            return OpCode::HALT;
        }

        let src1 = *self.instructions.get(self.pc + 1).unwrap();
        let src2 = *self.instructions.get(self.pc + 2).unwrap();
        let dest = *self.instructions.get(self.pc + 3).unwrap();

        let val1 = *self.instructions.get(src1).unwrap();
        let val2 = *self.instructions.get(src2).unwrap();

        self.pc += 4;
        match op {
            OpCode::ADD => {
                let sum = val1 + val2;
                self.instructions.push(sum);
                self.instructions.swap_remove(dest);

                OpCode::ADD
            }
            OpCode::MULT => {
                let product = val1 * val2;
                self.instructions.push(product);
                self.instructions.swap_remove(dest);

                OpCode::MULT
            }
            _ => panic!("invalid op code: {:?}", op),
        }
    }

    fn evaluate(&mut self) {
        let mut op = self.evaluate_one_step();

        while op != OpCode::HALT {
            op = self.evaluate_one_step();
        }
    }
}

fn int_to_opcode(n: usize) -> OpCode {
    match n {
        1 => OpCode::ADD,
        2 => OpCode::MULT,
        99 => OpCode::HALT,
        _ => panic!("invalid opcode: {:?}", n),
    }
}

fn main() {
    let s = fs::read_to_string("input-big.txt").unwrap();
    let s = s.trim();

    let instructions = s
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut my_program = IntcodeProgram::new(instructions);
    my_program.evaluate();
    println!("{:?}", my_program);
}
