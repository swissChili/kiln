use crate::column::*;

/// Wraps the type in a ColumnValue enum. Allows for doing
/// things like `123.to_cell()` instead of `ColumnValue::i32(123)`
/// All int types are cast to an i32 because it makes the db simpler.
pub trait ToCell {
    fn to_cell(&self) -> ColumnValue;
}

impl ToCell for i32 {
    fn to_cell(&self) -> ColumnValue {
        ColumnValue::I32(*self)
    }
}

impl ToCell for i64 {
    fn to_cell(&self) -> ColumnValue {
        ColumnValue::I32(*self as i32)
    }
}

impl ToCell for i16 {
    fn to_cell(&self) -> ColumnValue {
        ColumnValue::I32(*self as i32)
    }
}

impl ToCell for i8 {
    fn to_cell(&self) -> ColumnValue {
        ColumnValue::I32(*self as i32)
    }
}

impl ToCell for isize {
    fn to_cell(&self) -> ColumnValue {
        ColumnValue::I32(*self as i32)
    }
}

impl ToCell for str {
    fn to_cell(&self) -> ColumnValue {
        ColumnValue::Str(self.to_string())
    }
}

impl ToCell for &str {
    fn to_cell(&self) -> ColumnValue {
        ColumnValue::Str(self.to_string())
    }
}

impl ToCell for String {
    fn to_cell(&self) -> ColumnValue {
        ColumnValue::Str(self.to_string())
    }
}
