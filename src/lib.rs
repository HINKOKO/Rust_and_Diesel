// Post struct into which we can read our data, and have diesel generate the names
// we'll use to reference tables and columns in our queries
pub mod models;
pub mod schema;

use diesel::pg::PgConnection;
use ::diesel::prelude::*;
use dotenvy::dotenv;
use std::env;



// ========== Establish a Database connection

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// Function to save a new post
use self::models::{NewPost, Post};

pub fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost {title , body};

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

// When we call .get_result() on an insert or update statement,
// it automatically adds RETURNING * to the end of the query
// and lets us load it into any struct that implements Queryable for the right types. Neat!
