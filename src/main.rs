use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::io::{copy, BufReader};
use std::process;
use std::fs::File;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: `source` `target`");
        process::exit(1);
    }

    let mut input = BufReader::new(File::open(&args[1]).unwrap());
    let output = File::create(&args[2]).unwrap();

    let mut encoder = GzEncoder::new(output, Compression::default());

    let start = Instant::now();
    
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();

    println!("Source len: {:?}", input.get_ref().metadata().unwrap().len());
    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!("Elapsed time: {:?}", start.elapsed());
}