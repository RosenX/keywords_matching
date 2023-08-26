mod trie;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct MatchResult {
    pub match_words: HashMap<String, usize>,
    pub modified_html: String,
}

type TrieNodeRef = Rc<RefCell<TrieNode>>;

struct TrieNode {
    is_end_words: bool,
    children: HashMap<char, TrieNodeRef>,
    fail: Option<TrieNodeRef>,
    depth: usize,
}

pub struct Trie {
    root: TrieNodeRef,
}
