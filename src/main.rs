// Copyright 2018 Nicholas Young (and contributors).
// All rights reserved.
//
// Released under a 3-Clause BSD License.
// You should have received a copy with this software.
// Otherwise, visit https://opensource.org to acquire
// a copy.

extern crate matter;
extern crate structopt;

use std::{env::current_dir, fs::read_to_string};
use structopt::StructOpt;

/// Command line interface for the Matter frontmatter
/// parser and extractor.
#[derive(Debug, StructOpt)]
#[structopt(name = "matter")]
struct CLIOptions {
    #[structopt(help = "Input file")]
    input: String,
}

fn main() -> std::io::Result<()> {
    let opts = CLIOptions::from_args();

    let mut path = current_dir()?;
    path.push(opts.input);

    let contents = read_to_string(path)?;
    match matter::extract(&contents) {
        Some((matter, content)) => {
            println!("{:?} {:?}", matter, content);
        }
        _ => {}
    }

    Ok(())
}
