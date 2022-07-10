use crate::trie::Trie;
use crate::utils;

struct State {
    node_pos: u32,
    text_pos: usize,
}

impl State {
    fn new(node_pos: u32, text_pos: usize) -> Self {
        State { node_pos, text_pos }
    }
}

pub struct Enumerator<'a> {
    trie: &'a Trie,
    text: &'a [u8],
}

impl<'a> Enumerator<'a> {
    pub fn all_subsequences(trie: &'a Trie, text: &'a [u8]) -> Vec<u32> {
        let enumerator = Self { trie, text };
        let mut results = vec![];
        enumerator.all_subsequences_recur(State::new(Trie::root_pos(), 0), &mut results);
        results
    }

    fn all_subsequences_recur(&self, state: State, results: &mut Vec<u32>) {
        let State { node_pos, text_pos } = state;
        if text_pos == self.text.len() {
            if let Some(v) = self.trie.get_value(node_pos) {
                results.push(v);
            }
            return;
        }
        let c = self.text[text_pos];
        if !utils::is_upper_case(c) {
            // Allows an epsilon transition only for non upper letters.
            self.all_subsequences_recur(State::new(node_pos, text_pos + 1), results);
        }
        if let Some(node_pos) = self.trie.get_child(node_pos, utils::to_lower_case(c)) {
            self.all_subsequences_recur(State::new(node_pos, text_pos + 1), results);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toy() {
        let records = &[
            ("aa".as_bytes(), 0),
            ("abaab".as_bytes(), 1),
            ("abb".as_bytes(), 2),
            ("bab".as_bytes(), 3),
            ("bb".as_bytes(), 4),
            ("bbb".as_bytes(), 5),
        ];
        let text = "abAaB".as_bytes();
        let trie = Trie::from_records(records);

        let mut results = Enumerator::all_subsequences(&trie, text);
        results.sort();

        let expected = vec![
            1, // "abAaB"
            3, // "bAB"
        ];
        assert_eq!(results, expected);
    }
}
