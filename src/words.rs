use std::collections::BTreeSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::Rng;
use crate::config::Configuration;

pub fn parse(config:&Configuration)->Result<Vec<String>, ()> {
    let file = File::open(config.repository.clone()).unwrap();
    let reader = BufReader::new(file);
    let mut lines:Vec<String> = vec![];
    for line_result in reader.lines() {
        let line:String;
        match line_result {
            Ok(l) => line = l,
            Err(_) => {
                println!("words.txt parse error");
                continue;
            }
        }
        lines.push(line);
    }
    Ok(lines)
}

pub fn random_indexes(config:&Configuration, length:usize)->BTreeSet<usize> {
    let mut rng = rand::thread_rng();
    let mut disabled_lines:BTreeSet<usize> = BTreeSet::new();
    let disabled_lines_count = rng.gen_range(config.min_disabled_words..=config.max_disabled_words);
    while disabled_lines.len() < disabled_lines_count {
        disabled_lines.insert(rng.gen_range(0usize..length));
    }
    return disabled_lines;
}