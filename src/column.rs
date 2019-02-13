#[derive(Debug, PartialEq)]
pub enum Column {
    Str,
    LongStr,
    I32,
    Byte,
}

#[derive(Debug)]
pub enum ColumnValue {
    Str(String),
    LongStr(String),
    I32(i32),
    Byte(String),
}
