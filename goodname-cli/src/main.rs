use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::string::String;

use goodname::Enumerator;
use goodname::Trie;

fn main() -> Result<(), Box<dyn Error>> {
    let words = load_lines("words.txt")?;
    let trie = Trie::from_words(&words)?;

    let input = "Character-wise double-array dictionary";
    let mut matched = Enumerator::all_subsequences(&trie, input.as_bytes())?;
    matched.sort_by_key(|m| std::cmp::Reverse(m.score));

    for m in matched {
        println!("{} => {}", words[m.value], m.score);
    }

    Ok(())
}

fn load_lines<P>(path: P) -> std::io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    Ok(buf.lines().map(|line| line.unwrap()).collect())
}
