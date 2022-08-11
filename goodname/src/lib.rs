//! # Goodname: Tool to assist you with cool naming of your methods and software.
//!
//! Goodname is a tool to assist you with cool naming of your methods and software.
//! Given a brief description of your method or software,
//! this tool enumerates name candidates forming subsequences of the description (i.e., *acronym*).
mod enumerator;
mod lexicon;
mod trie;
mod utils;

pub use enumerator::{Enumerator, Match};
pub use lexicon::Lexicon;
