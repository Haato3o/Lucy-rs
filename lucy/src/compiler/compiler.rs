use std::{collections::HashMap, collections::VecDeque};
use crate::{core::{data::{AnyData, DataType}, instructions::Operations}, logger::log::{Log, Logger}, parser::token::TokenType};

type Program = Vec<LucyInstruction>;

pub struct OperationParameter {
    pub typ: DataType,
    pub data: AnyData
}

pub struct LucyInstruction {
    pub instruction: Operations,
    pub arguments: Vec<OperationParameter>
}

pub struct LucyCompiler {}

impl LucyCompiler {
    pub fn compile(tokens: &Vec<TokenType>) -> Program {
        
        let checkpoints = LucyCompiler::index_checkpoints(tokens);
        let mut program: Program = Program::new();
        let mut args: VecDeque<OperationParameter> = VecDeque::new();
        let mut queued_instruction: Operations = Operations::COUNT;

        for token in tokens {
            match token {
                TokenType::CHECKPOINT(_) => continue,
                TokenType::COMMENT(_) => continue,
                TokenType::GOTO(checkpoint) => {
                    match checkpoints.get(checkpoint) {
                        Some(i) => {
                            args.push_back(
                                OperationParameter { typ: DataType::Int64, data: AnyData::from(*i as i64) }
                            );
                            
                        },
                        None => {},
                    }
                },
                TokenType::DATA(typ, data) => {
                    unsafe {
                        match typ {
                            DataType::Uint32 => args.push_back(
                                OperationParameter { typ: DataType::Uint32, data: AnyData::from(data.uint32) }
                            ),
                            DataType::Uint64 => args.push_back(
                                OperationParameter { typ: DataType::Uint64, data: AnyData::from(data.uint64) }
                            ),
                            DataType::Int32 => args.push_back(
                                OperationParameter { typ: DataType::Int32, data: AnyData::from(data.int32) }
                            ),
                            DataType::Int64 => args.push_back(
                                OperationParameter { typ: DataType::Int64, data: AnyData::from(data.int64) }
                            ),
                            DataType::Float => args.push_back(
                                OperationParameter { typ: DataType::Float, data: AnyData::from(data.float) }
                            ),
                            DataType::Double => args.push_back(
                                OperationParameter { typ: DataType::Double, data: AnyData::from(data.double) }
                            ),
                            DataType::String => args.push_back(
                                OperationParameter { typ: DataType::String, data: AnyData::from(&data.string.to_string()) }
                            ),
                            DataType::Char => args.push_back(
                                OperationParameter { typ: DataType::Char, data: AnyData::from(data.char) }
                            ),
                            DataType::Register => args.push_back(
                                OperationParameter { typ: DataType::Register, data: AnyData::from(data.register) }
                            ),
                        }
                    }
                },
                TokenType::REGISTER(reg) => args.push_back(
                    OperationParameter { typ: DataType::Register, data: AnyData::from(*reg) }
                ),
                TokenType::INSTRUCTION(ins) => {
                    if queued_instruction != Operations::COUNT {
                        let mut vec: Vec<OperationParameter> = Vec::with_capacity(args.len());

                        while args.len() > 0 {
                            vec.push(args.pop_front().unwrap());
                        }

                        program.push(
                            LucyInstruction { instruction: queued_instruction, arguments: vec }
                        );
                    }
                    queued_instruction = *ins;
                },
            }
        }
        
        if args.len() > 0 {
            let mut vec: Vec<OperationParameter> = Vec::with_capacity(args.len());

            while args.len() > 0 {
                vec.push(args.pop_front().unwrap());
            }

            program.push(
                LucyInstruction { instruction: queued_instruction, arguments: vec }
            );

            queued_instruction = Operations::COUNT;
        }

        program
    }

    fn index_checkpoints(tokens: &Vec<TokenType>) -> HashMap<String, usize> {
        let mut idx: usize = 0;
        let mut checkpoints: HashMap<String, usize> = HashMap::new();

        for tkn in tokens {
            match tkn {
                TokenType::CHECKPOINT(cp) => {
                    let mut chars= cp.chars();
                    chars.next();
                    
                    checkpoints.insert(String::from(chars.as_str()), idx);
                },
                TokenType::INSTRUCTION(_) => {
                    idx += 1;
                    continue;
                },
                _ => continue
            }
        }

        checkpoints
    }
}