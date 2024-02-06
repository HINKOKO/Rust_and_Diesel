use diesel::prelude::*;
use crate::schema::posts;

// Queryable => will generate all the code needed to load a `Post`struct from a SQL query
// NOTE => Queryable assumes that order of fields in `Post`struct matches the columns in posts table (from schema.rs)
// Selectable => will generate code to construct a matching select clause based on your model type
#[derive(Queryable, Selectable)] 
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
	pub id: i32,
	pub title: String,
	pub body: String,
	pub published: bool,
}

// Function to create a new post
#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
	pub title: &'a str,
	pub body: &'a str,
}



