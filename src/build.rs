use std::time::SystemTime;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, Write};

include!("symbols.rs");

const SYMBOLS_DATABASE: &'static str = &"src/symbols.txt";
const MAIN_TEMPLATE: &'static str = &"src/bin/main.rs.in";
const MAIN_DEPENDENCIES: &[&'static str] = &[SYMBOLS_DATABASE, MAIN_TEMPLATE, &"src/symbols.rs"];
const MAIN_TARGET: &'static str = &"src/bin/main.rs";

fn build() {
    // Parse symbols
    let infile = File::open(SYMBOLS_DATABASE).expect(&format!("Could not open file '{}'", SYMBOLS_DATABASE));
    let mut infile = BufReader::new(infile);
    let mut buffer = String::new();
    infile.read_to_string(&mut buffer).expect("Couldn't read file into memory");
    let symbols_vec = from_string(&buffer).expect("Could not parse file");
    let symbols = Symbols(&symbols_vec);

    // Parse input template
    let mut infile = File::open(MAIN_TEMPLATE).expect(&format!("Could not open file '{}'", MAIN_TEMPLATE));
    let mut template = String::new();
    infile.read_to_string(&mut template).expect("Could not read template");

    // Fill template
    let main_body = template.replace("%SYMBOLS%", &format!("{}", symbols));

    // Write main.rs
    let mut outfile = File::create(MAIN_TARGET).expect("Couldn't open output file");
    outfile.write(main_body.as_bytes()).expect("Couldn't write file");
}

fn get_modified(fname: &str) -> Result<SystemTime, String> {
    let meta = fs::metadata(fname).expect(&format!("Could not stat file '{}'", fname));
    meta.modified().or(Err("Last modified time not supported on this platform".to_string()))
}

fn need_rebuild() -> bool {
    let main_target_modified = get_modified(MAIN_TARGET).unwrap();
    MAIN_DEPENDENCIES.iter().any(|f| get_modified(f).unwrap() >= main_target_modified)
}

fn main() {
    if need_rebuild() {
        build();
    }
}