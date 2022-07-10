pub struct Trie {
    units: Vec<u32>,
}

impl Trie {
    pub fn from_keyset<T>(keyset: &[(T, u32)]) -> Self
    where
        T: AsRef<[u8]>,
    {
        let data = yada::builder::DoubleArrayBuilder::build(keyset).unwrap();
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
    pub fn get_value(&self, node_pos: u32) -> Option<u32> {
        if Self::has_leaf(self.get_unit(node_pos)) {
            let node_pos = Self::offset(self.get_unit(node_pos)) ^ node_pos;
            Some(Self::value(self.get_unit(node_pos)))
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
        let keyset = &[
            ("a".as_bytes(), 0),
            ("ab".as_bytes(), 1),
            ("aba".as_bytes(), 2),
            ("ac".as_bytes(), 3),
            ("acb".as_bytes(), 4),
            ("acc".as_bytes(), 5),
            ("ad".as_bytes(), 6),
            ("ba".as_bytes(), 7),
            ("bb".as_bytes(), 8),
            ("bc".as_bytes(), 9),
            ("c".as_bytes(), 10),
            ("caa".as_bytes(), 11),
        ];
        let trie = Trie::from_keyset(keyset);
        for &(key, value) in keyset {
            let mut node_pos = Trie::root_pos();
            for &c in key {
                node_pos = trie.get_child(node_pos, c).unwrap();
            }
            assert_eq!(value, trie.get_value(node_pos).unwrap());
        }
    }
}
