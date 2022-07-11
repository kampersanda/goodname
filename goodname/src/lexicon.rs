use anyhow::Result;

use crate::trie::Trie;

pub struct Lexicon {
    words: Vec<String>,
    trie: Trie,
}

impl Lexicon {
    pub fn new<I, W>(words: I) -> Result<Self>
    where
        I: IntoIterator<Item = W>,
        W: AsRef<str>,
    {
        let words: Vec<_> = words.into_iter().map(|w| w.as_ref().to_string()).collect();
        let trie = Trie::from_words(&words)?;
        Ok(Self { words, trie })
    }

    pub fn word(&self, word_id: usize) -> &str {
        &self.words[word_id]
    }

    pub const fn trie(&self) -> &Trie {
        &self.trie
    }
}
