#[macro_use]
extern crate kiln;
use kiln::Db;
use kiln::ColumnValue::*;
use std::collections::HashMap;

fn main() {
    let db = Db::new("data").expect("Failed to create or access db");

    let table = table!{
        age: i32,
        name: str
    };

    println!("{:?}", table);
    let users = db.create("users", table).unwrap();

    let mut map = HashMap::new();
    map.insert("age".to_string(), I32(12));
    map.insert("name".to_string(), Str("Bob".to_string()));

    users.insert(map);
    println!("{:?}", users.spec.data);
    println!("{:?}", users.row("test"));

}
