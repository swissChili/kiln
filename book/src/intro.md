# Introduction to Kiln

Kiln is a relational database implemented in Rust. Unlike databases like PostgreSQL and SQLite, Kiln does not operate on a client-server model. Instead, it is a stand-alone library that allows it to be used with zero dependencies. 

This book seeks to provide a high level introduction to kiln. It will assume little to no knowledge of other database systems, although some relational databse knowledge will of course be helpful.

Knowledge of the rust programming language and ecosystem is needed as this book will expect a decent knowledge of the language, and an understanding of general programming principles.

Kiln databases are made up of tables, which in turn are made up of rows and columns. One can easily imagine a kiln database as a spreadsheet, except that there is a fixed number of columns, each of which has a fixed data type
