use std::io::BufRead;
use rayon::prelude::*;

#[derive(Clone, Debug)]
pub struct Symbol {
	pub symbol: String,
	pub description: String,
	pub hidden_description: String,
}

#[derive(Clone, Debug)]
pub struct Symbols(pub Vec<Symbol>);

#[derive(Debug)]
pub enum SymbolError {

}

impl Symbols {
	pub fn from_file<F>(mut infile: F) -> Result<Symbols, SymbolError>
		where F: BufRead
	{
		let mut buffer = String::new();
		infile.read_to_string(&mut buffer).expect("Couldn't read file into memory");
		Symbols::from_string(&buffer)
	}

	pub fn from_string(buffer: &str) -> Result<Symbols, SymbolError> {
		let symbols: Result<Vec<Symbol>, SymbolError> = buffer.par_lines().map(|line| parse_symbol(&line)).collect();
		symbols.map(Symbols)
	}
}

fn parse_symbol(line: &str) -> Result<Symbol, SymbolError> {
	let delim_pos = line.find("| ").expect(&format!("Expected '| ' delimiter in line {}", line));
	let symbol = &line[..delim_pos];
	let description = &line.get(delim_pos + 2..).expect(&format!("Expected text after '| ' delimiter in line {}", line));
	let main_description;
	let hidden_description;
	if let Some(delim_pos) = description.find("# ") {
		main_description = description[..delim_pos].to_owned();
		hidden_description = description.get(delim_pos + 2..).expect(&format!("Expected text after '# ' delimiter in line {}", line)).to_owned();
	} else {
		main_description = description.to_string();
		hidden_description = "".to_string();
	}
	Ok(Symbol {
		symbol: symbol.to_string(),
		description: main_description,
		hidden_description: hidden_description
	})
}