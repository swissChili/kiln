# Kiln 

[crates.io](![](https://img.shields.io/crates/v/kiln.svg?style=for-the-badge))
[docs.rs](https://docs.rs/crate/kiln/)

Kiln is a relational database implemented in Rust. Unlike databases like PostgreSQL and SQLite, Kiln does not operate on a client-server model. Instead, it is a stand-alone library that allows it to be used with zero runtime dependencies. 

This is a very early version of the database. So far the only thing done is the format the database stores tables.

## Features implemented so far

- Creating databases
- Creating tables
  - `table!` macro shorthand allows for easy definitions
- Inserting rows into tables with runtime type checking
  - IDs are hardcoded. Need to implement some sort of hashing for random IDs.
- Get row by ID
- DB structure allows for future O(1) look up of rows by value.

## Usage

Here is a simple example of working with one simple table. The structure of the queries is very different from how it would be accomplished in a tradition SQL database. This unique structure makes it possible to easily interact with the database using expressive, declarative Rust code.

```rust
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
    for age in select!(users => age) {
        // Age is an Option because all columns can be empty
        println!("Bob is {:?} years old", age.unwrap());
        //=> Bob is I32(24) years old
    }
}
```
