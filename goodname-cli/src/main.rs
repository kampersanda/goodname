use std::error::Error;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
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

    #[clap(short = 'k', action, default_value = "30")]
    topk: usize,

    #[clap(short = 'l', action, default_value = "0")]
    prefix_len: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let lex = Lexicon::new(load_lines(&args.wordlist_filename)?)?;
    let prefix_len = args.prefix_len;

    println!("Enter your text:");
    #[allow(clippy::significant_drop_in_scrutinee)]
    for line in stdin().lock().lines() {
        let line = line?;
        let enumerator = Enumerator::new(&lex, &line)?.prefix_len(prefix_len)?;
        let matched = enumerator.all_subsequences()?;
        println!("Matched {} candidates", matched.len());
        let k = args.topk.min(matched.len());
        for (i, m) in matched[..k].iter().enumerate() {
            let (word, desc) = enumerator.format_match(m);
            println!("{:>4} {}: {} (score={})", i + 1, word, desc, m.score);
        }
        println!("Enter your text:");
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
