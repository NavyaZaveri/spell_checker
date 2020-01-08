mod corrector;

#[macro_use]
extern crate generator;

use generator::{Generator, Gn};
use std::io::{BufReader, BufRead};
use std::collections::{HashMap, HashSet};
use std::fs::File;


fn get_thing() -> Generator<'static, (), i32> {
    let g = Gn::new_scoped(|mut s| {
        let (mut a, mut b) = (0, 1);
        while b < 200 {
            std::mem::swap(&mut a, &mut b);
            b = a + b;
            s.yield_(b);
        }
        done!();
    });
    return g;
}

fn print<S: AsRef<str>>(stringlike: S) {
    // call as_ref() to get a &str
    let str_ref = stringlike.as_ref();

    println!("got: {:?}", str_ref)
}

fn create() -> String {
    return "fpwe".to_string();
}


fn main() {
    extract_words("count_1w.txt");
}

trait Draw {
    fn draw(&self);
}

struct Foo;

struct Bar;


impl Draw for Foo {
    fn draw(&self) {
        dbg!("darawing doo");
    }
}


impl Draw for Bar {
    fn draw(&self) {
        dbg!("drawing bar");
    }
}

struct Screen {
    componenets: Vec<Box<dyn Draw>>
}


impl Screen {
    fn run(&self) {
        for comp in self.componenets.iter() {
            comp.draw();
        }
    }
}


fn dfs(pos: (i32, i32), seen: &mut HashSet<(i32, i32)>, skyMap: &Vec<Vec<char>>) {
    seen.insert(pos);
    let new_positions = vec![(pos.0, pos.1 - 1), (pos.0, pos.1 + 1), (pos.0 - 1, pos.1), (pos.0 + 1, pos.1)];
    for (u, v) in new_positions {
        if 0 <= u && 0 < skyMap.len()
            && 0 <= v && v < skyMap[0].len() as i32
            && skyMap[u as usize][v as usize] == '1'
            && seen.contains(&(u, v)) {
            dfs((u, v), seen, skyMap);
        }
    }
}


fn extract_words(filename: &'static str) -> HashMap<String, usize> {
    let reader = BufReader::new(File::open(filename).expect("Cannot open file.txt"));
    let mut counter = HashMap::new();
    for line in reader.lines() {
        let temp = line.unwrap();
        let data = temp.split_whitespace().collect::<Vec<&str>>();
        let word = data[0];
        let count = data[1];
        *counter.entry(word.to_string()).or_default() += 1;
    }
    counter
}


fn countClouds(skyMap: Vec<Vec<char>>) -> i32 {
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    let mut count = 0;
    for i in 0..skyMap.len() {
        for j in 0..skyMap[i].len() {
            if skyMap[i][j] == '1' {
                count += 1;
                dfs((i as i32, j as i32), &mut seen, &skyMap);
            }
        }
    }
    return count;
}


