extern crate bb;
extern crate diesel;

use self::bb::*;
use self::diesel::prelude::*;
use self::bb::models::post::*;

fn main() {
    use bb::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}