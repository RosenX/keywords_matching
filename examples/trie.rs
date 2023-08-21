use keywords_matching::Trie;

fn main() {
    let mut trie = Trie::new();
    trie.build(vec![
        "OpenAI".to_string(),
        "Stack Overflow".to_string(),
        "prompt engineering".to_string(),
    ]);
    trie.build_fail_point();

    // read from test.html
    let read = std::fs::read_to_string("examples/test.html").unwrap();

    let res = trie.search_replace(&read);

    println!("{:?}", res.match_words);

    // save res.text to test_modify.html
    std::fs::write("examples/test_modify.html", res.modify_text).unwrap();
}
