pub struct Trie {
    units: Vec<u32>,
}

impl Trie {
    pub fn from_words<K>(words: &[K]) -> Self
    where
        K: AsRef<[u8]>,
    {
        let records: Vec<_> = words
            .iter()
            .enumerate()
            .map(|(i, k)| (k.as_ref(), i as u32))
            .collect();
        let data = yada::builder::DoubleArrayBuilder::build(&records).unwrap();
        assert_eq!(data.len() % 4, 0);
        let mut units = Vec::with_capacity(data.len() / 4);
        for i in (0..data.len()).step_by(4) {
            units.push(u32::from_le_bytes(data[i..i + 4].try_into().unwrap()));
        }
        Self { units }
    }

    #[inline(always)]
    pub const fn root_pos() -> u32 {
        0
    }

    #[inline(always)]
    pub fn get_value(&self, node_pos: u32) -> Option<usize> {
        if Self::has_leaf(self.get_unit(node_pos)) {
            let node_pos = Self::offset(self.get_unit(node_pos)) ^ node_pos;
            Some(Self::value(self.get_unit(node_pos)) as usize)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn get_child(&self, node_pos: u32, c: u8) -> Option<u32> {
        let c = c as u32;
        let node_pos = Self::offset(self.get_unit(node_pos)) ^ node_pos ^ c;
        Some(node_pos).filter(|&node_pos| Self::label(self.get_unit(node_pos)) == c)
    }

    #[inline(always)]
    fn get_unit(&self, node_pos: u32) -> u32 {
        self.units[node_pos as usize]
    }

    #[inline(always)]
    fn has_leaf(unit: u32) -> bool {
        ((unit >> 8) & 1) == 1
    }

    #[inline(always)]
    fn value(unit: u32) -> u32 {
        unit & ((1 << 31) - 1)
    }

    #[inline(always)]
    fn label(unit: u32) -> u32 {
        unit & ((1 << 31) | 0xFF)
    }

    #[inline(always)]
    fn offset(unit: u32) -> u32 {
        (unit >> 10) << ((unit & (1 << 9)) >> 6)
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
        let trie = Trie::from_words(words);
        for (i, &word) in words.iter().enumerate() {
            let mut node_pos = Trie::root_pos();
            for &c in word {
                node_pos = trie.get_child(node_pos, c).unwrap();
            }
            assert_eq!(i, trie.get_value(node_pos).unwrap());
        }
    }
}
