use criterion::{BenchmarkId, criterion_group, criterion_main, Criterion};
use std::fs::File;
use std::io::{Read, BufReader};

use faster_unicode_symbol_search::symbols::Symbols;

pub fn bench_symbols(c: &mut Criterion) {
	let mut group = c.benchmark_group("symbols");
	group.sample_size(20);

    let mut buffer = String::new();
    let infile = File::open("src/symbols.txt").expect("Could not open file 'symbols.txt'");
	let mut infile = BufReader::new(infile);
    infile.read_to_string(&mut buffer).expect("Couldn't read file into memory");

    group.bench_with_input(BenchmarkId::new("Symbols::from_string ", "src/symbols.txt"), &buffer, |b, s| b.iter(|| Symbols::from_string(s)));

    group.finish();
}

pub fn bench_search(c: &mut Criterion) {
	let mut group = c.benchmark_group("search");
	group.sample_size(20);

    let infile = File::open("src/symbols.txt").expect("Could not open file 'symbols.txt'");
	let infile = BufReader::new(infile);
    let symbols = Symbols::from_file(infile).expect("couldn't parse symbols");

    for input in ["rarr", "sigma"].iter() {
	    group.bench_with_input(BenchmarkId::new("Symbols::search_symbols ", input), input, |b, s| b.iter(|| symbols.search_symbols(s)));
    }

    group.finish();
}

criterion_group!(benches, bench_search, bench_symbols);
criterion_main!(benches);
