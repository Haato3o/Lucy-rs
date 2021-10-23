use std::mem::ManuallyDrop;

use super::register::Register;

#[derive(Debug)]
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

pub union AnyData {
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
            uint32: val
        }
    }
}

impl From<u64> for AnyData {
    fn from(val: u64) -> Self {
        AnyData {
            uint64: val
        }
    }
}

impl From<i32> for AnyData {
    fn from(val: i32) -> Self {
        AnyData {
            int32: val
        }
    }
}

impl From<i64> for AnyData {
    fn from(val: i64) -> Self {
        AnyData {
            int64: val
        }
    }
}

impl From<f32> for AnyData {
    fn from(val: f32) -> Self {
        AnyData {
            float: val
        }
    }
}

impl From<f64> for AnyData {
    fn from(val: f64) -> Self {
        AnyData {
            double: val
        }
    }
}

impl From<&String> for AnyData {
    fn from(val: &String) -> Self {
        let str = String::from(val);
        let wrapper = ManuallyDrop::new(str);
        AnyData  {
            string: wrapper
        }
    }
}

impl From<char> for AnyData {
    fn from(val: char) -> Self {
        AnyData {
            char: val
        }
    }
}

impl From<Register> for AnyData {
    fn from(val: Register) -> Self {
        AnyData {
            register: val
        }
    }
}

pub union Address {
    pub uint64: i64,
    pub register: Register
}