#[derive(Debug, PartialEq, Clone)]
pub enum Column {
    Str,
    LongStr,
    I32,
    Byte,
}

#[derive(Debug, Clone)]
pub enum ColumnValue {
    Str(String),
    LongStr(String),
    I32(i32),
    Byte(String),
}

impl ColumnValue {
    pub fn i32(&self) -> Option<i32> {
        use ColumnValue::*;
        match self {
            I32(i) => Some(*i),
            _ => None
        }
    }

    pub fn string(&self) -> Option<String> {
        use ColumnValue::*;
        match self {
            Str(i) => Some(i.to_string()),
            _ => None
        }
    }
}
