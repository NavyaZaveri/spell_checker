use generator::{Gn, Generator, Scope, done};
use std::collections::{HashMap, HashSet};
use std::ops::Add;

use std::io::{BufRead, BufReader};
use std::fs::File;
use regex::Regex;
use std::fs;


#[derive(Debug)]
struct EditWord {
    word: String,
    editDistance: usize,
}


impl EditWord {
    fn new(w: String, editDistance: usize) -> EditWord {
        return EditWord { word: w, editDistance };
    }
}


static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e',
    'f', 'g', 'h', 'i', 'j',
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't',
    'u', 'v', 'w', 'x', 'y',
    'z',
];


type Stream<'s, T> = Generator<'s, (), T>;


#[derive(Debug)]
pub struct WordDataSet {
    counter: HashMap<String, usize>
}


fn extract_words_from_file(filename: &str) -> HashMap<String, usize> {
    let re = Regex::new("[a-z]+").unwrap();
    let filepath = fs::read_to_string(filename).unwrap();
    let words: Vec<&str> = re.find_iter(&filepath).map(|mat| mat.as_str()).collect();

    let mut counter = HashMap::new();
    for w in words {
        *counter.entry(w.to_string()).or_default() += 1;
    }
    return counter;
}

impl<'a> From<Vec<&'a str>> for WordDataSet {
    fn from(vec: Vec<&'a str>) -> Self {
        let mut counter = HashMap::new();
        for w in vec {
            *counter.entry(w.to_string()).or_default() += 1;
        }
        return WordDataSet { counter };
    }
}


impl<'a> From<Vec<&'a str>> for SimpleCorrector {
    fn from(vec: Vec<&'a str>) -> SimpleCorrector {
        return SimpleCorrector { data_set: WordDataSet::from(vec) };
    }
}

impl WordDataSet {
    pub fn prob(&self, word: &str) -> f64 {
        if !self.counter.contains_key(word) {
            return 0.0;
        }
        return *self.counter.get(word).unwrap() as f64 / self.counter.values().sum::<usize>() as f64;
    }

    fn exists(&self, word: &str) -> bool {
        return self.counter.contains_key(word);
    }

    pub fn new(filename: &str) -> WordDataSet {
        return WordDataSet {
            counter: extract_words_from_file(filename)
        };
    }
}


fn splits(w: &str) -> Vec<(&str, &str)> {
    (0..=w.len()).map(|i| w.split_at(i)).collect()
}


pub struct SimpleCorrector {
    data_set: WordDataSet
}


impl SimpleCorrector {
    fn new(filename: &str) -> SimpleCorrector {
        return SimpleCorrector {
            data_set: WordDataSet::new(filename)
        };
    }
    pub fn correct(&self, word: &str) -> Option<String> {
        if self.data_set.exists(word) {
            return Some(word.to_string());
        }

        edits(1, word)
            .filter(|e| self.data_set.exists(&e.word))
            .map(|e| ((1 / e.editDistance) as f64 * self.data_set.prob(&e.word), e.word))
            .max_by(|(p1, w1), (p2, w2)| p1.partial_cmp(p2).expect("Tried to compare NAN"))
            .map(|(p, w)| w)
    }
}


fn edit1(w: &str) -> Stream<String> {
    let pairs = splits(w);
    let g = Gn::new_scoped(move |mut s| {
        for (a, b) in pairs.iter() {
            let delete = format!("{}{}", a, b.get(1..).unwrap_or_default());
            s.yield_(delete);

            for new_char in ASCII_LOWER.iter() {
                let replace = format!("{}{}{}", a, new_char, b.get(1..).unwrap_or_default());
                s.yield_(replace);

                let insert = format!("{}{}{}", a, new_char, b);
                s.yield_(insert);
            }
        }

        done!();
    });
    return g;
}


fn edits(n: usize, word: &str) -> Stream<EditWord> {
    let g = Gn::new_scoped(move |mut s| {
        let mut v = vec![word.to_string()];
        let mut seen = HashSet::new();
        seen.insert(word.to_string());
        for i in 0..n {
            let mut next_list = vec![];
            for word in v {
                for w in edit1(&word) {
                    if !seen.contains(&w) {
                        next_list.push(w.to_string());
                        seen.insert(w.to_string());
                        let editWord = EditWord::new(w.to_string(), i + 1);
                        s.yield_(editWord);
                    }
                }
            }
            v = next_list;
        }
        done!();
    });
    return g;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_prob() {
        let data_set = WordDataSet::from(vec!["A", "B"]);
        assert_eq!(data_set.prob("B"), 0.5)
    }


    #[test]
    fn test_word_split() {
        let word = "abc";
        let word_splits = splits(word);
        assert_eq!(word_splits, vec![("", "abc"),
                                     ("a", "bc"),
                                     ("ab", "c"),
                                     ("abc", "")])
    }

    #[test]
    fn test_corrector_on_valid_word() {
        let word = "ab";
        let word_list = vec!["ab", "cd"];
        let s = SimpleCorrector::from(word_list);
        let res = s.correct("ab");
        dbg!(res);
    }


    #[test]
    fn test_corrector_on_invalid_word() {
        let test_word = "aa";
        let word_list = vec!["ab", "cd"];
        let s = SimpleCorrector::from(word_list);
        let res = s.correct(test_word);
        assert_eq!(res.unwrap(), "ab");
    }

    #[test]
    fn test_corrector_with_actual_dataset() {
        let test_word = "inconcevable";
        let s = SimpleCorrector::new("big.txt");
        let corrected_word = s.correct(test_word);
        assert_eq!(corrected_word.unwrap(), "inconceivable");
    }
}


