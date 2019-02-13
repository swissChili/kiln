# Kiln

Kiln is a relational database implemented in Rust. Unlike databases like PostgreSQL and SQLite, Kiln does not operate on a client-server model. Instead, it is a stand-alone library that allows it to be used with zero runtime dependencies. 

This is a very early version of the database. So far the only thing done is the format the database stores tables.

## Features implemented so far

- Creating databases
- Creating tables
  - `table!` macro shorthand allows for easy definitions
- Inserting rows into tables with runtime type checking
  - IDs are hardcoded. Need to implement some sort of hashing for random IDs.
- Get row by ID
- DB structure allows for future O(1) look up of rows by value. (eg: `get!(FROM users WHERE { name: "bob" })`)

## DB structure

This tree represents how the data is stored in the database. The contents of files is shown in parenthesis next to the file name.

```
data
└── users
    ├── _data
    │   └── 000
    │       ├── age  (12)
    │       └── name (Bob)
    ├── _index
    │   ├── age
    │   │   └── 12
    │   │       └── 000
    │   └── name
    │       └── Bob
    │           └── 000
    └── _spec
        ├── age  (i32)
        └── name (str)
```
