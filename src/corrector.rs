use generator::{Gn, Generator, Scope, done};
use std::collections::{HashMap, HashSet};
use std::ops::Add;


macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
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


pub struct WordDataSet {
    counter: HashMap<String, usize>
}

fn splits(w: &str) -> Vec<(&str, &str)> {
    (0..=w.len()).map(|i| w.split_at(i)).collect()
}


impl WordDataSet {
    fn load(filename: &str) -> WordDataSet {
        unimplemented!()
    }


    fn prob(&self, word: &str) -> f64 {
        if !self.counter.contains_key(word) {
            return 0.0;
        }
        return *self.counter.get(word).unwrap() as f64 / self.counter.values().sum::<usize>() as f64;
    }
    fn exists(&self, word: &str) -> bool {
        return self.counter.contains_key(word);
    }
}

impl From<Vec<String>> for WordDataSet {
    fn from(vec: Vec<String>) -> Self {
        let mut counter: HashMap<String, usize> = HashMap::new();
        for w in vec {
            *counter.entry(w).or_default() += 1;
        }
        return WordDataSet { counter };
    }
}


pub struct SimpleCorrector {
    data_set: WordDataSet
}


impl SimpleCorrector {
    pub fn correct(&self, word: &str) -> String {
        if self.data_set.exists(word.as_ref()) {
            return word.to_string();
        }


        let (word, prob) =
            self.edit1(&word)
                .map(|x| (x.to_string(), self.data_set.prob(&x)))
                .max_by(|(w1, p1), (w2, p2)| p1.partial_cmp(p2).expect("Tried to compare NAN"))
                .unwrap();

        return word;
    }


    fn edit1<'a>(&self, w: &'a str) -> Stream<'a, String> {
        let pairs = splits(w);
        let g = Gn::new_scoped(move |mut s| {
            //deletes
            for (a, b) in pairs.iter() {
                let delete = format!("{}{}", a, &b[1..]);
                s.yield_(delete);
            }

            for (a, b) in pairs.iter() {
                for c in ASCII_LOWER.iter() {

                    //replace
                    let replace = format!("{}{}{}", a, c, &b[1..]);
                    s.yield_(replace);

                    //insert
                    let insert = format!("{}{}{}", a, c, b);
                    s.yield_(insert);
                }
            }


            done!();
        });
        return g;
    }


    /*
        fn edits<'a>(&self, n: usize, word: &'a str) -> Stream<'a, &str> {
            let word = word;

            let g = Gn::new_scoped(move |mut s| {
                let mut words_seen = HashSet::new();
                words_seen.insert(word.clone());
                let mut initial = vec![word];

                for i in 0..n {
                    let mut next_word_list = vec![];

                    for w in initial.iter() {
                        for new_word in self.edit1(w) {
                            if !words_seen.contains(&new_word) {
                                s.yield_(new_word.clone());
                                words_seen.insert(new_word.clone());
                                next_word_list.push(new_word.clone());
                            }
                        }
                    }
                    initial = next_word_list;
                }
                done!();
            });
            return g;
        }*/
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_prob() {
        let data_set = WordDataSet { counter: hashmap!["A".to_string() => 2, "B".to_string() => 2] };
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
        let word_list = vec!["ab".to_string(), "cd".to_string()];
        let word_dataset = WordDataSet::from(word_list);
        let s = SimpleCorrector { data_set: word_dataset };
        let res = s.correct("ab");
        dbg!(res);
    }


    #[test]
    fn test_corrector_on_invalid_word() {
        let word = "aa";
        let word_list = vec!["ab".to_string(), "cd".to_string()];
        let word_dataset = WordDataSet::from(word_list);
        let s = SimpleCorrector { data_set: word_dataset };
        let res = s.correct("ab");
        dbg!(res);
    }
}


