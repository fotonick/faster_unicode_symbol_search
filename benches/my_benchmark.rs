use criterion::{BenchmarkId, black_box, criterion_group, criterion_main, Criterion};

use std::process::Command;
use std::fs::File;
use std::io::{Read, BufReader};

use faster_unicode_symbol_search::symbols::{Symbols, from_string};

pub fn bench_symbols(c: &mut Criterion) {
	let mut group = c.benchmark_group("symbols");
	group.sample_size(20);

    let mut buffer = String::new();
    let infile = File::open("src/symbols.txt").expect("Could not open file 'symbols.txt'");
	let mut infile = BufReader::new(infile);
    infile.read_to_string(&mut buffer).expect("Couldn't read file into memory");

    group.bench_with_input(BenchmarkId::new("Symbols::from_string", "src/symbols.txt"), &buffer, |b, s| b.iter(|| from_string(black_box(s))));

    group.finish();
}

pub fn bench_search(c: &mut Criterion) {
	let mut group = c.benchmark_group("search");
	group.sample_size(20);

    let infile = File::open("src/symbols.txt").expect("Could not open file 'symbols.txt'");
	let mut infile = BufReader::new(infile);
    let mut buffer = String::new();
    infile.read_to_string(&mut buffer).expect("Couldn't read file into memory");
    let symbols_vec = from_string(&buffer).expect("couldn't parse symbols");
    let symbols = Symbols(symbols_vec);

    for input in ["rarr", "µ", "capital delta"].iter() {
	    group.bench_with_input(BenchmarkId::new("Symbols::search_symbols", input), input, |b, s| b.iter(|| symbols.search_symbols(black_box(s))));
    }

    group.finish();
}

pub fn bench_program(c: &mut Criterion) {
	let mut group = c.benchmark_group("program");
	group.sample_size(20);

	Command::new("cargo").arg("build").arg("--release").spawn().unwrap().wait().unwrap();
	let mut command = Command::new("target/release/fuss");

    for input in ["rarr", "µ", "capital delta"].iter() {
	    group.bench_with_input(BenchmarkId::new("target/release/fuss", input), input, |b, input| b.iter(|| command.arg(black_box(input)).output().unwrap()));
    }

    group.finish();
}

pub fn bench_orig_uss(c: &mut Criterion) {
	let mut group = c.benchmark_group("python");
	group.sample_size(10);

	let mut command = Command::new("python3");

    for input in ["rarr", "sigma", "µ"].iter() {
	    group.bench_with_input(BenchmarkId::new("unicode_symbols_search.py", input), input, |b, input| b.iter(|| command.arg("../unicode-symbols-search/unicode_symbols_search/unicode_symbols_search.py").arg(black_box(input)).output().unwrap()));
    }

    group.finish();
}

criterion_group!(benches, bench_search, bench_symbols, bench_program, bench_orig_uss);
criterion_main!(benches);
