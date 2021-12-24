use crate::{compiler::compiler::LucyInstruction, core::{data::{AnyData, DataType, self}, instructions::{OperationData, OperationImpl, Operations}, register::Register}};

pub struct Flags {
    pub zf: bool
}

#[allow(dead_code)]
pub struct LucyEnvironment {
    flags: Flags,
    registers: Vec<OperationData>,
    pub pc: i64,
    stack: Vec<OperationData>,
}

impl LucyEnvironment {
    pub fn new() -> Self {
        let mut this = LucyEnvironment {
            registers: Vec::with_capacity(Register::COUNT as usize),
            pc: 0,
            stack: Vec::new(),
            flags: Flags { zf: false },
        };

        for _ in 0..this.registers.capacity() {
            this.registers.push(OperationData {
                typ: DataType::Int32,
                data: AnyData::from(0),
            });
        }

        this
    }

    pub fn execute(&mut self, instruction: &LucyInstruction) {
        // TODO: Return error or ()
        match instruction.op_code {
            Operations::MOV => self.exec_mov(&instruction.arguments[0], &instruction.arguments[1]),
            Operations::PUSH => todo!(),
            Operations::POP => todo!(),
            Operations::ADD => self.exec_add(&instruction.arguments[0], &instruction.arguments[1]),
            Operations::SUB => todo!(),
            Operations::MUL => todo!(),
            Operations::DIV => todo!(),
            Operations::MOD => self.exec_mod(&instruction.arguments[0], &instruction.arguments[1]),
            Operations::CMP => self.exec_cmp(&instruction.arguments[0], &instruction.arguments[1]),
            Operations::JNE => self.exec_jne(&instruction.arguments[0]),
            Operations::JMP => self.exec_jmp(&instruction.arguments[0]),
            Operations::JE => self.exec_je(&instruction.arguments[0]),
            Operations::INC => todo!(),
            Operations::OR => todo!(),
            Operations::AND => todo!(),
            Operations::XOR => todo!(),
            Operations::CALL => todo!(),
            Operations::RET => todo!(),
            Operations::STDOUT => todo!(),
            Operations::STDIN => todo!(),
            Operations::DMP => self.exec_dmp(&instruction.arguments[0]),
            Operations::MALLOC => todo!(),
            Operations::FREE => todo!(),
            Operations::COUNT => todo!(),
        }
    }
}

#[allow(unused_variables)]
impl OperationImpl for LucyEnvironment {
    fn exec_mov(&mut self, register: &OperationData, any: &OperationData) {
        assert!(register.typ == DataType::Register);
        let reg_id = register.data.register as usize;

        if any.typ == DataType::Register {
            let other_id = any.data.register as usize;
            
            self.registers[reg_id].typ = self.registers[other_id].typ.clone();
            self.registers[reg_id].data = self.registers[other_id].data.clone();
        } else {
            self.registers[reg_id].typ = any.typ.clone();
            self.registers[reg_id].data = any.data.clone();
        }
    }

    fn exec_push(&mut self, any: &OperationData) {
        todo!()
    }

    fn exec_pop(&mut self, register: &OperationData) {
        todo!()
    }

    fn exec_add(&mut self, left: &OperationData, right: &OperationData) {
        // TODO: Set flags
        if left.typ == DataType::Register {
            let reg_id = left.data.register as usize;
            let data = &self.registers[reg_id];
            
            if right.typ == DataType::Register {
                let other_id = right.data.register as usize;
                let other_data = &self.registers[other_id];

                assert!(other_data.typ == data.typ);
                
                match data.typ {
                    DataType::Uint32 => self.registers[reg_id].data.uint32 += other_data.data.uint32,
                    DataType::Uint64 => self.registers[reg_id].data.uint64 += other_data.data.uint64,
                    DataType::Int32 => self.registers[reg_id].data.int32 += other_data.data.int32,
                    DataType::Int64 => self.registers[reg_id].data.int64 += other_data.data.int64,
                    DataType::Float => self.registers[reg_id].data.float += other_data.data.float,
                    DataType::Double => self.registers[reg_id].data.double += other_data.data.double,
                    DataType::String => todo!(),
                    DataType::Char => self.registers[reg_id].data.char = (self.registers[reg_id].data.char as u8 + other_data.data.char as u8) as char,
                    DataType::Register => todo!(),
                }
            } else {
                assert!(right.typ == data.typ);

                match data.typ {
                    DataType::Uint32 => self.registers[reg_id].data.uint32 += right.data.uint32,
                    DataType::Uint64 => self.registers[reg_id].data.uint64 += right.data.uint64,
                    DataType::Int32 => self.registers[reg_id].data.int32 += right.data.int32,
                    DataType::Int64 => self.registers[reg_id].data.int64 += right.data.int64,
                    DataType::Float => self.registers[reg_id].data.float += right.data.float,
                    DataType::Double => self.registers[reg_id].data.double += right.data.double,
                    DataType::String => todo!(),
                    DataType::Char => self.registers[reg_id].data.char = (self.registers[reg_id].data.char as u8 + right.data.char as u8) as char,
                    DataType::Register => todo!(),
                }
            }


        } else {

        }
    }

