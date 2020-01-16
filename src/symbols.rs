use std::fmt;

#[derive(Clone, Debug)]
pub struct Symbol<'a> {
    pub symbol: &'a str,
    pub description: &'a str,
    pub hidden_description: &'a str,
}

#[derive(Clone, Debug)]
pub struct Symbols<'a>(pub &'a[Symbol<'a>]);

#[derive(Debug)]
pub enum SymbolError {

}

impl fmt::Display for Symbol<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Symbol {{ symbol: &\"{}\", description: &\"{}\", hidden_description: &\"{}\" }}", self.symbol.escape_debug(), self.description.escape_debug(), self.hidden_description.escape_debug())
    }
}

impl fmt::Display for Symbols<'_> {
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

fn parse_symbol<'a>(line: &'a str) -> Result<Symbol<'a>, SymbolError> {
    let delim_pos = line.find("| ").expect(&format!("Expected '| ' delimiter in line {}", line));
    let symbol = &line[..delim_pos].trim();
    let description = &line.get(delim_pos + 2..).expect(&format!("Expected text after '| ' delimiter in line {}", line));
    let main_description;
    let hidden_description;
    if let Some(delim_pos) = description.find("# ") {
        main_description = &description[..delim_pos];
        hidden_description = description.get(delim_pos + 2..).expect(&format!("Expected text after '# ' delimiter in line {}", line));
    } else {
        main_description = description;
        hidden_description = "";
    }
    Ok(Symbol {
        symbol: symbol,
        description: main_description,
        hidden_description: hidden_description
    })
}