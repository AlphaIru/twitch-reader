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

pub fn url_shouryaku(text: String) -> String {
    let email_re = Regex::new(r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b").unwrap();
    let url_re = Regex::new(r"https?:[//\w/:%#\$&\?\(\)~\.=\+\-]+").unwrap();

    let text = email_re.replace_all(&text, "メール省略");
    url_re.replace_all(&text, "URL省略").to_string()
}

pub fn to_zenkaku_punctuation(input: String) -> String {
    input.chars().map( |c| {
        match c {
            '!' => '！',
            '?' => '？',
            ',' => '、',
            '.' => '。',
            ':' => '：',
            ';' => '；',
            '(' => '（',
            ')' => '）',
            '+' => '＋',
            '-' => '－',
            '=' => '＝',
            _ => c,
        }

    }).collect()
}

pub fn to_hankaku_alphabets(input: String) -> String {
    input.chars().map( |c| {
        let code = c as u32;
        if (0xFF01..=0xFF5E).contains(&code) {
            std::char::from_u32(code - 0xFEE0).unwrap_or(c)
        } else {
            c
        }
    }).collect()
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


pub fn hear_aid(text: String) -> String {
    text.replace('~', "ー")
        .replace('～', "ー")
        .replace('-', "ー")
        .replace('|', " ")
        .replace('…', "、")
}


pub fn remove_garbage(text: String) -> String {
    // 読み上げに不要な記号（| や連続する記号など）を削除
    let re = Regex::new(r"[|｜_＿\[\]【】]").unwrap();
    re.replace_all(&text, "").to_string()
}


pub fn replace_with_trie(
    word: &str,
    map: &HashMap<String, String>,
    trie: &Trie<u8>,
) -> String {
    let mut result = String::new();
    let mut cursor = 0;
    let bytes = word.as_bytes();

    if word.trim() == "" {
        return word.to_string();
    }

    while cursor < bytes.len() {
        let remaining = &word[cursor..];
        let mut longest_match = None;

        for prefix in trie.common_prefix_search(remaining) {
            if let Ok(s) = String::from_utf8(prefix) {
                longest_match = Some(s);
            }
        }

        if let Some(matched) = longest_match {
            if let Some(kana) = map.get(&matched) {
                result.push_str(kana);

            }
            else {
                result.push_str(&matched);
            }
            cursor += matched.len();
        }
        else {
            let next_char = word[cursor..].chars().next().unwrap();
            result.push(next_char);
            cursor += next_char.len_utf8();
        }
    }

    result
}

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

pub fn clean_text(
    recv_msg: String,
    dict_map: &HashMap<String, String>,
    dict_trie: &Trie<u8>
) -> String {
    let mut recv_msg = url_shouryaku(recv_msg);
    recv_msg = hear_aid(recv_msg);
    recv_msg = to_zenkaku_punctuation(recv_msg);
    recv_msg = to_hankaku_alphabets(recv_msg);
    recv_msg = replace_words(recv_msg, &dict_map);
    recv_msg = replace_with_trie(&recv_msg, &dict_map, &dict_trie); 
    recv_msg = remove_garbage(recv_msg);

    recv_msg.trim().to_string()
}


pub fn limit_length(raw_msg: String, max_length: usize) -> String {
    if raw_msg.len() <= max_length {
        return raw_msg;
    }
    let fmt_msg = format!("{}、以下略。", raw_msg.chars().take(max_length).collect::<String>());
    fmt_msg
}