    fn exec_sub(&mut self, left: &OperationData, right: &OperationData) {
        todo!()
    }

    fn exec_mul(&mut self, left: &OperationData, right: &OperationData) {
        todo!()
    }

    fn exec_div(&mut self, left: &OperationData, right: &OperationData) {
        todo!()
    }

    fn exec_mod(&mut self, left: &OperationData, right: &OperationData) {
        let mut left_data = left;
        let mut right_data = right;

        let is_register = left.typ == DataType::Register;
        let register_id = left.data.register as usize; 

        if is_register {
            left_data = &self.registers[register_id];
        }

        if right.typ == DataType::Register {
            let right_reg = right.data.register as usize;
            right_data = &self.registers[right_reg];
        }

        assert!(left_data.typ == right_data.typ);

        match left_data.typ {
            DataType::Uint32 => {
                let result = left_data.data.uint32 % right_data.data.uint32;

                if is_register {
                    self.registers[register_id].data.uint32 = result;
                    self.flags.zf = result == 0u32;
                }
            },
            DataType::Uint64 => {
                let result = left_data.data.uint64 % right_data.data.uint64;

                if is_register {
                    self.registers[register_id].data.uint64 = result;
                    self.flags.zf = result == 0u64;
                }
            },
            DataType::Int32 => {
                let result = left_data.data.int32 % right_data.data.int32;

                if is_register {
                    self.registers[register_id].data.int32 = result;
                    self.flags.zf = result == 0i32;
                }
            },
            DataType::Int64 => {
                let result = left_data.data.int64 % right_data.data.int64;

                if is_register {
                    self.registers[register_id].data.int64 = result;
                    self.flags.zf = result == 0i64;
                }
            },
            DataType::Float => {
                let result = left_data.data.float % right_data.data.float;

                if is_register {
                    self.registers[register_id].data.float = result;
                    self.flags.zf = result == 0f32;
                }
            },
            DataType::Double => {
                let result = left_data.data.double % right_data.data.double;

                if is_register {
                    self.registers[register_id].data.double = result;
                    self.flags.zf = result == 0f64;
                }
            },
            _ => assert!(false, "not supported"),
        }
    }

    fn exec_jmp(&mut self, address: &OperationData) {
        if address.typ == DataType::Register {
            let other_id = address.data.register as usize;
            let other_data = &self.registers[other_id];

            assert!(other_data.typ == DataType::Int64);
            self.pc = other_data.data.int64 - 1;
        } else {
            assert!(address.typ == DataType::Int64);
            self.pc = address.data.int64 - 1;
        }
    }
    
    fn exec_jne(&mut self, address: &OperationData) {
        
        if address.typ == DataType::Register {
            let register_id = address.data.register as usize;
            let register_data = &self.registers[register_id];

            assert!(register_data.typ == DataType::Int64);

            if !self.flags.zf {
                self.pc = register_data.data.int64 - 1;
            }
        } else {
            assert!(address.typ == DataType::Int64);

            if !self.flags.zf {
                self.pc = address.data.int64 - 1;
            }
        }
        
    }

    fn exec_je(&mut self, address: &OperationData) {
        if address.typ == DataType::Register {
            let register_id = address.data.register as usize;
            let register_data = &self.registers[register_id];

            assert!(register_data.typ == DataType::Int64);

            if self.flags.zf {
                self.pc = register_data.data.int64 - 1;
            }
        } else {
            assert!(address.typ == DataType::Int64);

            if self.flags.zf {
                self.pc = address.data.int64 - 1;
            }
        }
    }

    fn exec_cmp(&mut self, left: &OperationData, right: &OperationData) {
        let mut left_data = left;
        let mut right_data = right;

        if left.typ == DataType::Register {
            let left_register = left.data.register as usize;
            left_data = &self.registers[left_register];

        }
        if right.typ == DataType::Register {
            let right_register = right.data.register as usize;
            right_data = &self.registers[right_register];
        }

        assert!(left_data.typ == right_data.typ);

        match left_data.typ {
            DataType::Uint32 =>  self.flags.zf = left_data.data.uint32 == right_data.data.uint32,
            DataType::Uint64 =>  self.flags.zf = left_data.data.uint64 == right_data.data.uint64,
            DataType::Int32 =>  self.flags.zf = left_data.data.int32 == right_data.data.int32,
            DataType::Int64 =>  self.flags.zf = left_data.data.int64 == right_data.data.int64,
            DataType::Float =>  self.flags.zf = left_data.data.float == right_data.data.float, 
            DataType::Double =>  self.flags.zf = left_data.data.double == right_data.data.double,
            DataType::String =>  self.flags.zf = left_data.data.string == right_data.data.string,
            DataType::Char =>  self.flags.zf = left_data.data.char == right_data.data.char,
            DataType::Register =>  self.flags.zf = false,
        }
        
    }

    fn exec_dmp(&mut self, any: &OperationData) {
        
        if any.typ == DataType::Register {
            let reg_id = any.data.register as usize;

            println!("{}", self.registers[reg_id]);
        } else {
            println!("{}", any);
        }
        
    }

}