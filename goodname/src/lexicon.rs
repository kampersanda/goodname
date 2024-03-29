use anyhow::Result;

use crate::trie::Trie;

/// Word lexicon.
pub struct Lexicon {
    words: Vec<String>,
    trie: Trie,
}

impl Lexicon {
    /// Creates an instance from a set of words.
    pub fn new<I, W>(words: I) -> Result<Self>
    where
        I: IntoIterator<Item = W>,
        W: AsRef<str>,
    {
        let words: Vec<_> = words.into_iter().map(|w| w.as_ref().to_string()).collect();
        let trie = Trie::from_words(&words)?;
        Ok(Self { words, trie })
    }

    /// Gets the word.
    pub fn word(&self, word_id: usize) -> &str {
        &self.words[word_id]
    }

    /// Gets the reference of the trie.
    pub const fn trie(&self) -> &Trie {
        &self.trie
    }
}
