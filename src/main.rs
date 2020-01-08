mod corrector;

#[macro_use]
extern crate generator;

use generator::{Generator, Gn};
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use std::fs::File;


type Word = AsRef<str>;


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

fn create() -> String{
    return "fpwe".to_string();
}


fn main() {
    let a: &str = "str";
    let b: String = String::from("String");
    let c: &String = &b;

    print(a);
    print(c);
    print(b);


    for i in get_thing() {
        println!("{}", i);
    }

    let s = Screen { componenets: vec![Box::new(Foo {}), Box::new(Bar {})] };
    s.run();
    let string = String::from("foepfe");

    let reader = BufReader::new(File::open("foo.xt").expect("Cannot open file.txt"));

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

