#![feature(uniform_paths)]
extern crate uuid;

use uuid::Uuid;
use std::fs;
use std::path;
use std::collections::HashMap;
use std::vec::Vec;
use std::io;
use std::boxed::Box;

mod column;
mod traits;
mod row;
pub use crate::column::*;
pub use crate::traits::*;
pub use crate::row::*;

pub struct Db {
    path: String
}

#[derive(Debug, PartialEq, Clone)]
pub struct TableSpec {
    pub data: HashMap<String, Column>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Table {
    pub spec: TableSpec,
    name: String,
    path: String,
}

/// Take a ColumnValue and return a stringified representation of it
/// eg:
/// ```rust
/// println!("{}", stringify_col(12.to_col()));
/// // 12
/// ```
pub fn stringify_col(v: ColumnValue) -> String {
    match v {
        ColumnValue::Str(s) => s,
        ColumnValue::LongStr(s) => s,
        ColumnValue::I32(s) => format!("{}", s),
        ColumnValue::Byte(s) => s,
    }
}


impl Table {
    /// Insert a row into a Table. Rows can be constructed as HashMaps
    /// or using the row! macro as shown bellow:
    /// ```rust
    /// users.insert(row!{name: "bob", age: 12});
    /// ```
    /// Or manually by creating a HashMap<String, ColumnValue> object
    pub fn insert(&self, value: HashMap<String, ColumnValue>) -> Result<String, io::Error> {
        let id = &Uuid::new_v4().to_hyphenated().to_string();
        for (k, v) in value {
            // Panic if the key doesn't exist
            let t = self.spec.data.get(&k).unwrap();
            if *t == match v {
                ColumnValue::Str(_) => Column::Str,
                ColumnValue::LongStr(_) => Column::LongStr,
                ColumnValue::I32(_) => Column::I32,
                ColumnValue::Byte(_) => Column::Byte,
            } {
                let p = path::Path::new(&self.path);
                
                let f = stringify_col(v);

                let val = p.join("_data").join(id);
                fs::create_dir_all(&val)?;
                fs::write(&val.join(&k), &f)?;
                let idx = p.join("_index").join(&k).join(&f);
                fs::create_dir_all(&idx)?;
                fs::write(idx.join(id), "")?;
            }
        }
        Ok(id.to_string())
    }

    /// Gets all rows where a certain key == a certain value.
    /// For example, in the following table
    ///
    /// | age: i32 | name: str |
    /// 
    /// To get the names of all users of age n one would do the following
    /// ```rust
    /// users.get("age", n.to_col());
    /// ```
    pub fn get<T: ToCell>(&self, key: &str, v: T) -> Vec<Row> {
        let val = v.to_cell();

        let p = path::Path::new(&self.path);
        let matches = p.join("_index").join(key).join(stringify_col(val));
        //println!("Matches: {:?}", matches);
        let mut rows = Vec::new();

        for r in fs::read_dir(matches).unwrap() {
            let row = r.unwrap();
            let id = row.file_name()
                        .to_string_lossy()
                        .to_string();

            rows.push(id);
        }
        rows.iter().map(|x| self.row(&x)).collect::<Vec<_>>()
    }

    pub fn get_one<T: ToCell>(&self, key: &str, v: T) -> Option<Row> {
        let p = path::Path::new(&self.path);
        let matches = p.join("_index").join(key).join(stringify_col(v.to_cell()));

        for r in fs::read_dir(matches).unwrap() {
            let row = r.unwrap();
            let id = row.file_name()
                        .to_string_lossy()
                        .to_string();

            return Some(self.row(&id));
        }
        None
    }

    /// Returns a row from it's ID
    pub fn row(&self, id: &str) -> Row {
        let p = path::Path::new(&self.path);
        let mut map = HashMap::new();
        for k in fs::read_dir(p.join("_data").join(id)).unwrap() {
            let key = k.unwrap();
            let val = fs::read_to_string(key.path()).unwrap();
            let strkey = key.file_name()
                        .to_string_lossy()
                        .to_string();
            let val_type = &self.spec.data.get(&strkey).unwrap();
            map.insert(strkey,
                match val_type {
                    Column::Str => ColumnValue::Str(val),
                    Column::LongStr => ColumnValue::LongStr(val),
                    Column::Byte => ColumnValue::Byte(val),
                    Column::I32 => {
                        let i: i32 = val.parse().expect("Failed to parse file to i32");
                        ColumnValue::I32(i)
                    }
                });
        }
        Row::new(self, map, id.to_string())
    }

