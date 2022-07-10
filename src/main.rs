pub mod enumerator;
pub mod trie;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::string::String;

use enumerator::Enumerator;
use trie::Trie;

fn main() {
    let words = load_lines("words.txt");
    let trie = Trie::from_keys(&words);

    let input = "Character-wise Double-array Dictionary";
    let mut ids = Enumerator::all_subsequences(&trie, input.as_bytes());
    ids.sort();
    ids.dedup();

    for id in ids {
        println!("{}", words[id as usize]);
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
