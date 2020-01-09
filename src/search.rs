use crate::symbols::{Symbol, Symbols};
use bstr::Finder;

impl Symbols {
    pub fn search_symbols<'a>(&'a self, query_text: &'a str) -> impl Iterator<Item=String> + 'a {
        let match_fn = create_query(query_text);
        Box::new(self.0.iter().cloned().filter(match_fn).map(|sym| sym.symbol))
    }
}

fn create_query<'a>(query_text: &'a str) -> Box<dyn Fn(&Symbol) -> bool + 'a> {
    let letters: Vec<&str> = query_text.split(" ").filter(|w| w.len() == 1).collect();
    if letters.len() == 1 {
        create_single_letter_query(letters[0], query_text)
    } else {
        create_words_query(query_text)
    }
}

fn create_words_query<'a>(query_text: &'a str) -> Box<dyn Fn(&Symbol) -> bool + 'a> {
    let finders: Box<Vec<Finder>> = Box::new(query_text.split(" ").map(Finder::new).collect());  // Want to reuse these structs across many closure invocations.
    let query_text_copy2 = query_text.to_string();
    let is_symbol_matching = move |symbol: &Symbol| {
        // symbol in query...
        query_text_copy2.find(&symbol.symbol).is_some() ||
        // ... or all query words found somewhere in extended description
        finders.iter().all(|f| f.find(&symbol.description).is_some() ||
                               f.find(&symbol.hidden_description).is_some())
    };
    Box::new(is_symbol_matching)
}

fn create_single_letter_query(letter: &str, query_text: &str) -> Box<dyn Fn(&Symbol) -> bool> {
    let letter = letter.to_owned();
    let query = query_text.to_owned();
    let is_symbol_matching = move |symbol: &Symbol| {
        let words_query = create_words_query(&query);
        let infix = format!(" {} ", letter);
        (symbol.symbol == letter ||
         symbol.description.starts_with(&infix[1..]) ||
         symbol.description.ends_with(&infix[..infix.len()-1]) ||
         symbol.description.contains(&infix)) &&
        words_query(symbol)
    };
    Box::new(is_symbol_matching)
}