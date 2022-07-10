use crate::trie::Trie;

const UPPER_TO_LOWER: [u8; 256] = {
    let mut map = [0; 256];
    let mut i = 1;
    while i < 256 {
        let c = i as u8;
        if b'A' <= c && c <= b'Z' {
            map[i] = c + (b'a' - b'A'); // To the lower one
        }
        i += 1;
    }
    map
};

const fn is_upper_case(c: u8) -> bool {
    UPPER_TO_LOWER[c as usize] != 0
}

const fn to_lower_case(c: u8) -> u8 {
    if is_upper_case(c) {
        UPPER_TO_LOWER[c as usize]
    } else {
        c
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
        enumerator.all_subsequences_recur(Trie::root_pos(), 0, &mut results);
        results
    }

    pub fn all_subsequences_recur(&self, node_pos: u32, text_pos: usize, results: &mut Vec<u32>) {
        if text_pos == self.text.len() {
            if let Some(v) = self.trie.get_value(node_pos) {
                results.push(v);
            }
            return;
        }
        let c = self.text[text_pos];
        // Allows an epsilon transition only for non upper letters.
        if !is_upper_case(c) {
            self.all_subsequences_recur(node_pos, text_pos + 1, results);
        }
        if let Some(node_pos) = self.trie.get_child(node_pos, to_lower_case(c)) {
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

    #[test]
    fn test_letter_case() {
        assert!(!is_upper_case(b'@'));
        assert!(is_upper_case(b'A'));
        assert!(is_upper_case(b'Z'));
        assert!(!is_upper_case(b'['));

        assert_eq!(to_lower_case(b'A'), b'a');
        assert_eq!(to_lower_case(b'Z'), b'z');
        assert_eq!(to_lower_case(b'a'), b'a');
        assert_eq!(to_lower_case(b'z'), b'z');
    }
}
