// Copyright 2018 Nicholas Young (and contributors).
// All rights reserved.
//
// Released under a 3-Clause BSD License. You should have received a
// copy with this software. Otherwise, visit https://opensource.org
// to acquire a copy.

use std::{env::current_dir, fs::read_to_string};
use structopt::StructOpt;

fn main() -> std::io::Result<()> {
    let opts = CLI::from_args();
    let path = current_dir()?.join(opts.input);
    let data = read_to_string(path)?;

    match matter::extract(&data) {
        Err(e) => {
            eprintln!("{}", e);
        },
        Ok(Some((m, t))) => {
            println!("{} {}", m, t);
        },
        _ => {},
    };

    Ok(())
}

/// Command line interface for the Matter frontmatter parser and
/// extractor.
#[derive(Debug, StructOpt)]
#[structopt(name = "matter")]
struct CLI {
    #[structopt(help = "Input file")]
    input: String,
}
