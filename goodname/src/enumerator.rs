use std::collections::HashMap;

use anyhow::{anyhow, Result};

use crate::trie::Trie;
use crate::utils;

const DELIMITER: u8 = b' ';
const MAX_MATCHES: usize = 65536;
const MIN_LENGTH: usize = 3;
const SCORE_FACTOR: usize = 1;

struct State {
    node_pos: u32,
    text_pos: usize,
    score: usize,
    length: usize,
}

impl State {
    const fn new(node_pos: u32, text_pos: usize, score: usize, length: usize) -> Self {
        Self {
            node_pos,
            text_pos,
            score,
            length,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Match {
    pub value: usize,
    pub score: usize,
}

pub struct Enumerator<'a> {
    trie: &'a Trie,
    text: &'a [u8],
    scores: Vec<usize>,
}

impl<'a> Enumerator<'a> {
    pub fn all_subsequences(trie: &'a Trie, text: &'a [u8]) -> Result<Vec<Match>> {
        let scores = Self::build_scores(text);
        let enumerator = Self { trie, text, scores };
        let mut matched = HashMap::new();
        enumerator.all_subsequences_recur(State::new(Trie::root_pos(), 0, 0, 0), &mut matched)?;
        Ok(matched.iter().map(|(_, &m)| m).collect())
    }

    fn build_scores(text: &'a [u8]) -> Vec<usize> {
        let mut scores = vec![0; text.len()];
        let max_score = text
            .split(|&c| c == DELIMITER)
            .fold(0, |max, sub| max.max(sub.len()))
            * SCORE_FACTOR;
        let mut curr_score = 0;
        for (&c, score) in text.iter().zip(scores.iter_mut()) {
            if c == DELIMITER {
                curr_score = 0;
            } else if curr_score == 0 {
                curr_score = max_score;
            } else {
                curr_score -= SCORE_FACTOR;
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
            length,
        } = state;

        if text_pos == self.text.len() {
            if length < MIN_LENGTH {
                return Ok(());
            }
            if let Some(value) = self.trie.get_value(node_pos) {
                matched
                    .entry(value)
                    .and_modify(|m| {
                        m.value = m.value.max(value);
                    })
                    .or_insert(Match { value, score });
                if MAX_MATCHES <= matched.len() {
                    return Err(anyhow!("#matches exceeds {}.", MAX_MATCHES));
                }
            }
            return Ok(());
        }
        let c = self.text[text_pos];
        if !utils::is_upper_case(c) {
            // Allows an epsilon transition only for non upper letters.
            self.all_subsequences_recur(
                State::new(node_pos, text_pos + 1, score, length),
                matched,
            )?;
        }
        if let Some(node_pos) = self.trie.get_child(node_pos, utils::to_lower_case(c)) {
            self.all_subsequences_recur(
                State::new(
                    node_pos,
                    text_pos + 1,
                    score + self.scores[text_pos],
                    length + 1,
                ),
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
    fn test_toy() {
        let words = &[
            "aa".as_bytes(),
            "abaab".as_bytes(),
            "abb".as_bytes(),
            "bab".as_bytes(),
            "bb".as_bytes(),
            "bbb".as_bytes(),
        ];
        let trie = Trie::from_words(words).unwrap();
        let text = "abAaB".as_bytes();

        let mut matched = Enumerator::all_subsequences(&trie, text).unwrap();
        matched.sort_by_key(|m| std::cmp::Reverse(m.score));

        let expected = vec![
            Match {
                value: 1,
                score: 15,
            }, // "abAaB"
            Match { value: 3, score: 8 }, // "bAB"
        ];
        assert_eq!(matched, expected);
    }
}
