use std::mem::ManuallyDrop;

use super::register::Register;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DataType {
    Uint32,
    Uint64,
    Int32,
    Int64,
    Float,
    Double,
    String,
    Char,
    Register
}

#[derive(Debug, Clone)]
pub struct AnyData {
    pub uint32: u32,
    pub uint64: u64,
    pub int32: i32,
    pub int64: i64,
    pub float: f32,
    pub double: f64,
    pub string: ManuallyDrop<String>,
    pub char: char,
    pub register: Register
}

impl From<u32> for AnyData {
    fn from(val: u32) -> Self {
        AnyData {
            uint32: val,
            uint64: 0,
            int32: 0,
            int64: 0,
            float: 0.0,
            double: 0.0,
            string: ManuallyDrop::new(String::from("")),
            char: ' ',
            register: Register::COUNT
        }
    }
}

impl From<u64> for AnyData {
    fn from(val: u64) -> Self {
        AnyData {
            uint32: 0,
            uint64: val,
            int32: 0,
            int64: 0,
            float: 0.0,
            double: 0.0,
            string: ManuallyDrop::new(String::from("")),
            char: ' ',
            register: Register::COUNT
        }
    }
}

impl From<i32> for AnyData {
    fn from(val: i32) -> Self {
        AnyData {
            uint32: 0,
            uint64: 0,
            int32: val,
            int64: 0,
            float: 0.0,
            double: 0.0,
            string: ManuallyDrop::new(String::from("")),
            char: ' ',
            register: Register::COUNT
        }
    }
}

impl From<i64> for AnyData {
    fn from(val: i64) -> Self {
        AnyData {
            uint32: 0,
            uint64: 0,
            int32: 0,
            int64: val,
            float: 0.0,
            double: 0.0,
            string: ManuallyDrop::new(String::from("")),
            char: ' ',
            register: Register::COUNT
        }
    }
}

impl From<f32> for AnyData {
    fn from(val: f32) -> Self {
        AnyData {
            uint32: 0,
            uint64: 0,
            int32: 0,
            int64: 0,
            float: val,
            double: 0.0,
            string: ManuallyDrop::new(String::from("")),
            char: ' ',
            register: Register::COUNT
        }
    }
}

impl From<f64> for AnyData {
    fn from(val: f64) -> Self {
        AnyData {
            uint32: 0,
            uint64: 0,
            int32: 0,
            int64: 0,
            float: 0.0,
            double: val,
            string: ManuallyDrop::new(String::from("")),
            char: ' ',
            register: Register::COUNT
        }
    }
}

impl From<&String> for AnyData {
    fn from(val: &String) -> Self {
        let str = String::from(val);
        AnyData  {
            uint32: 0,
            uint64: 0,
            int32: 0,
            int64: 0,
            float: 0.0,
            double: 0.0,
            string: ManuallyDrop::new(str),
            char: ' ',
            register: Register::COUNT
        }
    }
}

impl From<char> for AnyData {
    fn from(val: char) -> Self {
        AnyData {
            uint32: 0,
            uint64: 0,
            int32: 0,
            int64: 0,
            float: 0.0,
            double: 0.0,
            string: ManuallyDrop::new(String::from("")),
            char: val,
            register: Register::COUNT
        }
    }
}

impl From<Register> for AnyData {
    fn from(val: Register) -> Self {
        AnyData {
            uint32: 0,
            uint64: 0,
            int32: 0,
            int64: 0,
            float: 0.0,
            double: 0.0,
            string: ManuallyDrop::new(String::from("")),
            char: ' ',
            register: val
        }
    }
}

pub union Address {
    pub uint64: i64,
    pub register: Register
}