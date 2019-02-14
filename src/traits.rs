use crate::column::*;

/// Wraps the type in a ColumnValue enum. Allows for doing
/// things like `123.to_row()` instead of `ColumnValue::i32(123)`
/// All int types are cast to an i32 because it makes the db simpler.
pub trait ToRow {
    fn to_row(&self) -> ColumnValue;
}

impl ToRow for i32 {
    fn to_row(&self) -> ColumnValue {
        ColumnValue::I32(*self)
    }
}

impl ToRow for i64 {
    fn to_row(&self) -> ColumnValue {
        ColumnValue::I32(*self as i32)
    }
}

impl ToRow for i16 {
    fn to_row(&self) -> ColumnValue {
        ColumnValue::I32(*self as i32)
    }
}

impl ToRow for i8 {
    fn to_row(&self) -> ColumnValue {
        ColumnValue::I32(*self as i32)
    }
}

impl ToRow for isize {
    fn to_row(&self) -> ColumnValue {
        ColumnValue::I32(*self as i32)
    }
}

impl ToRow for str {
    fn to_row(&self) -> ColumnValue {
        ColumnValue::Str(self.to_string())
    }
}

impl ToRow for &str {
    fn to_row(&self) -> ColumnValue {
        ColumnValue::Str(self.to_string())
    }
}

impl ToRow for String {
    fn to_row(&self) -> ColumnValue {
        ColumnValue::Str(self.to_string())
    }
}
