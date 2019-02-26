# Kiln v0.2

[![](https://img.shields.io/crates/v/kiln.svg?style=for-the-badge)](https://crates.io/crates/kiln)
[![](https://img.shields.io/badge/-docs.rs-blue.svg?style=for-the-badge)](https://docs.rs/crate/kiln/)
![](https://img.shields.io/circleci/project/github/swissChili/kiln.svg?style=for-the-badge)

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
- Accessing rows
  - Getting columns from rows
  - Setting columns in rows

## Roadmap

- Implement O(1) joining (Easier said than done)
- Make this all thread-safe with async support (futures maybe?)

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
    let mut bob = users.insert(row!{
        name: "Bob",
        age: 24
    }).expect("Could not insert row");

    users.insert(row!{
        name: "Jeff",
        age: 24
    }).expect("Failed to insert");

    println!("Bob is {} years old", bob["age"].i32().unwrap());
    //=> Bob is 24 years old

    bob.set("age", 42);

    println!("Bob is now {} years old", bob["age"].i32().unwrap());

    for user in users.get("age", 24) {
        println!("24 year old named {}", user["name"].string().unwrap());
    }
    //=> 24 year old named Jeff
}
```
