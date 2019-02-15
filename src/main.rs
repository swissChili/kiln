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
    users.insert(row!{
        name: "Bob",
        age: 24
    }).expect("Could not insert row");

    // Get back a row where the name = "Bob"
    let users = users.get("name", "Bob");

    // Select just the ages from these rows
    for user in users {
        // Age is an Option because all columns can be empty
        println!("Bob is {} years old", user["age"].i32().unwrap());
        //=> Bob is I32(24) years old
    }
}
