use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

type TrieNodeRef = Rc<RefCell<TrieNode>>;

struct TrieNode {
    is_end_words: bool,
    children: HashMap<char, TrieNodeRef>,
    fail: Option<TrieNodeRef>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            is_end_words: false,
            children: HashMap::new(),
            fail: None,
        }
    }

    fn get_child(&self, c: char) -> Option<TrieNodeRef> {
        self.children.get(&c).map(|x| x.clone())
    }

    fn get_fail(&self) -> Option<TrieNodeRef> {
        self.fail.as_ref().map(|x| x.clone())
    }
}

pub struct Trie {
    root: TrieNodeRef,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: Rc::new(RefCell::new(TrieNode::new())),
        }
    }

    pub fn build(&mut self, words: Vec<String>) {
        for word in words {
            self.insert(word);
        }
    }

    fn insert(&mut self, word: String) {
        let mut p_ref = self.root.clone();
        for c in word.chars() {
            let child = p_ref.borrow().get_child(c);
            match child {
                Some(child) => {
                    p_ref = child;
                }
                None => {
                    let new_node = TrieNode::new();
                    let new_node_ref = Rc::new(RefCell::new(new_node));
                    p_ref.borrow_mut().children.insert(c, new_node_ref.clone());
                    p_ref = new_node_ref;
                }
            }
        }
        p_ref.borrow_mut().is_end_words = true;
    }

    pub fn build_fail_point(&mut self) {
        let mut queue = VecDeque::<TrieNodeRef>::new();
        {
            for (_, child) in &self.root.as_ref().borrow().children {
                queue.push_back(child.clone());
            }
        }

        while !queue.is_empty() {
            let p = queue.pop_front().unwrap();
            for (c, child) in &p.borrow().children {
                let mut fail = p.borrow().get_fail();
                while let Some(fail_ref) = fail {
                    if fail_ref.borrow().children.contains_key(&c) {
                        child.borrow_mut().fail = Some(fail_ref);
                        queue.push_back(child.clone());
                        break;
                    }
                    fail = fail_ref.borrow().fail.clone();
                }
            }
        }
    }
}

mod tests {

    #[test]
    fn test_trie() {
        use crate::Trie;
        let mut trie = Trie::new();
        trie.build(vec![
            "he".to_string(),
            "her".to_string(),
            "say".to_string(),
            "she".to_string(),
            "shr".to_string(),
        ]);
        trie.build_fail_point();
        assert!(trie.root.borrow().get_child('h').is_some());
        assert!(trie.root.borrow().get_child('s').is_some());
        assert!(trie.root.borrow().get_child('a').is_none());
    }
}
