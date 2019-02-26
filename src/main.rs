#[macro_use]
extern crate kiln;

fn main() {
    // Create a new database in the `data` dir
    let db = kiln::Db::new("data").expect("Failed to create or access db");

    // Create or access a table "users" with col types int and string
    let users = db.table("users", table!{
        age: i32,
        name: str
    }).expect("Table with same name exists with different spec");

    // Insert a row into the users table
    let mut user = users.insert(row!{
        name: "Bob",
        age: 24
    }).expect("Could not insert row");

    users.insert(row!{
        name: "Jeff",
        age: 24
    }).expect("Failed to insert");

    println!("Bob is {} years old", user["age"].i32().unwrap());
    //=> Bob is 24 years old

    user.set("age", 42);

    println!("Bob is now {} years old", user["age"].i32().unwrap());

    for user in users.get("age", 24) {
        println!("24 year old named {}", user["name"].string().unwrap());
    }
    //=> 24 year old named Jeff
    //=> 24 year old named Bob
}
