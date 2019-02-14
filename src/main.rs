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

    println!("Creating table: {:?}", table);
    let users = db.create("users", table).expect("Table exists with different spec");


    users.insert(row!{
        name: "Bob",
        age: 12
    });

    println!("Spec:     {:?}", users.spec.data);
    println!("Row test: {:?}", users.row("test"));

    let users = get_where!( users { name: "Bob" } );
    let ages = select!(users => age);

    for age in ages {
        println!("User's age: {:?}", age.unwrap());
    }

}
