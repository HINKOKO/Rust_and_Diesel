use self::models::*;
use diesel::prelude::*;
use diesel_demo::*;

fn main() {
	// import aliases to be able to be able to shorten call to posts
	// keep this import in this scope for not polluting
	use self::schema::posts::dsl::*;

	let connection = &mut establish_connection();
	let results = posts
		.filter(published.eq(true))
		.limit(5)
		.select(Post::as_select())
		.load(connection)
		.expect("Error loading posts");
	
	println!("Displaying {} posts", results.len());
	for post in results {
		println!("{}", post.title);
		println!("------------\n");
		println!("{}", post.body);
	}
}
