extern crate rusty_compiler;

use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use rusty_compiler::lexer::tokenize;
use rusty_compiler::lexer::Token;
//use lexer::*;
//use std::io;
//use std::str::StrSlice;

fn main()
{
    println!("Hello, World!");

    let filename: String = "my.lang".into();

    // Deserialize using `json::decode`
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(..)  => panic!("Nej!!!"),
    };

    let reader = BufReader::new(&file);

    for line in reader.lines().filter_map(|result| result.ok()) {
        //for c in line.chars() {
        //    print!("{}", c);
        //}
        let tokens = tokenize(&line);
        println!("Tokens: {:?}", tokens);
    }
}
