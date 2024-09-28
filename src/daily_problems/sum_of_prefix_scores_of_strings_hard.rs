use std::collections::HashMap;

struct TrieNode {
    count: i32,
    children: HashMap<char, Box<TrieNode>>,
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            count: 0,
            children: HashMap::new(),
        }
    }
}

struct Solution;

impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let mut root = TrieNode::new();

        for word in &words {
            let mut node = &mut root;
            for ch in word.chars() {
                node = node.children.entry(ch).or_insert(Box::new(TrieNode::new()));
                node.count += 1;
            }
        }

        words
            .iter()
            .map(|word| {
                let mut node = &root;
                let mut score = 0;
                for ch in word.chars() {
                    if let Some(child) = node.children.get(&ch) {
                        score += child.count;
                        node = child;
                    } else {
                        break;
                    }
                }
                score
            })
            .collect()
    }
}
