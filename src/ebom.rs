#[derive(Debug)]
struct TrieNode<T> {
    character: Option<T>,
    transitions: Vec<TrieNode<T>>,
}

impl<T> TrieNode<T> {
    fn is_final(&self) -> bool {
        self.transitions.is_empty()
    }
}

struct PatternAndTrie<T> {
    _pattern: Vec<T>,
    trie: TrieNode<T>,
}

fn build_trie<T: Clone>(pattern: &[T]) -> TrieNode<T> {
    let mut root_node = TrieNode {
        character: None,
        transitions: Vec::new(),
    };
    let mut current_node = &mut root_node;

    for ch in pattern {
        let new_node = TrieNode {
            character: Some(ch.clone()),
            transitions: Vec::new(),
        };

        current_node.transitions.push(new_node);
        current_node = current_node.transitions.last_mut().unwrap();
    }

    root_node
}

pub fn extended_bom<T: PartialEq + Clone>(pattern: &[T], text: &[T]) -> Vec<usize> {
    
    // FIXME
    // Implement precomputed factor-oracle.
    // This is naive implementation.
    let mut pattern_and_trie = PatternAndTrie {
        _pattern: pattern.to_owned(),
        trie: build_trie(pattern),
    };

    let mut current_position = 0;
    let mut current_state = &mut pattern_and_trie.trie;
    let mut match_position = Vec::new();

    if pattern.is_empty() {
        return match_position;
    }

    while let Some(c) = text.get(current_position) {
        // Fast-loop
        for index in 0..current_state.transitions.len() {
            if current_state.transitions[index].character == Some(c.clone()) {
                current_state = &mut current_state.transitions[index];
                break;
            }
        }

        if current_state.is_final() {
            match_position.push(current_position);
            current_position += pattern.len();
            current_state = &mut pattern_and_trie.trie;
        } else {
            current_position += 1;
            current_state = &mut pattern_and_trie.trie;
        }
    }

    match_position
}

pub fn print_match_positions(match_positions: &[usize]) {
    println!("Match positions:");
    for (i, position) in match_positions.iter().enumerate() {
        println!("Match {}: {}", i + 1, position);
    }
}
