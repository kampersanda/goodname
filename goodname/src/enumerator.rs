use std::collections::HashMap;
use std::num::NonZeroU8;

use anyhow::{anyhow, Result};

use crate::utils;
use crate::{trie::Trie, Lexicon};

const DELIMITER: u8 = b' ';
const MAX_MATCHES: usize = 10000;
const MAX_PREFIX_LEN: usize = 3;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
struct Prefix([Option<NonZeroU8>; MAX_PREFIX_LEN + 1]);

impl Prefix {
    #[inline(always)]
    fn new() -> Self {
        Self::default()
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.0.iter().position(|&x| x.is_none()).unwrap()
    }

    #[inline(always)]
    fn push(mut self, c: u8) -> Self {
        let i = self.len();
        debug_assert!(i < MAX_PREFIX_LEN);
        self.0[i] = NonZeroU8::new(c);
        self
    }

    #[inline(always)]
    fn string(&self) -> String {
        let buf: Vec<_> = self.0[..self.len()]
            .iter()
            .map(|&c| c.unwrap().get())
            .collect();
        String::from_utf8(buf).unwrap()
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Match {
    pub word_id: usize,
    pub score: usize,
    pub positions: u128,
    pub prefix: String,
}

struct State {
    node_pos: u32,
    text_pos: usize,
    score: usize,
    positions: u128,
    prefix: Prefix,
}

impl State {
    #[inline(always)]
    const fn new(
        node_pos: u32,
        text_pos: usize,
        score: usize,
        positions: u128,
        prefix: Prefix,
    ) -> Self {
        Self {
            node_pos,
            text_pos,
            score,
            positions,
            prefix,
        }
    }
}

pub struct Enumerator<'a> {
    lex: &'a Lexicon,
    text: &'a [u8],
    scores: Vec<usize>,
    prefix_len: usize,
}

impl<'a> Enumerator<'a> {
    pub fn init(lex: &'a Lexicon, text: &'a str) -> Result<Self> {
        let text = text.as_bytes();
        if 128 <= text.len() {
            return Err(anyhow!(
                "the length of an input text must be less than 128."
            ));
        }
        let scores = Self::build_scores(text);
        let enumerator = Self {
            lex,
            text,
            scores,
            prefix_len: 0,
        };
        Ok(enumerator)
    }

    pub fn prefix_len(mut self, prefix_len: usize) -> Result<Self> {
        if MAX_PREFIX_LEN < prefix_len {
            return Err(anyhow!(
                "the prefix length must be no more than {}.",
                MAX_PREFIX_LEN
            ));
        }
        self.prefix_len = prefix_len;
        Ok(self)
    }

    pub fn all_subsequences(&self) -> Result<Vec<Match>> {
        let mut matched = HashMap::new();
        self.all_subsequences_recur(
            State::new(Trie::root_pos(), 0, 0, 0, Prefix::new()),
            &mut matched,
        )?;
        let mut matched: Vec<_> = matched.iter().map(|(_, m)| m.clone()).collect();
        matched.sort_by(|m1, m2| {
            m2.score
                .cmp(&m1.score)
                .then_with(|| m1.word_id.cmp(&m2.word_id))
        });
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
            positions,
            prefix,
        } = state;

        if text_pos == 0 && prefix.len() < self.prefix_len {
            for c in b'a'..=b'z' {
                if let Some(child_pos) = self.lex.trie().get_child(node_pos, c) {
                    // Because score is not incremented, the score of a recursive acronym never become
                    // larger than that of the equivalent acronym.
                    self.all_subsequences_recur(
                        State::new(child_pos, text_pos, score, positions, prefix.push(c)),
                        matched,
                    )?;
                }
            }
        }

        if text_pos == self.text.len() {
            if let Some(word_id) = self.lex.trie().get_value(node_pos) {
                matched
                    .entry(word_id)
                    .and_modify(|m| {
                        debug_assert_eq!(m.word_id, word_id);
                        if m.score < score {
                            m.score = score;
                            m.positions = positions;
                            m.prefix = prefix.string();
                        }
                    })
                    .or_insert(Match {
                        word_id,
                        score,
                        positions,
                        prefix: prefix.string(),
                    });
                if MAX_MATCHES <= matched.len() {
                    return Err(anyhow!(
                        "#matches is too many, exceeding {}. Adjust the number by shortening the description or specifying more uppercase letters.",
                        MAX_MATCHES
                    ));
                }
            }
            return Ok(());
        }

        let c = self.text[text_pos];
        if !utils::is_upper_case(c) {
            // Allows an epsilon transition only for non upper letters.
            self.all_subsequences_recur(
                State::new(node_pos, text_pos + 1, score, positions, prefix),
                matched,
            )?;
        }

