mod corrector;

#[macro_use]
extern crate generator;
extern crate regex;


use generator::{Generator, Gn};
use std::io::{BufReader, BufRead};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use structopt::StructOpt;
use quicli::prelude::*;
use crate::corrector::SimpleCorrector;


#[derive(Debug, StructOpt)]
struct Cli {
    /// Input file to read
    correct: String,

}

fn main() -> CliResult {
    let args = Cli::from_args();
    let s = SimpleCorrector::new("big.txt");
    let corrected = s.correct(&args.correct);
    match corrected {
        None => { println!("Sorry, no matches found!"); }
        Some(x) => { println!("Did you mean {}?", x); }
    }

    Ok(())
}

