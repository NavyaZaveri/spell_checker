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

pub struct WordDataSet<'a> {
    counter: HashMap<&'a str, usize>
}

impl<'a> From<Vec<&'a str>> for WordDataSet<'a> {
    fn from(vec: Vec<&'a str>) -> Self {
        let mut counter: HashMap<&'a str, usize> = HashMap::new();
        for w in vec {
            *counter.entry(w).or_default() += 1;
        }
        return WordDataSet { counter };
    }
}

impl<'a> WordDataSet<'a> {
    pub fn prob(&'a self, word: &'a str) -> f64 {
        if !self.counter.contains_key(word) {
            return 0.0;
        }
        return *self.counter.get(word).unwrap() as f64 / self.counter.values().sum::<usize>() as f64;
    }

    fn exists(&'a self, word: &'a str) -> bool {
        return self.counter.contains_key(word);
    }
}


fn splits(w: &str) -> Vec<(&str, &str)> {
    (0..=w.len()).map(|i| w.split_at(i)).collect()
}


pub struct SimpleCorrector<'a> {
    data_set: WordDataSet<'a>
}


impl<'a> SimpleCorrector<'a> {
    pub fn correct(&self, word: &str) -> Option<String> {
        if self.data_set.exists(word) {
            return Some(word.to_string());
        }

        edit1(word)
            .filter(|w| self.data_set.exists(w))
            .map(|x| (x.to_string(), self.data_set.prob(&x)))
            .max_by(|(w1, p1), (w2, p2)| p1.partial_cmp(p2).expect("Tried to compare NAN"))
            .map(|(w, p)| w)
    }
}


fn edit1<'a>(w: &'a str) -> Stream<String> {
    let pairs = splits(w);
    let g = Gn::new_scoped(move |mut s| {
        //deletes
        for (a, b) in pairs.iter() {
            let delete = format!("{}{}", a, b.get(1..).unwrap_or_default());
            s.yield_(delete);
        }

        for (a, b) in pairs.iter() {
            for c in ASCII_LOWER.iter() {

                //replace
                let replace = format!("{}{}{}", a, c, b.get(1..).unwrap_or_default());
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

fn edits<'a>(n: usize, word: &'a str) -> Stream<'a, String> {
    let g = Gn::new_scoped(move |mut s| {
        let mut v = vec![word];
        let r = &String::from("fneopfe");
        v.push(r);
        dbg!(v);
        for i in 0..n {
            for w in edit1(word) {
                s.yield_(w);
            }
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
        let data_set = WordDataSet { counter: hashmap!["A" => 2, "B"=> 2] };
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
        let word_dataset = WordDataSet::from(word_list);
        let s = SimpleCorrector { data_set: word_dataset };
        let res = s.correct("ab");
        dbg!(res);
    }


    #[test]
    fn test_corrector_on_invalid_word() {
        let test_word = "aa";
        let word_list = vec!["ab", "cd"];
        let word_dataset = WordDataSet::from(word_list);
        let s = SimpleCorrector { data_set: word_dataset };
        let res = s.correct(test_word);
        assert_eq!(res.unwrap(), "ab");
    }
}


