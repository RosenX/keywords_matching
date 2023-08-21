use keywords_matching::Trie;

fn main() {
    let mut trie = Trie::new();
    trie.build(vec!["hello".to_string(), "world".to_string()]);
}
