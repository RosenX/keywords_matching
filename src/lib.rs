mod trie;

use std::collections::HashMap;

pub use trie::Trie;

#[derive(Debug)]
pub struct MatchResult {
    pub match_words: HashMap<String, usize>,
    pub modified_html: String,
}
