//! # Goodname: Tool to assist you with cool naming of your methods and software
//!
//! Goodname is a tool to assist you with cool naming of your methods and software.
//! Given a brief description of your method or software,
//! this tool enumerates name candidates forming subsequences of the description (i.e., *acronym*).
//!
//! ## Examples
//!
//! ```
//! use goodname::{Enumerator, Lexicon, Match};
//!
//! let words = &["aa", "abaab", "abb", "bab", "bb", "bbb", "cbab", "ccbab"];
//! let lex = Lexicon::new(words).unwrap();
//! let text = "abAaB";
//!
//! let matched = Enumerator::new(&lex, text)
//!     .unwrap()
//!     .prefix_len(1)
//!     .unwrap()
//!     .all_subsequences()
//!     .unwrap();
//!
//! let expected = vec![
//!     Match {
//!         word_id: 1,
//!         score: 31,
//!         positions: 0b11111,
//!         prefix: "".to_string(),
//!     }, // "abAaB"
//!     Match {
//!         word_id: 3,
//!         score: 13,
//!         positions: 0b10110,
//!         prefix: "".to_string(),
//!     }, // "bAB"
//!     Match {
//!         word_id: 6,
//!         score: 13,
//!         positions: 0b10110,
//!         prefix: "c".to_string(),
//!     }, // "c|bAB"
//! ];
//!
//! assert_eq!(matched, expected);
//! ```
mod enumerator;
mod lexicon;
mod trie;
mod utils;

pub use enumerator::{Enumerator, Match};
pub use lexicon::Lexicon;
