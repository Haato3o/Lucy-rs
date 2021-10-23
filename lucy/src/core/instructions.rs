use super::register::Register;
use super::data::AnyData;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub type Any = AnyData;

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
    fn exec_mov(&self, register: Register, any: Any);
    fn exec_push(&self, any: Any);
    fn exec_pop(&self, register: Register);
    fn exec_add(&self, left: Any, right: Any);
    fn exec_sub(&self, left: Any, right: Any);
    fn exec_mul(&self, left: Any, right: Any);
    fn exec_div(&self, left: Any, right: Any);
    fn exec_mod(&self, left: Any, right: Any);
    fn exec_dmp(&self, any: Any);
}