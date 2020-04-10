use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

use bincode;

// Can't `use` at build time, so must include.
include!("src/symbols.rs");

const SYMBOLS_DATABASE: &'static str = &"src/symbols.txt";
const MAIN_DEPENDENCIES: &[&'static str] = &[SYMBOLS_DATABASE, &"src/symbols.rs"];

fn build() {
    // Parse symbols
    let infile = File::open(SYMBOLS_DATABASE).expect(&format!("Could not open file '{}'", SYMBOLS_DATABASE));
    let mut infile = BufReader::new(infile);
    let mut buffer = String::new();
    infile.read_to_string(&mut buffer).expect("Couldn't read file into memory");
    let symbols_vec = from_string(&buffer).expect("Could not parse file");
    let symbols = Symbols(symbols_vec);

    // Write fully parsed input data structure.
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("parsed_symbols.bin");
    let outfile = File::create(dest_path).expect("Couldn't open output file");
    bincode::serialize_into(outfile, &symbols).expect("Couldn't serialize into file");

    // Tell cargo when to rerun.
    for f in MAIN_DEPENDENCIES {
        println!("cargo:rerun-if-changed={}", f);
    }
}

fn main() {
    build();
}