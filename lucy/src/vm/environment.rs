use crate::{compiler::compiler::LucyInstruction, core::{data::{AnyData, DataType}, instructions::{OperationData, OperationImpl, Operations}, register::Register}, logger::log::{Log, Logger}};

pub struct LucyEnvironment {
    registers: Vec<OperationData>,
    pub pc: i64,
    stack: Vec<OperationData>,
}

impl LucyEnvironment {
    pub fn new() -> Self {
        let mut this = LucyEnvironment {
            registers: Vec::with_capacity(Register::COUNT as usize),
            pc: 0,
            stack: Vec::new()
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
            Operations::MOD => todo!(),
            Operations::CMP => todo!(),
            Operations::JNE => todo!(),
            Operations::JMP => self.exec_jmp(&instruction.arguments[0]),
            Operations::JE => todo!(),
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
        todo!()
    }

    fn exec_dmp(&mut self, any: &OperationData) {
        
        if any.typ == DataType::Register {
            let reg_id = any.data.register as usize;

            println!("{}", self.registers[reg_id]);
        } else {
            println!("{}", any);
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
}