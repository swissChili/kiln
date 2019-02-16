# Basic Usage

To use the crate simply add this line to your project's `dependencies` section in your `cargo.toml`:

```toml
kiln = "0.1"
```

From there include the crate in your code like you would any other. Kiln has several useful macros so using the `#[macro_use]` attribute is suggested.

Before actually doing anything with Kiln it is important to have at least a surface level understanding of what sets it apart from other databases. Kiln operates as a single library, requiring no other dependencies at runtime.

Because of this, Kiln stores all of the database files in a directory, which houses everything kiln needs to function. Very little of the data is actually loaded into memory at a single time, which greatly simplifies the internal representation of the data. 

Creating or opening a database is as simple as specifying the directory in which it resides, as is shown bellow:


```rs
#[macro_use]
extern crate kiln;

fn main() {
    let db = kiln::Db::new("data").unwrap();
}
```

This will create the `data` directory if it does not exist, and create all the necessary files for it to function.

A database is made up of tables. Tables are type-safe constructs that store certain data, organized into rows and columns. Each column has a fixed type that all items in that column must conform to. Kiln provides a `table!` macro that eases creation of such tables.


```rs
let db = kiln::Db::new("data").unwrap();

let users = db.table("users", table!{
    name: str,
    age: i32,
    favorite_food: str
}).unwrap();
```

This will either create a table called "users" if it does not exist, or open it if it already exists with the same column types. If a table exists with different types, the function will return an Err variant. Such errors are unlikely to occur in a controlled environment unless someone is manually tampering with the DB files (in which case Kiln can not be expected to function correctly), but if they do occur, it is important to handle them immediately as they will almost surely lead to dangerous silent deaths later on if left unhandled.


Now that a table exists it is necessary to populate it with some data. Tables are made up of rows, each of which can be represented as a set of key-value pairs. Each key being the column, and each value being the content of the coresponding cell. A row can be constructed easily using the `row!` macro, which provides an abstraction layer over the HashMap used to represent a row's contents. The table macro itself is not aware of the types of each column, and as such, no type checking is done on the creation of a row. Type checking is handled once the row is inserted into a table.

```rs
let row = users.insert(row!{
    name: "Bob",
    age: 64,
    favorite_food: "Gas station brand lasagna"
}).unwrap();
```

This will insert a new row into the `users` table. This row has a uuid which can be used to select that row. The `insert` method returns that uuid as a String. A row can be selected by uuid using the `row` method.

```rs
let name = users.row(row)["name"].string().unwrap();
```

This is the simplest query possible. It gets a row by it's uuid, and selects the name which it specifies should be of type string. If all goes well, the name should be `"Bob"`, same as what was inserted into the db.

This isn't particularly useful though. To be remotely useful, a db must be able to query rows not only by their uuids, but also by a value. This is trivial in Kiln, as it should be in any database. The following example will query the database for all 64-year olds and print their names. The order the rows are returned can not be assumed to be consistant between queries.

```rs
for user in users.get("age", 64) {
    println!("64 year old named {}", user["name"].string().unwrap());
}
```

This should print the expected `64 year old named Bob`. Remember that each time the code is run, another row is inserted into the table, so if you run it 10 times you will have 10 64 year olds named Bob.
