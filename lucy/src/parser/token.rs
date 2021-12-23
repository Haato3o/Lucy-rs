use std::fmt::Display;

use crate::core::{data::{AnyData, DataType}, instructions::Operations, register::Register};

pub type TokenData = AnyData;

pub enum TokenType {
    CHECKPOINT(String),
    GOTO(String),
    DATA(DataType, TokenData),
    INSTRUCTION(Operations),
    REGISTER(Register),
    COMMENT(String)
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::CHECKPOINT(str) => f.write_fmt(format_args!("<Checkpoint {}>", str)),
            TokenType::GOTO(str) => f.write_fmt(format_args!("<Goto {}>", str)),
            TokenType::DATA(typ, data) => match typ {
                DataType::Uint32 => f.write_fmt(format_args!("<Uint32 {}>", data.uint32)),
                DataType::Uint64 => f.write_fmt(format_args!("<Uint64 {}>", data.uint64)),
                DataType::Int32 => f.write_fmt(format_args!("<Int32 {}>", data.int32)),
                DataType::Int64 => f.write_fmt(format_args!("<Int64 {}>", data.int64)),
                DataType::Float => f.write_fmt(format_args!("<Float {}>", data.float)),
                DataType::Double => f.write_fmt(format_args!("<Double {}>", data.double)),
                DataType::String => f.write_fmt(format_args!("<String \"{}\">", data.string.as_str())),
                DataType::Char => f.write_fmt(format_args!("<Char '{}'>", data.char)),
                DataType::Register => f.write_fmt(format_args!("<Register {:?}>", data.register)),
            },
            TokenType::INSTRUCTION(inst) => f.write_fmt(format_args!("<Instruction {:?}>", inst)),
            TokenType::REGISTER(reg) => f.write_fmt(format_args!("<Register {:?}>", reg)),
            TokenType::COMMENT(str) => f.write_fmt(format_args!("<Comment \"{}\">", str)),
        }
    }
}