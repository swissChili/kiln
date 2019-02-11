#[macro_use]
extern crate kiln;
use kiln::Db;

fn main() {
    let db = Db::new("data").expect("Failed to create or access db");

    let table = table!{
        age: i32,
        name: str
    };

    println!("{:?}", table);
    println!("{:?}", db.create("users", table));
}
