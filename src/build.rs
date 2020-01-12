use std::fs::File;
use std::io::{BufReader, Read, Write};

include!("symbols.rs");

fn main() {
	// Parse symbols
	let infile = File::open("src/symbols.txt").expect("Could not open file 'symbols.txt'");
	let mut infile = BufReader::new(infile);
    let mut buffer = String::new();
    infile.read_to_string(&mut buffer).expect("Couldn't read file into memory");
    let symbols = Symbols::from_string(&buffer).expect("Could not parse file");

    // Parse input template
	let mut infile = File::open("src/bin/main.rs.in").expect("Could not open file 'main.rs.in'");
	let mut template = String::new();
	infile.read_to_string(&mut template).expect("Could not read template");

	// Fill template
	let main_body = template.replace("%SYMBOLS%", &format!("{}", symbols));

	// Write main.rs
	let mut outfile = File::create("src/bin/main.rs").expect("Couldn't open output file");
    outfile.write(main_body.as_bytes()).expect("Couldn't write file");
}
