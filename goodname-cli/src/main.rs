use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::string::String;

use goodname::Enumerator;
use goodname::Trie;

fn main() {
    let words = load_lines("words.txt");
    let trie = Trie::from_words(&words).unwrap();

    let input = "Character-wise double-array dictionary";
    let mut matched = Enumerator::all_subsequences(&trie, input.as_bytes());
    matched.sort_by_key(|m| std::cmp::Reverse(m.score));

    for m in matched {
        println!("{} => {}", words[m.value], m.score);
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
