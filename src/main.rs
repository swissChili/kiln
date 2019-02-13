#[macro_use]
extern crate kiln;
use kiln::Db;
use kiln::ToRow;
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
    map.insert("age".to_string(), 12.to_row());
    map.insert("name".to_string(), "Bob".to_row());

    users.insert(map);
    println!("{:?}", users.spec.data);
    println!("{:?}", users.row("test"));

    let users = get_where!( users { name: "Bob" } );
    let rows = select!(users => age);

    for row in rows {
        println!("row {:?}", row);
    }

}
