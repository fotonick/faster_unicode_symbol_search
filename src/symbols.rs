use std::fmt;
use serde::{Deserialize, Serialize};

// Use owned strings instead of references to original string for serializability
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Symbol {
    pub symbol: String,
    pub description: String,
    pub hidden_description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Symbols(pub Vec<Symbol>);

#[derive(Debug)]
pub enum SymbolError {

}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Symbol {{ symbol: &\"{}\", description: &\"{}\", hidden_description: &\"{}\" }}", self.symbol.escape_debug(), self.description.escape_debug(), self.hidden_description.escape_debug())
    }
}

impl fmt::Display for Symbols {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for sym in self.0.iter().take(1) {
            write!(f, "{}", sym)?;
        }
        for sym in self.0.iter().skip(1) {
            write!(f, ",\n {}", sym)?;
        }
        write!(f, "]")
    }
}

pub fn from_string(buffer: &str) -> Result<Vec<Symbol>, SymbolError> {
    buffer.lines().map(|line| parse_symbol(&line)).collect()
}

fn parse_symbol(line: &str) -> Result<Symbol, SymbolError> {
    let delim_pos = line.find("| ").expect(&format!("Expected '| ' delimiter in line {}", line));
    let symbol = &line[1..delim_pos];
    let description = &line.get(delim_pos + 2..).expect(&format!("Expected text after '| ' delimiter in line {}", line));
    let main_description;
    let hidden_description;
    if let Some(delim_pos) = description.find("# ") {
        main_description = description[..delim_pos].trim();
        hidden_description = description.get(delim_pos + 2..).expect(&format!("Expected text after '# ' delimiter in line {}", line));
    } else {
        main_description = description.trim();
        hidden_description = "";
    }
    Ok(Symbol {
        symbol: symbol.to_string(),
        description: main_description.to_string(),
        hidden_description: hidden_description.to_string()
    })
}