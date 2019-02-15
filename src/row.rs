use crate::column::ColumnValue;
use std::ops::Index;
use std::collections::HashMap;
use crate::*;

#[derive(Debug, Clone)]
pub struct Row {
    vals: HashMap<String, ColumnValue>,
    table: Box<Table>,
}


impl Row {
    pub fn from_map(tab: &Table, from: HashMap<String, ColumnValue>) -> Self {
        Self {
            table: Box::new(tab.clone()),
            vals: from,
        }
    }
}

impl Index<&str> for Row {
    type Output = ColumnValue;

    fn index(&self, key: &str) -> &ColumnValue {
        &self.vals.get(key).unwrap()
    }
}
