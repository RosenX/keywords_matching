use html_keywords_matching::{MatchResult, Trie};

fn main() {
    let mut trie = Trie::new();
    trie.build(vec![
        "openai".to_string(),
        "stack overflow".to_string(),
        "prompt engineering".to_string(),
    ]);

    // read from test.html
    let read = std::fs::read_to_string("examples/test.html").unwrap();

    let res: MatchResult = trie.search_replace(&read);

    println!("{:?}", res.match_words);

    // save res.text to test_modify.html
    std::fs::write("examples/test_modify.html", res.modified_html).unwrap();
}
