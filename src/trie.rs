use std::option::Option::{Some, None};
use rand::Rng;

pub struct Trie<const ALPHABET_SIZE: usize> {
    children: [Option<Box<Trie<ALPHABET_SIZE>>>; ALPHABET_SIZE],
    end_of_word: bool,
}

impl<const ALPHABET_SIZE: usize> Trie<ALPHABET_SIZE> {
    pub fn new() -> Self {
        struct Helper<const ALPHABET_SIZE: usize>;
        impl<const ALPHABET_SIZE: usize> Helper<ALPHABET_SIZE> {
            const NONE_CHILDREN: Option<Box<Trie<ALPHABET_SIZE>>> = None;
        }
        Self {
            children: [Helper::<ALPHABET_SIZE>::NONE_CHILDREN; ALPHABET_SIZE],
            end_of_word: false,
        }
    }

    pub fn insert(&mut self, word: &[usize]) {
        let mut node = self;
        for c in word {
            if let None = node.children[*c] {
                node.children[*c] = Some(Box::new(Trie::new()));
            }
            node = node.children[*c].as_mut().unwrap();
        }
        node.end_of_word = true;
    }

    pub fn find(&self, word: &[Option<usize>]) -> Vec<Vec<usize>> {
        let mut result = self.find_reverse(word);
        for mut word in result {
            word.reverse();
        }
        result
    }

    fn find_reverse(&self, word: &[Option<usize>]) -> Vec<Vec<usize>> {
        if word.is_empty() {
            let mut result = Vec::new();
            if self.end_of_word {
                result.push(Vec::new());
            }
            return result;
        }
        if let Some(c) = word[0] {
            if let Some(child) = &self.children[c] {
                let mut result = child.as_ref().find_reverse(&word[1..]);
                for word in result.iter_mut() {
                    word.push(c);
                }
                result
            } else {
                Vec::new()
            }
        } else {
            let mut result = Vec::new();
            for (c, child) in self.children.iter().enumerate() {
                if let Some(child) = child {
                    let mut child_result = child.as_ref().find_reverse(&word[1..]);
                    for word in child_result.iter_mut() {
                        word.push(c);
                    }
                    result.append(&mut child_result);
                }
            }
            result
        }
    }

    pub fn find_random(&self, word: &[Option<usize>], rng: &mut impl Rng) -> Vec<Vec<usize>> {
        let mut result = self.find_reverse_random(word, rng);
        for mut word in result {
            word.reverse();
        }
        result
    }

    fn find_reverse_random(&self, word: &[Option<usize>], rng: &mut impl Rng) -> Vec<Vec<usize>> {
        if word.is_empty() {
            let mut result = Vec::new();
            if self.end_of_word {
                result.push(Vec::new());
            }
            return result;
        }
        if let Some(c) = word[0] {
            if let Some(child) = &self.children[c] {
                let mut result = child.as_ref().find_reverse(&word[1..]);
                for word in result.iter_mut() {
                    word.push(c);
                }
                result
            } else {
                Vec::new()
            }
        } else {
            let mut result = Vec::new();
            let random_offset = rng.gen_range(0..ALPHABET_SIZE);
            for (c, child) in self.children.iter().enumerate().skip(random_offset) {
                if let Some(child) = child {
                    let mut child_result = child.find_reverse_random(&word[1..], rng);
                    for word in child_result.iter_mut() {
                        word.push(c);
                    }
                    result.append(&mut child_result);
                }
            }
            for (c, child) in self.children.iter().enumerate().take(random_offset) {
                if let Some(child) = child {
                    let mut child_result = child.find_reverse_random(&word[1..], rng);
                    for word in child_result.iter_mut() {
                        word.push(c);
                    }
                    result.append(&mut child_result);
                }
            }
            result
        }
    }

    pub fn find_one_random(&self, word: &[Option<usize>], rng: &mut impl Rng) -> Option<Vec<usize>> {
        let mut result = self.find_one_reverse_random(word, rng);
        if let Some(word) = result.as_mut() {
            word.reverse();
        }
        result
    }

    fn find_one_reverse_random(&self, word: &[Option<usize>], rng: &mut impl Rng) -> Option<Vec<usize>> {
        if word.is_empty() {
            if self.end_of_word {
                return Some(Vec::new());
            } else {
                return None;
            }
        }
        if let Some(c) = word[0] {
            if let Some(child) = &self.children[c] {
                let mut result = child.as_ref().find_one_reverse_random(&word[1..], rng);
                if let Some(word) = result.as_mut() {
                    word.push(c);
                }
                result
            } else {
                None
            }
        } else {
            let random_offset = rng.gen_range(0..ALPHABET_SIZE);
            for (c, child) in self.children.iter().enumerate().skip(random_offset) {
                if let Some(child) = child {
                    let mut child_result = child.find_one_reverse_random(&word[1..], rng);
                    if let Some(word) = child_result.as_mut() {
                        word.push(c);
                        return Some(*word);
                    }
                }
            }
            for (c, child) in self.children.iter().enumerate().take(random_offset) {
                if let Some(child) = child {
                    let mut child_result = child.find_one_reverse_random(&word[1..], rng);
                    if let Some(word) = child_result.as_mut() {
                        word.push(c);
                        return Some(*word);
                    }
                }
            }
            None
        }
    }

    pub fn count_matches(&self, word: &[Option<usize>]) -> usize {
        if word.is_empty() {
            return if self.end_of_word { 1 } else { 0 };
        }
        if let Some(c) = word[0] {
            if let Some(child) = &self.children[c] {
                child.as_ref().count_matches(&word[1..])
            } else {
                0
            }
        } else {
            let mut result: usize = 0;
            for child in self.children.iter() {
                if let Some(child) = child {
                    result += child.as_ref().count_matches(&word[1..]);
                }
            }
            result
        }
    }
}