        let c = utils::to_lower_case(c).unwrap_or(c);
        if let Some(child_pos) = self.lex.trie().get_child(node_pos, c) {
            self.all_subsequences_recur(
                State::new(
                    child_pos,
                    text_pos + 1,
                    score + self.scores[text_pos],
                    positions | (1 << text_pos),
                    prefix,
                ),
                matched,
            )?;
        }
        Ok(())
    }

    pub fn format_match(&self, m: &Match) -> (String, String) {
        let word = {
            let word = self.lex.word(m.word_id);
            assert!(word.starts_with(&m.prefix));
            let mut bytes = word.as_bytes().to_vec();
            for c in bytes[..m.prefix.len()].iter_mut() {
                *c = utils::to_upper_case(*c).unwrap();
            }
            String::from_utf8(bytes).unwrap()
        };
        let desc = {
            let mut bytes = self.text.to_vec();
            for (i, c) in bytes.iter_mut().enumerate() {
                if m.positions & (1 << i) != 0 {
                    *c = utils::to_upper_case(*c).unwrap_or(*c);
                } else {
                    assert!(!utils::is_upper_case(*c));
                }
            }
            String::from_utf8(bytes).unwrap()
        };
        (word, desc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enumerate() {
        let words = &["aa", "abaab", "abb", "bab", "bb", "bbb", "cbab", "ccbab"];
        let lex = Lexicon::new(words).unwrap();
        let text = "abAaB";

        let matched = Enumerator::init(&lex, text)
            .unwrap()
            .all_subsequences()
            .unwrap();
        let expected = vec![
            Match {
                word_id: 1,
                score: 31,
                positions: 0b11111,
                prefix: "".to_string(),
            }, // "abAaB"
            Match {
                word_id: 3,
                score: 13,
                positions: 0b10110,
                prefix: "".to_string(),
            }, // "bAB"
        ];
        assert_eq!(matched, expected);
    }

    #[test]
    fn test_enumerate_1() {
        let words = &["aa", "abaab", "abb", "bab", "bb", "bbb", "cbab", "ccbab"];
        let lex = Lexicon::new(words).unwrap();
        let text = "abAaB";

        let matched = Enumerator::init(&lex, text)
            .unwrap()
            .prefix_len(1)
            .unwrap()
            .all_subsequences()
            .unwrap();
        let expected = vec![
            Match {
                word_id: 1,
                score: 31,
                positions: 0b11111,
                prefix: "".to_string(),
            }, // "abAaB"
            Match {
                word_id: 3,
                score: 13,
                positions: 0b10110,
                prefix: "".to_string(),
            }, // "bAB"
            Match {
                word_id: 6,
                score: 13,
                positions: 0b10110,
                prefix: "c".to_string(),
            }, // "c|bAB"
        ];
        assert_eq!(matched, expected);
    }

    #[test]
    fn test_enumerate_2() {
        let words = &["aa", "abaab", "abb", "bab", "bb", "bbb", "cbab", "ccbab"];
        let lex = Lexicon::new(words).unwrap();
        let text = "abAaB";

        let matched = Enumerator::init(&lex, text)
            .unwrap()
            .prefix_len(2)
            .unwrap()
            .all_subsequences()
            .unwrap();
        let expected = vec![
            Match {
                word_id: 1,
                score: 31,
                positions: 0b11111,
                prefix: "".to_string(),
            }, // "abAaB"
            Match {
                word_id: 3,
                score: 13,
                positions: 0b10110,
                prefix: "".to_string(),
            }, // "bAB"
            Match {
                word_id: 6,
                score: 13,
                positions: 0b10110,
                prefix: "c".to_string(),
            }, // "c|bAB"
            Match {
                word_id: 7,
                score: 13,
                positions: 0b10110,
                prefix: "cc".to_string(),
            }, // "cc|bAB"
        ];
        assert_eq!(matched, expected);
    }

    #[test]
    fn test_build_score() {
        let text = "ab abc a".as_bytes();
        let scores = Enumerator::build_scores(text);
        assert_eq!(scores, vec![4, 2, 0, 4, 2, 1, 0, 4]);
    }

    #[test]
    fn test_prefix() {
        let mut prefix = Prefix::new();
        assert_eq!(prefix.len(), 0);

        prefix = prefix.push(b'a');
        prefix = prefix.push(b'b');
        prefix = prefix.push(b'c');
        assert_eq!(prefix.len(), 3);
        assert_eq!(prefix.string(), "abc");
    }
}