    /// Set a cell to a value. Takes the row, column, value, and old
    /// value as arguments and panics if it fails. It is not recommended
    /// to use this function, you should use the row.set(k, v) function
    /// instead.
    pub fn set_cell<T: ToCell>(&self,
            row: String, 
            col: String,
            val: T,
            old_val: ColumnValue) {

        let p = path::Path::new(&self.path);
        let idx = p.join("_index");
        let data = p.join("_data");
        let stringified = stringify_col(val.to_cell());
        let index_dir = idx.join(&col).join(&stringified);

        fs::create_dir_all(&index_dir).unwrap();
        fs::write(&index_dir.join(&row), "").unwrap();
        fs::write(data.join(&row).join(&col), &stringified).unwrap();
        fs::remove_file(&idx.join(&col).join(stringify_col(old_val)).join(row)).unwrap();
    }
}

/// Shorthand for creating a table spec:
/// ```rust
/// let users = table!{name: str, age: i32};
/// ```
#[macro_export]
macro_rules! table {
    ( $( $n:ident : $t:ident ),* ) => {{
        extern crate kiln;
        use std::collections::HashMap;
        let mut v: HashMap<String, kiln::Column> = HashMap::new();
        $(
            let s = stringify!($n).to_string();
            match stringify!($t) {
                "str" => v.insert(s, kiln::Column::Str),
                "block" => v.insert(s, kiln::Column::LongStr),
                "i32" => v.insert(s, kiln::Column::I32),
                "byte" => v.insert(s, kiln::Column::Byte),
                // So far just skips invalid tokens.
                _ => None
            };
        )*
        kiln::TableSpec{data:v}
    }}
}

/// Select a single column from the rows given. Basically just a map func.
#[macro_export]
macro_rules! select {
    ( $rows:expr => $col:ident ) => {
        $rows.iter().map(|x| x.get(stringify!($col))).collect::<Vec<_>>()
    }
}

/// Creates a new Row object from an abstract HashMap-like representation
/// ```rs
/// users.insert(row!{
///     name: "richard",
///     age: 83
/// });
/// ```
#[macro_export]
macro_rules! row {
    ( $( $key:ident : $val:expr ),* ) => {{
        extern crate kiln;
        use kiln::ToCell;
        let mut map = std::collections::HashMap::new();
        $(
            map.insert(stringify!($key).to_string(), $val.to_cell());
        )*
        map
    }}
}

impl Db {
    /// Create or connect to an existing database in the directory
    /// provided. If one exists, it will return a Db object in that
    /// directory, however it will not verify the integrity of the
    /// tables at creation. These are only verified when calling
    /// `db.table(name, spec)` which verifies that the spec of the
    /// table of that name matches that which is provided.
    pub fn new(path: &str) -> Result<Self, std::io::Error> {
        let exists = path::Path::new(path).exists();
        if exists {
            Ok(Self {
                path: path.to_string()
            })
        } else {
            fs::create_dir(path)?;
            Ok(Self {
                path: path.to_string()
            })
        }
    }

    /// Parses a tables specfile (_spec/*) and returns a TableSpec object
    fn spec(&self, table: &str) -> TableSpec {
        let t = path::Path::new(&self.path).join(table);
        let spec = t.join("_spec");

        let mut v: HashMap<String, Column> = HashMap::new();

        for i in fs::read_dir(spec).unwrap() {
            let col = i.unwrap().path();
            let data = match &*fs::read_to_string(&col)
                .expect("Unable to read specfile") {
                    "i32" => Some(Column::I32),
                    "byte" => Some(Column::Byte),
                    "str" => Some(Column::Str),
                    "longstr" => Some(Column::LongStr),
                    _ => None
                };
            v.insert(col.file_name()
                        .expect("Error in parsing specfile, could not get name of file")
                        .to_string_lossy()
                        .to_string()
                    , data.unwrap());
        }
        TableSpec{data:v}
    }

    /// Create or access a table by name and spec. Verifies
    /// that if the table exists, it matches the spec given.
    /// This will return an Err if the table spec can not be
    /// parsed. It is important to handle this error as it
    /// will lead to errors inserting data later on in the
    /// tables usage.
    pub fn table(&self, name: &str, tablespec: TableSpec) -> Result<Table, std::io::Error> {
        let p = path::Path::new(&self.path).join(name);
        if !&p.as_path().exists() {
            println!("Creating");
            fs::create_dir(p.clone())?;
            let spec = p.join("_spec");
            let idx = p.join("_index");
            fs::create_dir(&spec)?;
            fs::create_dir(&idx)?;
            fs::create_dir(p.join("_data"))?;

            // Create a specfile and index for each col
            for (name, t) in tablespec.data.into_iter() {
                // Write type to specfile
                fs::write(&spec.join(&name), 
                    match t {
                        Column::I32 => "i32",
                        Column::Byte => "byte",
                        Column::Str => "str",
                        Column::LongStr => "longstr",
                    })?;
                // Create the index dir for the col
                fs::create_dir(idx.join(&name))?;
            }
            Ok(
                Table {
                    spec: self.spec(name),
                    name: name.to_string(),
                    path: p.to_str().unwrap().to_string(),
                }
            )
        } else {
            let spec_found = self.spec(name);
            if spec_found == tablespec {
                Ok(
                    Table {
                        spec: tablespec,
                        name: name.to_string(),
                        path: p.to_str().unwrap().to_string(),
                    }
                )
            } else {
                Err (
                    std::io::Error::new(
                        std::io::ErrorKind::PermissionDenied,
                        "Table of same name exists with different spec"
                    )
                )
            }
        }
    }
}
