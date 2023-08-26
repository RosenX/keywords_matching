use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    rc::Rc,
};

use crate::{MatchResult, Trie, TrieNode, TrieNodeRef};

impl TrieNode {
    fn new(depth: usize) -> Self {
        TrieNode {
            is_end_words: false,
            children: HashMap::new(),
            fail: None,
            depth: depth,
        }
    }

    fn get_child(&self, c: &char) -> Option<TrieNodeRef> {
        self.children.get(c).map(|x| x.clone())
    }

    fn get_fail(&self) -> Option<TrieNodeRef> {
        self.fail.as_ref().map(|x| x.clone())
    }
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: Rc::new(RefCell::new(TrieNode::new(0))),
        }
    }

    pub fn build(&mut self, words: Vec<String>) {
        for word in words {
            self.insert(word);
        }
        self.build_fail_point()
    }

    fn insert(&mut self, word: String) {
        let mut p_ref = self.root.clone();
        for c in word.chars() {
            let child = p_ref.borrow().get_child(&c);
            match child {
                Some(child) => {
                    p_ref = child;
                }
                None => {
                    let mut new_node = TrieNode::new(p_ref.borrow().depth + 1);
                    new_node.fail = Some(self.root.clone());
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
                child.borrow_mut().fail = Some(self.root.clone());
            }
        }

        while let Some(p) = queue.pop_front() {
            for (c, child) in &p.borrow().children {
                let mut fail = p.borrow().get_fail();
                while let Some(fail_ref) = fail {
                    let fail_child = fail_ref.borrow().get_child(c);
                    if fail_child.is_some() {
                        child.borrow_mut().fail = fail_child;
                        queue.push_back(child.clone());
                        break;
                    }
                    fail = fail_ref.borrow().fail.clone();
                }
            }
        }
    }

    pub fn search_replace(&self, text: &String) -> MatchResult {
        let mut p_ref = self.root.clone();
        let mut match_result = HashMap::<String, usize>::new();
        let mut modify_text = String::new();
        let mut byte_index: Vec<usize> = vec![];
        for (i, mut c) in text.char_indices() {
            c = c.to_ascii_lowercase();
            byte_index.push(i);
            let mut child = p_ref.borrow().get_child(&c);
            while child.is_none() {
                let fail = p_ref.borrow().get_fail();
                match fail {
                    Some(fail_ref) => {
                        p_ref = fail_ref;
                        child = p_ref.borrow().get_child(&c);
                    }
                    None => {
                        break;
                    }
                }
            }
            match child {
                Some(child) => {
                    p_ref = child;
                    if p_ref.borrow().is_end_words {
                        let word_len = p_ref.borrow().depth;
                        let start = byte_index[byte_index.len() - word_len];
                        let end = i + c.len_utf8();
                        let word = &text[start..end].to_ascii_lowercase().to_string();
                        let count = match_result.entry(word.to_string()).or_insert(0);
                        *count += 1;
                        // pop word_len-1 char from modify_text
                        for _ in 0..word_len - 1 {
                            modify_text.pop();
                        }

                        // push
                        modify_text
                            .push_str(format!("<span class=\"keyword\">{}</span>", word).as_str());
                    } else {
                        modify_text.push(c);
                    }
                }
                None => {
                    p_ref = self.root.clone();
                    modify_text.push(c);
                }
            }
        }
        MatchResult {
            match_words: match_result,
            modified_html: modify_text,
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
        assert!(trie.root.borrow().get_child(&'h').is_some());
        assert!(trie.root.borrow().get_child(&'s').is_some());
        assert!(trie.root.borrow().get_child(&'a').is_none());

        let h_node = trie.root.borrow().get_child(&'h').unwrap();
        assert!(h_node.borrow().get_child(&'e').is_some());
        assert!(h_node.borrow().get_child(&'s').is_none());

        let s_node = trie.root.borrow().get_child(&'s').unwrap();
        assert!(s_node.borrow().get_child(&'h').is_some());
        assert!(s_node.borrow().get_child(&'a').is_some());
        assert!(s_node.borrow().get_child(&'e').is_none());

        let sh_node = s_node.borrow().get_child(&'h').unwrap();
        assert!(sh_node.borrow().get_child(&'r').is_some());
        assert!(sh_node.borrow().get_child(&'e').is_some());

        assert!(sh_node.borrow().get_fail().is_some());

        let she_node = sh_node.borrow().get_child(&'e').unwrap();
        assert!(she_node.borrow().get_fail().is_some());
        assert!(she_node.borrow().is_end_words == true);
    }
}
