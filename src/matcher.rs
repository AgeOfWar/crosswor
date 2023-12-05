use std::{io::{BufRead, Result}, collections::HashMap};

use rand::Rng;

use crate::trie::Trie;

pub struct Matcher {
    trie: Trie<26>,
    find_cache: HashMap<Vec<Option<char>>, Vec<String>>,
}

impl Matcher {
    fn new() -> Self {
        Self {
            trie: Trie::new(),
            find_cache: HashMap::new(),
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
        let word = word.chars().map(char_to_int).collect::<Vec<_>>();
        self.trie.insert(&word);
    }

    pub fn find(&self, word: &[Option<char>]) -> Vec<String> {
        let trie_word = word.iter().map(|c| c.map(char_to_int)).collect::<Vec<_>>();
        let result = self.trie.find_reverse(&trie_word);
        result.iter().map(|w| w.iter().rev().map(|c| int_to_char(*c)).collect::<String>()).collect::<Vec<_>>()
    }

    pub fn find_vec(&self, word: &Vec<Option<char>>) -> Vec<String> {
        let trie_word = word.iter().map(|c| c.map(char_to_int)).collect::<Vec<_>>();
        let result = self.trie.find_reverse(&trie_word);
        result.iter().map(|w| w.iter().rev().map(|c| int_to_char(*c)).collect::<String>()).collect::<Vec<_>>()
    }

    pub fn find_vec_cached(&mut self, word: &Vec<Option<char>>) -> Vec<String> {
        if let Some(result) = self.find_cache.get(word) {
            return result.clone();
        }
        let result = self.find_vec(word);
        self.find_cache.insert(word.clone(), result.clone());
        result
    }

    pub fn find_vec_random<R: Rng + ?Sized>(&self, word: &Vec<Option<char>>, rng: &mut R) -> Vec<String> {
        let trie_word = word.iter().map(|c| c.map(char_to_int)).collect::<Vec<_>>();
        let result = self.trie.find_reverse_random(&trie_word, rng);
        result.iter().map(|w| w.iter().rev().map(|c| int_to_char(*c)).collect::<String>()).collect::<Vec<_>>()
    }

    pub fn find_vec_random_cached<R: Rng + ?Sized>(&mut self, word: &Vec<Option<char>>, rng: &mut R) -> Vec<String> {
        if let Some(result) = self.find_cache.get(word) {
            return result.clone();
        }
        let result = self.find_vec_random(word, rng);
        self.find_cache.insert(word.clone(), result.clone());
        result
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