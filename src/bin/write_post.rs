use diesel_demo::*;
use std::io::{stdin, Read};

fn main() {
	let connection = &mut establish_connection();

	let mut title = String::new();
	let mut body = String::new();

	println!("What is the title of your post ?");
	stdin().read_line(&mut title).unwrap();
	let title = title.trim_end(); // remove trailing newline

	println!("\nOk fine ! Let's write {} post (Press {} when finished)", title, EOF);
	stdin().read_to_string(&mut body).unwrap();

	let post = create_post(connection, title, &body);
	println!("\nSaved dragt {} with id {}", title, post.id);

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";


#[cfg(windows)]
const EOF: &str = "CTRL+Z";


	
	
	
}
