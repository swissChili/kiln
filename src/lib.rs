use std::fs;
use std::path;
use std::collections::HashMap;
use std::vec::Vec;
use std::io;

mod column;
mod traits;
pub use crate::column::*;
pub use crate::traits::*;

pub struct Db {
    path: String,
    tables: Vec<Table>
}

#[derive(Debug, PartialEq)]
pub struct TableSpec {
    pub data: HashMap<String, Column>
}

#[derive(Debug)]
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
    pub fn insert(&self, value: HashMap<String, ColumnValue>) -> Result<(), io::Error> {
        for (k, v) in value {
            // Panic if the key doesn't exist
            let t = self.spec.data.get(&k).unwrap();
            if *t == match v {
                ColumnValue::Str(_) => Column::Str,
                ColumnValue::LongStr(_) => Column::LongStr,
                ColumnValue::I32(_) => Column::I32,
                ColumnValue::Byte(_) => Column::Byte,
            } {
                let id = "test";
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
        Ok(())
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
    pub fn get(&self, key: &str, val: ColumnValue) -> Vec<HashMap<String, ColumnValue>> {
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

    /// Returns a row from it's ID
    pub fn row(&self, id: &str) -> HashMap<String, ColumnValue> {
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
        map
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

/// A shorthand for getting rows that match a certain criteria.
/// Abstracts over the `table.get()` method to find all rows in
/// which a column matches a certain value specified.
/// ```rust
/// let users_named_bob = get_where( users { name: "bob" } );
/// for user in users_named_bob {
///     println!("{}", user.get("age"));
/// }
/// ```
#[macro_export]
macro_rules! get_where {
    ( $t:ident { $key:ident : $val:expr } ) => {
        $t.get(stringify!($key), $val.to_row())
    }
}

/// Select a single column from the rows given. Basically just a map func.
#[macro_export]
macro_rules! select {
    ( $rows:expr => $col:ident ) => {
        $rows.iter().map(|x| x.get(stringify!($col))).collect::<Vec<_>>()
    }
}

#[macro_export]
macro_rules! row {
    ( $( $key:ident : $val:expr ),* ) => {{
        extern crate kiln;
        let mut map: HashMap<String, kiln::ColumnValue> = std::collections::HashMap::new();
        $(
            map.insert(stringify!($key).to_string(), $val.to_row());
        )*
        map
    }}
}

impl Db {
    pub fn new(path: &str) -> Result<Self, std::io::Error> {
        let exists = path::Path::new(path).exists();
        if exists {
            Ok(Self {
                path: path.to_string(),
                tables: Vec::new(),
            })
        } else {
            fs::create_dir(path)?;
            Ok(Self {
                path: path.to_string(),
                tables: Vec::new(),
            })
        }
    }

    /// Parses a tables specfile (_spec/*) and returns a TableSpec object
    ///
    /// TODO: implement macro or function to create a Table object out
    /// of this spec.
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

    /// Create a new table in the db dir.
    pub fn create(&self, name: &str, tablespec: TableSpec) -> Result<Table, std::io::Error> {
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
