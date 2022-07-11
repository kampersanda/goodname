use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::string::String;

use goodname::Enumerator;
use goodname::Lexicon;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "goodname-cli", about = "A CLI tool of goodname.")]
struct Args {
    #[clap(short = 'w', action)]
    wordlist_filename: String,

    #[clap(short = 'i', action)]
    input_text: String,

    #[clap(short = 'k', action, default_value = "30")]
    topk: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let lex = Lexicon::new(load_lines(&args.wordlist_filename)?)?;
    let matched = Enumerator::all_subsequences_sorted(&lex, &args.input_text)?;
    println!("Matched {} candidates", matched.len());

    let k = args.topk.min(matched.len());
    for (i, m) in matched[..k].iter().enumerate() {
        println!("{}: {} (score={})", i + 1, lex.word(m.word_id), m.score);
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
