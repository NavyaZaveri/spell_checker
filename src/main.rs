mod corrector;

#[macro_use]
extern crate generator;
extern crate regex;

use crate::corrector::SimpleCorrector;
use quicli::prelude::*;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short, long)]
    typo: String,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    let s = SimpleCorrector::new("big.txt");
    let corrected = s.correct(&args.typo);
    match corrected {
        None => {
            println!("Sorry, no matches found!");
        }
        Some(x) => {
            println!("Did you mean {}?", x);
        }
    }
    Ok(())
}
