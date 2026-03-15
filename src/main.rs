// dande
// parallel ingestion engine
// stop using pandas for multi-gig files.

use std::{env, process};
use rayon::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { 
        eprintln!("usage: dande_ingest <file.csv>");
        process::exit(1); 
    }

    let mut rdr = csv::Reader::from_path(&args[1]).unwrap_or_else(|_| {
        eprintln!("fatal: could not open file.");
        process::exit(1);
    });

    let records: Vec<_> = rdr.byte_records().filter_map(Result::ok).collect();

    let processed: Vec<_> = records.par_iter()
        .map(|r| {
            // raw byte manipulation. fast and memory safe.
            r.iter().map(|b| b.to_ascii_uppercase()).collect::<Vec<_>>()
        }).collect();

    println!("processed {} records across {} threads.", processed.len(), rayon::current_num_threads());
}
