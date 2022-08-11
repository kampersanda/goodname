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
//! let enumerator = Enumerator::new(&lex, text).unwrap().prefix_len(2).unwrap();
//! let matched = enumerator.all_subsequences().unwrap();
//!
//! assert_eq!(matched.len(), 4);
//! assert_eq!(
//!     enumerator.format_match(&matched[0]),
//!     ("abaab".to_string(), "ABAAB".to_string())
//! );
//! assert_eq!(
//!     enumerator.format_match(&matched[1]),
//!     ("bab".to_string(), "aBAaB".to_string())
//! );
//! assert_eq!(
//!     enumerator.format_match(&matched[2]),
//!     ("Cbab".to_string(), "aBAaB".to_string())
//! );
//! assert_eq!(
//!     enumerator.format_match(&matched[3]),
//!     ("CCbab".to_string(), "aBAaB".to_string())
//! );
//! ```
mod enumerator;
mod lexicon;
mod trie;
mod utils;

pub use enumerator::{Enumerator, Match};
pub use lexicon::Lexicon;
