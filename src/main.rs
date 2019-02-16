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

    users.insert(row!{
        name: "Jeff",
        age: 24
    }).expect("Failed to insert");

    // Get back a row where the name = "Bob"
    let user = users.get_one("name", "Bob").unwrap();

    println!("Bob is {} years old", user["age"].i32().unwrap());
    //=> Bob is 24 years old

    for user in users.get("age", 24) {
        println!("24 year old named {}", user["name"].string().unwrap());
    }
    //=> 24 year old named Jeff
    //=> 24 year old named Bob
}
