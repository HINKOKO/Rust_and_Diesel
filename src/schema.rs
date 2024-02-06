// @generated automatically by Diesel CLI.
// table! macro creates a bunch of code based on the database schema to represent all of the tables & columns

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
