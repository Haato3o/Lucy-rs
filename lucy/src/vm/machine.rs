use crate::compiler::compiler::LucyInstruction;

use super::environment::LucyEnvironment;

type Program = Vec<LucyInstruction>;

pub struct LucyMachine {
    environment: LucyEnvironment,
    program: Program
}

impl LucyMachine {
    pub fn new(program: Program) -> Self {
        LucyMachine { environment: LucyEnvironment::new(), program: program }
    }

    pub fn run(&mut self) {
        while (self.environment.pc as usize) < self.program.len() {
            let instruction = &self.program[self.environment.pc as usize];

            self.environment.execute(instruction);
            self.environment.pc += 1;
        }
    }

    pub fn run_next(&mut self, program: Program) {
        self.environment.pc = 0;
        self.program = program;

        self.run();
    }
}