# Kiln 

[![](https://img.shields.io/crates/v/kiln.svg?style=for-the-badge)](https://crates.io/crates/kiln)
[![](https://img.shields.io/badge/-docs.rs-blue.svg?style=for-the-badge)](https://docs.rs/crate/kiln/)

Kiln is a relational database implemented in Rust. Unlike databases like PostgreSQL and SQLite, Kiln does not operate on a client-server model. Instead, it is a stand-alone library that allows it to be used with zero dependencies. 

This is a very early version of the database. So far the only thing done is the format the database stores tables.

A high level [guide](https://swisschili.gitlab.io/kiln) is available that provides an introduction to Kiln.

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
    }).expect("Failed to insert again");

    // Get back a row where the name = "Bob"
    let user = users.get_one("name", "Bob").unwrap();

    println!("Bob is {} years old", user["age"].i32().unwrap());
    //=> Bob is 24 years old

    for user in users.get("age", 24) {
        println!("24 year old named {}", user["name"].string().unwrap())
    }
    //=> 24 year old named Jeff
    //=> 24 year old named Bob
}

```
