use crate::trie::Trie;

pub struct Enumerator<'a> {
    trie: &'a Trie,
    text: &'a [u8],
}

impl<'a> Enumerator<'a> {
    pub fn all_subsequences(trie: &'a Trie, text: &'a [u8]) -> Vec<u32> {
        let e = Self { trie, text };
        let mut results = vec![];
        e.all_subsequences_recur(Trie::root_pos(), 0, &mut results);
        results
    }

    pub fn all_subsequences_recur(&self, node_pos: u32, text_pos: usize, results: &mut Vec<u32>) {
        if text_pos == self.text.len() {
            if let Some(v) = self.trie.get_value(node_pos) {
                results.push(v);
            }
            return;
        }
        self.all_subsequences_recur(node_pos, text_pos + 1, results);
        if let Some(node_pos) = self.trie.get_child(node_pos, self.text[text_pos]) {
            self.all_subsequences_recur(node_pos, text_pos + 1, results);
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
        let text = "abaab".as_bytes();
        let trie = Trie::from_records(records);

        let mut results = Enumerator::all_subsequences(&trie, text);
        results.sort();

        let expected = vec![
            0, 0, 0, // "aa"
            1, // "abaab"
            2, // "abb"
            3, 3, // "bab"
            4, // "bb"
        ];
        assert_eq!(results, expected);
    }
}
