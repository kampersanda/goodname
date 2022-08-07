use std::error::Error;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::Path;
use std::string::String;

use goodname::Lexicon;
use goodname::{activate_positions, Enumerator};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "goodname-cli", about = "A CLI tool of goodname.")]
struct Args {
    #[clap(short = 'w', action)]
    wordlist_filename: String,

    #[clap(short = 'k', action, default_value = "30")]
    topk: usize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let lex = Lexicon::new(load_lines(&args.wordlist_filename)?)?;

    println!("Enter your text:");
    #[allow(clippy::significant_drop_in_scrutinee)]
    for line in stdin().lock().lines() {
        let line = line?;
        let matched = Enumerator::all_subsequences_sorted(&lex, &line)?;
        println!("Matched {} candidates", matched.len());
        let k = args.topk.min(matched.len());
        for (i, m) in matched[..k].iter().enumerate() {
            let formatted = activate_positions(&line, m);
            println!(
                "{:>4} {}: {} (score={})",
                i + 1,
                lex.word(m.word_id),
                formatted,
                m.score
            );
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
