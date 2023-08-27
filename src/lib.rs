const WORDS: &str = include_str!("words.txt");

use std::collections::HashSet;

use rand::seq::SliceRandom;

pub struct Dictionary {
    words: HashSet<&'static str>,
}

impl Dictionary {
    pub fn new() -> Self {
        let words: HashSet<&str> = WORDS.split('\n').collect();
        Self { words }
    }

    pub fn get_random_word(&self) -> String {
        Vec::from_iter(self.words.iter())
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_string()
    }
}

impl Default for Dictionary {
    fn default() -> Self {
        Dictionary::new()
    }
}

pub struct Game {
    answer: String,
    dictionary: Dictionary,
}

impl Game {
    pub fn get_answer(&self) -> String {
        self.answer.to_string()
    }
}

impl Default for Game {
    fn default() -> Self {
        let dict = Dictionary::default();
        Game {
            answer: dict.get_random_word(),
            dictionary: dict,
        }
    }
}
