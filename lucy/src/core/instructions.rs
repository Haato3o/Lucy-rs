use std::{fmt::Display, cmp::PartialEq};
use super::data::{AnyData, DataType};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub type Any = AnyData;

pub struct OperationData {
    pub typ: DataType,
    pub data: AnyData
}

impl Display for OperationData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.typ {
            DataType::Uint32 => write!(f, "{}", self.data.uint32),
            DataType::Uint64 => write!(f, "{}", self.data.uint64),
            DataType::Int32 => write!(f, "{}", self.data.int32),
            DataType::Int64 => write!(f, "{}", self.data.int64),
            DataType::Float => write!(f, "{}", self.data.float),
            DataType::Double => write!(f, "{}", self.data.double),
            DataType::String => write!(f, "{}", self.data.string.as_str()),
            DataType::Char => write!(f, "{}", self.data.char),
            DataType::Register => write!(f, "{:?}", self.data.register),
        }
    }
}

#[derive(Debug, FromPrimitive, Clone, Copy, PartialEq)]
pub enum Operations {
    MOV,
    PUSH,
    POP,
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    CMP,
    JNE,
    JMP,
    JE,
    INC,
    OR,
    AND,
    XOR,
    CALL,
    RET,

    STDOUT,
    STDIN,
    DMP,
    MALLOC,
    FREE,

    COUNT
}

impl Operations {
    pub fn is_operation(name: &String) -> bool {
        let mut instruction: Vec<String> = Vec::with_capacity(Operations::COUNT as usize);

        for inst in 0..(Operations::COUNT as i32) {
            let op: Operations = FromPrimitive::from_i32(inst).unwrap();

            instruction.push(format!("{:?}", op))
        }

        let primitive = name.to_uppercase();

        instruction.contains(&primitive)
    }

    pub fn from_string(name: &String) -> Option<Operations> {
        let mut instruction: Vec<String> = Vec::with_capacity(Operations::COUNT as usize);

        for inst in 0..(Operations::COUNT as i32) {
            let op: Operations = FromPrimitive::from_i32(inst).unwrap();

            instruction.push(format!("{:?}", op))
        }

        let primitive = name.to_uppercase();

        let i = instruction.iter().position(|ins| &primitive == ins);
        match i {
            Some(idx) => FromPrimitive::from_usize(idx),
            None => None,
        }
    }
}

pub trait OperationImpl {
    fn exec_mov(&mut self, register: &OperationData, any: &OperationData);
    fn exec_push(&mut self, any: &OperationData);
    fn exec_pop(&mut self, register: &OperationData);
    fn exec_add(&mut self, left: &OperationData, right: &OperationData);
    fn exec_sub(&mut self, left: &OperationData, right: &OperationData);
    fn exec_mul(&mut self, left: &OperationData, right: &OperationData);
    fn exec_div(&mut self, left: &OperationData, right: &OperationData);
    fn exec_mod(&mut self, left: &OperationData, right: &OperationData);
    fn exec_jmp(&mut self, address: &OperationData);
    fn exec_jne(&mut self, address: &OperationData);
    fn exec_je(&mut self, address: &OperationData);
    fn exec_dmp(&mut self, any: &OperationData);
    fn exec_cmp(&mut self, left: &OperationData, right: &OperationData);
}