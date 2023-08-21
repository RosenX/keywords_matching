mod trie;

pub use trie::Trie;

#[derive(Debug)]
pub struct MatchResult {
    pub match_words: Vec<String>,
    pub modify_text: String,
}
