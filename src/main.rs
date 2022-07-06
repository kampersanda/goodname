use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::string::String;

fn main() {
    let words = load_lines("words.txt");
    let dict = Dictionary::new(words);

    let input = "abcd";
    enumurate_all_subsequences(input.as_bytes().to_vec(), vec![], &dict);
}

fn enumurate_all_subsequences(input: Vec<u8>, output: Vec<u8>, dict: &Dictionary) {
    if input.is_empty() {
        if let Some(i) = dict.get(&output) {
            println!("{} => {}", String::from_utf8(output).unwrap(), i);
        }
        return;
    }

    let mut concat = output.clone();
    concat.push(input[0]);

    enumurate_all_subsequences(input[1..].to_vec(), concat, dict);
    enumurate_all_subsequences(input[1..].to_vec(), output, dict);
}

struct Dictionary {
    map: HashMap<Vec<u8>, usize>,
}

impl Dictionary {
    fn new<I, W>(words: I) -> Self
    where
        I: IntoIterator<Item = W>,
        W: AsRef<str>,
    {
        let mut map = HashMap::new();
        for (i, word) in words.into_iter().enumerate() {
            let word = word.as_ref().as_bytes();
            map.insert(word.to_vec(), i);
        }
        Self { map }
    }

    fn get(&self, word: &[u8]) -> Option<usize> {
        self.map.get(word).cloned()
    }
}

fn load_lines<P>(path: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let file = File::open(path).unwrap();
    let buf = BufReader::new(file);
    buf.lines().map(|line| line.unwrap()).collect()
}
