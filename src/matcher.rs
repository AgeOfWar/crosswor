use std::io::{BufRead, Result};

use rand::Rng;

use crate::trie::Trie;

pub struct Matcher {
    trie: Trie<26>,
    word_count_by_length: Vec<usize>,
}

impl Matcher {
    fn new() -> Self {
        Self {
            trie: Trie::new(),
            word_count_by_length: Vec::new(),
        }
    }

    pub fn from_file(path: &str) -> Result<Self> {
        let mut matcher = Self::new();
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        for line in reader.lines() {
            let word = line?.to_uppercase();
            if is_uppercase_str(&word) {
                matcher.insert(&word);
            }
        }
        Ok(matcher)
    }

    pub fn insert(&mut self, word: &str) {
        if word.chars().any(|c| !c.is_ascii_uppercase() && !c.is_ascii_lowercase()) {
            panic!("Invalid word: {}", word);
        }
        let word = word.chars().map(char_to_int).collect::<Vec<_>>();
        self.trie.insert(&word);
        if word.len() >= self.word_count_by_length.len() {
            self.word_count_by_length.resize(word.len() + 1, 0);
        }
        self.word_count_by_length[word.len()] += 1;
    }

    pub fn word_count_by_length(&self, len: usize) -> usize {
        if len >= self.word_count_by_length.len() {
            0
        } else {
            self.word_count_by_length[len]
        }
    }

    pub fn find(&self, word: &[Option<char>]) -> Vec<String> {
        let trie_word = word.iter().map(|c| c.map(char_to_int)).collect::<Vec<_>>();
        let result = self.trie.find(&trie_word);
        result.iter().map(|w| w.iter().map(|c| int_to_char(*c)).collect::<String>()).collect()
    }

    pub fn find_vec_random(&self, word: &[Option<char>], rng: &mut impl Rng) -> Vec<String> {
        let trie_word = word.iter().map(|c| c.map(char_to_int)).collect::<Vec<_>>();
        let result = self.trie.find_random(&trie_word, rng);
        result.iter().map(|w| w.iter().map(|c| int_to_char(*c)).collect::<String>()).collect()
    }

    pub fn count_matches(&self, word: &[Option<char>]) -> usize {
        let trie_word = word.iter().map(|c| c.map(char_to_int)).collect::<Vec<_>>();
        self.trie.count_matches(&trie_word)
    }
}

fn char_to_int(c: char) -> usize {
    if c >= 'A' && c <= 'Z' {
        c as usize - 'A' as usize
    } else if c >= 'a' && c <= 'z' {
        c as usize - 'a' as usize
    } else {
        panic!("Invalid character: {}", c);
    }
}

fn int_to_char(i: usize) -> char {
    if i < 26 {
        (i as u8 + 'A' as u8) as char
    } else {
        panic!("Invalid integer: {}", i);
    }
}

fn is_uppercase_str(input: &str) -> bool {
    for c in input.chars() {
        if !c.is_ascii_uppercase() {
            return false;
        }
    }
    true
}