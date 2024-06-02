mod config;
mod words;
use std::io::{BufRead, BufReader};
use rand::prelude::*;


fn main() {
    let config = config::load_config();
    let lines:Vec<String>;
    match words::parse(&config) {
        Ok(l) => lines = l,
        Err(_) => {
            println!("Couldn't load file. Good luck!");
            return;
        }
    }
    let disabled_lines = words::random_indexes(&config, lines.len());
    for i in 0usize..lines.len() {
        if(disabled_lines.contains(&i)) {
            continue;
        }
        println!("{}", lines[i]);
    }
    println!("\nProgram finished. You got {} words.\nFound {} words total, excluded {} words.", lines.len() - disabled_lines.len(), lines.len(), disabled_lines.len())
}
