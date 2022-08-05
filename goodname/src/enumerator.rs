use std::collections::HashMap;

use anyhow::{anyhow, Result};

use crate::utils;
use crate::{trie::Trie, Lexicon};

const DELIMITER: u8 = b' ';
const MAX_MATCHES: usize = 10000;

struct State {
    node_pos: u32,
    text_pos: usize,
    score: usize,
}

impl State {
    #[inline(always)]
    const fn new(node_pos: u32, text_pos: usize, score: usize) -> Self {
        Self {
            node_pos,
            text_pos,
            score,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Match {
    pub word_id: usize,
    pub score: usize,
}

pub struct Enumerator<'a> {
    trie: &'a Trie,
    text: &'a [u8],
    scores: Vec<usize>,
}

impl<'a> Enumerator<'a> {
    pub fn all_subsequences(lex: &'a Lexicon, text: &'a str) -> Result<Vec<Match>> {
        let text = text.as_bytes();
        let scores = Self::build_scores(text);
        let enumerator = Self {
            trie: lex.trie(),
            text,
            scores,
        };
        let mut matched = HashMap::new();
        enumerator.all_subsequences_recur(State::new(Trie::root_pos(), 0, 0), &mut matched)?;
        Ok(matched.iter().map(|(_, &m)| m).collect())
    }

    pub fn all_subsequences_sorted(lex: &'a Lexicon, text: &'a str) -> Result<Vec<Match>> {
        let mut matched = Self::all_subsequences(lex, text)?;
        matched.sort_by_key(|m| std::cmp::Reverse(m.score));
        Ok(matched)
    }

    fn build_scores(text: &'a [u8]) -> Vec<usize> {
        let mut scores = vec![0; text.len()];
        let max_word_len = text
            .split(|&c| c == DELIMITER)
            .fold(0, |max, sub| max.max(sub.len()));
        let max_score = 1 << (max_word_len - 1);
        let mut curr_score = 0;
        for (&c, score) in text.iter().zip(scores.iter_mut()) {
            if c == DELIMITER {
                curr_score = 0;
            } else if curr_score == 0 {
                curr_score = max_score;
            } else {
                curr_score /= 2;
            }
            *score = curr_score;
        }
        scores
    }

    fn all_subsequences_recur(
        &self,
        state: State,
        matched: &mut HashMap<usize, Match>,
    ) -> Result<()> {
        let State {
            node_pos,
            text_pos,
            score,
        } = state;

        if text_pos == self.text.len() {
            if let Some(word_id) = self.trie.get_value(node_pos) {
                matched
                    .entry(word_id)
                    .and_modify(|m| {
                        m.word_id = m.word_id.max(word_id);
                    })
                    .or_insert(Match { word_id, score });
                if MAX_MATCHES <= matched.len() {
                    return Err(anyhow!(
                        "#matches is too many, exceeding {}. Please reconsider your input.",
                        MAX_MATCHES
                    ));
                }
            }
            return Ok(());
        }

        let c = self.text[text_pos];

        if !utils::is_upper_case(c) {
            // Allows an epsilon transition only for non upper letters.
            self.all_subsequences_recur(State::new(node_pos, text_pos + 1, score), matched)?;
        }

        if let Some(node_pos) = self.trie.get_child(node_pos, utils::to_lower_case(c)) {
            self.all_subsequences_recur(
                State::new(node_pos, text_pos + 1, score + self.scores[text_pos]),
                matched,
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enumerate() {
        let words = &["aa", "abaab", "abb", "bab", "bb", "bbb"];
        let lex = Lexicon::new(words).unwrap();
        let text = "abAaB";

        let matched = Enumerator::all_subsequences_sorted(&lex, text).unwrap();
        let expected = vec![
            Match {
                word_id: 1,
                score: 31,
            }, // "abAaB"
            Match {
                word_id: 3,
                score: 13,
            }, // "bAB"
        ];
        assert_eq!(matched, expected);
    }

    #[test]
    fn test_build_score() {
        let text = "ab abc a".as_bytes();
        let scores = Enumerator::build_scores(text);
        assert_eq!(scores, vec![4, 2, 0, 4, 2, 1, 0, 4]);
    }
}
