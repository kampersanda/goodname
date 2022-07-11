use anyhow::{anyhow, Result};

use crate::utils;

pub struct Trie {
    units: Vec<u32>,
}

impl Trie {
    pub fn from_words<W>(words: &[W]) -> Result<Self>
    where
        W: AsRef<str>,
    {
        let records: Vec<_> = words
            .iter()
            .enumerate()
            .map(|(i, k)| (k.as_ref(), u32::try_from(i).unwrap()))
            .collect();
        Self::verify_words(&records)?;
        let data = yada::builder::DoubleArrayBuilder::build(&records)
            .ok_or_else(|| anyhow!("Failed to run yada::builder::DoubleArrayBuilder::build."))?;
        assert_eq!(data.len() % 4, 0);
        let mut units = Vec::with_capacity(data.len() / 4);
        for i in (0..data.len()).step_by(4) {
            units.push(u32::from_le_bytes(data[i..i + 4].try_into().unwrap()));
        }
        Ok(Self { units })
    }

    fn verify_words<W>(records: &[(W, u32)]) -> Result<()>
    where
        W: AsRef<str>,
    {
        if records.is_empty() {
            return Err(anyhow!("Input words must not be empty."));
        }
        let a = records[0].0.as_ref();
        if a.is_empty() {
            return Err(anyhow!("Input words must not contain an empty one."));
        }
        Self::verify_ascii(a)?;
        for i in 1..records.len() {
            let a = records[i - 1].0.as_ref();
            let b = records[i].0.as_ref();
            if a >= b {
                return Err(anyhow!("Input words must be sorted ({} vs {}).", a, b));
            }
            Self::verify_ascii(b)?;
        }
        Ok(())
    }

    fn verify_ascii<W>(word: W) -> Result<()>
    where
        W: AsRef<str>,
    {
        let word = word.as_ref();
        for &c in word.as_bytes() {
            if c >= 0x80 {
                return Err(anyhow!(
                    "Input words must not contain multibyte characters ({}).",
                    word
                ));
            }
            if utils::is_upper_case(c) {
                return Err(anyhow!(
                    "Input words must not contain upper-case letters ({}).",
                    word
                ));
            }
        }
        Ok(())
    }

    #[inline(always)]
    pub(crate) const fn root_pos() -> u32 {
        0
    }

    #[inline(always)]
    pub(crate) fn get_value(&self, node_pos: u32) -> Option<usize> {
        if Self::has_leaf(self.get_unit(node_pos)) {
            let node_pos = Self::offset(self.get_unit(node_pos)) ^ node_pos;
            Some(Self::value(self.get_unit(node_pos)) as usize)
        } else {
            None
        }
    }

    #[inline(always)]
    pub(crate) fn get_child(&self, node_pos: u32, c: u8) -> Option<u32> {
        let c = c as u32;
        let node_pos = Self::offset(self.get_unit(node_pos)) ^ node_pos ^ c;
        Some(node_pos).filter(|&i| Self::label(self.get_unit(i)) == c)
    }

    #[inline(always)]
    fn get_unit(&self, node_pos: u32) -> u32 {
        self.units[node_pos as usize]
    }

    #[inline(always)]
    const fn has_leaf(unit: u32) -> bool {
        ((unit >> 8) & 1) == 1
    }

    #[inline(always)]
    const fn value(unit: u32) -> u32 {
        unit & ((1 << 31) - 1)
    }

    #[inline(always)]
    const fn label(unit: u32) -> u32 {
        unit & ((1 << 31) | 0xFF)
    }

    #[inline(always)]
    const fn offset(unit: u32) -> u32 {
        (unit >> 10) << ((unit & (1 << 9)) >> 6)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie() {
        let words = &["aa", "abaab", "abb", "bab", "bb", "bbb"];
        let trie = Trie::from_words(words).unwrap();
        for (i, &word) in words.iter().enumerate() {
            let mut node_pos = Trie::root_pos();
            for &c in word.as_bytes() {
                node_pos = trie.get_child(node_pos, c).unwrap();
            }
            assert_eq!(i, trie.get_value(node_pos).unwrap());
        }
    }

    #[test]
    #[should_panic]
    fn test_empty_set() {
        Trie::from_words(&[""][0..0]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_empty_word() {
        Trie::from_words(&["", "a", "b"]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_unsorted() {
        Trie::from_words(&["a", "c", "b"]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_uppercase() {
        Trie::from_words(&["a", "B", "c"]).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_multibyte() {
        Trie::from_words(&["a", "Ｂ", "c"]).unwrap();
    }
}
