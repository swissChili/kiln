# Kiln 

[![](https://img.shields.io/crates/v/kiln.svg?style=for-the-badge)](https://crates.io/crates/kiln)
[![](https://img.shields.io/badge/-docs.rs-blue.svg?style=for-the-badge)](https://docs.rs/crate/kiln/)

Kiln is a relational database implemented in Rust. Unlike databases like PostgreSQL and SQLite, Kiln does not operate on a client-server model. Instead, it is a stand-alone library that allows it to be used with zero dependencies. 

This is a very early version of the database. So far the only thing done is the format the database stores tables.

## Features implemented so far

- Creating databases
- Creating and accessing existing tables
  - Accessing rows from tables
    - By row ID
    - By value (eg: find all rows where foo = bar)
  - Parse specfiles for type safe columns

## Roadmap

- Implement a `Row` type to allow for abstractions like `foo["bar"] = "baz"`
- Implement a `Rows` type that will abstract over the current `Vec<HashMap<String, Columnval>>`
- Implement O(1) joining (Easier said than done)

## Usage

Here is a simple example of working with one simple table. The structure of the queries is very different from how it would be accomplished in a tradition SQL database. This unique structure makes it possible to easily interact with the database using expressive, declarative Rust code.

```rust
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
    let people = users.get("name", "Bob");

    // Select just the ages from these rows
    for person in people {
        // Age is an Option because all columns can be empty
        println!("Bob is {} years old", person["age"].i32().unwrap());
        //=> Bob is I32(24) years old
    }
}
```
