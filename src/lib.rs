use std::fs;
use std::path;
use std::collections::HashMap;
use std::vec::Vec;
use std::io;

pub struct Db {
    path: String,
    tables: Vec<Table>
}

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

#[derive(Debug)]
pub struct TableSpec {
    pub data: HashMap<String, Column>
}

#[derive(Debug)]
pub struct Table {
    pub spec: TableSpec,
    name: String,
    path: String,
}

impl Table {
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
                
                let f = match v {
                    ColumnValue::Str(s) => s,
                    ColumnValue::LongStr(s) => s,
                    ColumnValue::I32(s) => format!("{}", s),
                    ColumnValue::Byte(s) => s,
                };
            
                let val = p.join("_data").join(id);
                fs::create_dir_all(&val)?;
                fs::write(&val.join(&k), &f)?;
                let idx = p.join("_index").join(&f);
                fs::create_dir_all(&idx)?;
                fs::write(idx.join(id), "")?;
            }
        }
        Ok(())
    }

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

impl Db {
    pub fn new(path: &str) -> Result<Self, std::io::Error> {
        if path::Path::new(path).exists() {
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

    pub fn create(&self, name: &str, tablespec: TableSpec) -> Result<Table, std::io::Error> {
        let p = path::Path::new(&self.path).join(name);
        println!("{:?}", p.clone().as_path().exists());
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
            }
            Ok(
                Table {
                    spec: self.spec(name),
                    name: name.to_string(),
                    path: p.to_str().unwrap().to_string(),
                }
            )
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "DB already exists!"
            ))
        }
    }
}
