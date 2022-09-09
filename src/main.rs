pub mod core;
pub mod errors;
pub mod hashers;
pub mod models;
pub mod schema;
pub mod storers;
pub mod tokeners;

#[macro_use]
extern crate diesel;

fn main() {
    println!("Hello, world!");
}
