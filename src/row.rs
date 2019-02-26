use crate::column::ColumnValue;
use std::ops::Index;
use std::collections::HashMap;
use crate::*;

#[derive(Debug, Clone)]
pub struct Row {
    vals: HashMap<String, ColumnValue>,
    table: Box<Table>,
    id: String,
}


impl Row {
    pub fn new(tab: &Table, from: HashMap<String, ColumnValue>, id: String) -> Self {
        Self {
            table: Box::new(tab.clone()),
            vals: from,
            id: id,
        }
    }

    /// Set a key to a value on a mutable row. The row's internal value
    /// cache is updated so the row **must** be mutable. Abstracts
    /// over table.set_cell().
    pub fn set<T: ToCell + Clone>(&mut self, k: &str, val: T) {
        let key = k.to_string();

        let old = self.vals.get(k).unwrap().clone();

        self.table.set_cell(self.id.to_owned(), key.clone(), val.clone(), old);
        self.vals.remove(k);
        self.vals.insert(key, val.to_cell());
    }
}

impl Index<&str> for Row {
    type Output = ColumnValue;

    fn index(&self, key: &str) -> &ColumnValue {
        &self.vals.get(key).unwrap()
    }
}
