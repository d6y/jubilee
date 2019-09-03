use std::fs;
mod parse;
extern crate soup;

use parse::BookAvailability;

fn main() -> Result<(), std::io::Error> {
    let html = fs::read_to_string("data/one-result.html")?;
    let maybe_available = BookAvailability::from_html(&html);
    println!("{:?}", maybe_available);
    Ok( () )
}

