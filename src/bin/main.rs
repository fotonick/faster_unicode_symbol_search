#![forbid(unsafe_code)]

use faster_unicode_symbol_search::symbols::{Symbol, Symbols};
use itertools::Itertools;
use std::env;
use std::io::{BufWriter, Write};
use std::process::exit;

use bincode;

const PARSED_SYMBOLS_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/parsed_symbols.bin"));

// Format for Alfred script filter display (Afred v3 JSON format)
// Ref: https://www.alfredapp.com/help/workflows/inputs/script-filter/json/
fn write_symbol<W>(w: &mut W, sym: &Symbol)
where
    W: Write,
{
    write!(
        w,
        "{{\"arg\": \"{}\", \"subtitle\": \"{}\", \"title\": \"{}\"}}",
        sym.symbol, sym.description, sym.symbol
    )
    .unwrap();
}

fn main() {
    let query = env::args().skip(1).join(" ");
    if query.is_empty() || query == "-h" || query == "--help" {
        println!("Usage: {} [query]", env::args().nth(0).unwrap());
        exit(2);
    }
    // let symbols = Symbols(SYMBOLS);
    let symbols: Symbols = bincode::deserialize(&PARSED_SYMBOLS_BYTES).unwrap();

    // Do the work
    let results: Vec<_> = symbols.search_symbols(&query);

    // Print
    let stdout = std::io::stdout();
    let handle = stdout.lock();
    let mut writer = BufWriter::new(handle);
    writer.write_all(b"{\"items\":[").unwrap();
    for symbol in results.iter().take(1) {
        write_symbol(&mut writer, &symbol);
    }
    for symbol in results.iter().skip(1) {
        writer.write_all(b", ").unwrap();
        write_symbol(&mut writer, &symbol);
    }
    writer.write_all(b"]}\n").unwrap();
    writer.flush().unwrap();
}
