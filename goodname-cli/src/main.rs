use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::string::String;

use goodname::Enumerator;
use goodname::Trie;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "goodname-cli", about = "A CLI tool of goodname.")]
struct Args {
    #[clap(short = 'w', action)]
    words_filename: String,

    #[clap(short = 'i', action)]
    input_text: String,

    #[clap(short = 'k', action, default_value = "100")]
    topk: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let words = load_lines(&args.words_filename)?;
    let trie = Trie::from_words(&words)?;

    let mut matched = Enumerator::all_subsequences(&trie, args.input_text.as_bytes())?;
    matched.sort_by_key(|m| std::cmp::Reverse(m.score));

    let k = args.topk.min(matched.len());
    for m in &matched[..k] {
        println!("{} (score={})", words[m.value], m.score);
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