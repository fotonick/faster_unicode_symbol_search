mod symbols;
mod search;

use std::fs::File;
use std::io::{BufReader};
use symbols::Symbols;

fn main() {
	let f = File::open("src/symbols.txt").expect("Could not open file 'symbols.txt'");
	let f = BufReader::new(f);
    let symbols = Symbols::from_file(f).expect("Could not parse file");
    println!("{:?}", symbols.search_symbols("rarr").take(10).collect::<Vec<String>>());
}
