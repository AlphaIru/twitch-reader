//!
//! AlphaIru
//! Twitch Reader
//!
//! word_process.rs
//!
//! This is the module for word processing related functions.
//!
//!

use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::iter::Inspect;

use regex::Regex;
use std::io::BufRead;
use trie_rs::{Trie, TrieBuilder};


pub fn load_files() -> (HashMap<String, String>, Trie<u8>) {
    let mut dict_map = HashMap::new();
    let mut dict_trie_builder = TrieBuilder::new();


    if let Err(e) = std::fs::create_dir("./dict") {
        if e.kind() != std::io::ErrorKind::AlreadyExists {
            panic!("Failed to create directory: {}", e);
        }
    }

    for file_entry in std::fs::read_dir("./dict").unwrap() {
        let file = File::open(file_entry.expect("failed to read file").path()).expect("failed to open file");
        let bufreader = std::io::BufReader::new(file);

        for line in bufreader.lines().map_while(Result::ok) {
            let (left, right) = line.split_once(',').expect("Im expecting a csv file with just two elements in a row");
            
            let left = left.trim().to_string();
            let right = right.trim().to_string();
           
            dict_trie_builder.push(&left);

            dict_map.insert(left, right);
        }
    }

    let dict_trie = dict_trie_builder.build();
    (dict_map, dict_trie)
}


pub fn replace_words(input: String, map: &HashMap<String, String>) -> String {
    let re = Regex::new(r"[A-Za-z]+").unwrap();

    let result = re.replace_all(&input, |caps: &regex::Captures| {
        let word = caps[0].to_lowercase();

        match map.get(&word) {
            Some(kana) => kana.clone(),
            None => {
                if word.len() >= 3 {
                    append_unknown_word(&word);
                }
                word.to_string()
            }
        }
    });
    
    result.split_whitespace().collect::<Vec<_>>().join("")
}

/*
pub fn replace_words(input: String, map: &HashMap<String, String>) -> String {
    let parts = input.split_whitespace()
        .map(|v| v.trim().to_lowercase())
        .map(|v| {
            if v.is_ascii() {
                match map.get(&v) {
                    Some(kana) => kana.clone(),
                    None => {
                        if v.len() >= 3 {
                            append_unknown_word(&v);
                        }
                        v.to_string()
                    }
                // map.get(&v).cloned().unwrap_or(v.to_string())
                }
            } else {
                v.to_string()
            }
        })
    .collect::<Vec<_>>();

    parts.join(" ")
}
*/


pub fn append_unknown_word(word: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("./unknown.csv")
        .expect("Failed to open unknown.csv");

    if let Err(e) = writeln!(file, "{},", word) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

