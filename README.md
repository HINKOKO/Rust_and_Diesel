# **RUST AND DIESEL ORM**

[![diesel logo](https://diesel.rs/assets/images/diesel_logo_stacked_black.png)](https://diesel.rs)

## **overview**

This repository is a code-along tutorial from the official Rust ORM [**Diesel**](https://diesel.rs/guides/getting-started.html)<br>
It's an absolute delight. <br>
In this project, we build a basic CRUD functionality in Rust, connected to a Database system (availables are: MySql, Postgresql, and SQLite) with the use of (some) **DIESEL** (Even though it's pretty bad for global warming) as an **ORM** <br>

<hr>
With diesel, writing SQL queries in Rust is a breeze. <br>
- Loading users from a database (a), or all the post belonging to a user (b):

```
(a)
SELECT * FROM users; // SQL world
// gets turned into
users::table.load(&mut connection) // Rust world

(b)
SELECT * FROM posts WHERE user_id = 1;
Post:belonging_to(user).load(&mut connection)
```

## **Prerequisites**

This tutorial requires you to have installed at least the following

- `rust` installed on your system, Note that diesel requires a rust version >= 1.65 (Check yours with `rustc --version``)
- `postgresql` solution as a database to ease things, otherwise you have to swap between the tutorial page and another exemples from their Github repo to accomodate things. I also used postgresql in my porfolio at Holberton so it was pretty convenient.
- `diesel_cli` which can be a pain to install depending on your system... Please feel free to contact me if you have any trouble installing the Diesel Command line !
