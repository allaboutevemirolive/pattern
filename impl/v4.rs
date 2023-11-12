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

fn extended_bom<T: PartialEq + Clone>(pattern: &[T], text: &[T]) -> Vec<usize> {
    let mut pattern_and_trie = PatternAndTrie {
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

fn print_match_positions(match_positions: &[usize]) {
    println!("Match positions:");
    for (i, position) in match_positions.iter().enumerate() {
        println!("Match {}: {}", i + 1, position);
    }
}

fn main() {
    let text = "abcabcabc";
    let pattern = "c";

    let text_chars: Vec<char> = text.chars().collect();
    let pattern_chars: Vec<char> = pattern.chars().collect();

    let match_positions = extended_bom(&pattern_chars, &text_chars);

    println!("Text: {}", text);
    println!("Pattern: {}", pattern);
    print_match_positions(&match_positions);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extended_bom() {
        // Test case 1
        let text_1 = vec!['a', 'b', 'c', 'a', 'b', 'c', 'a'];
        let pattern_1 = vec!['c'];
        let match_positions_1 = extended_bom(&pattern_1, &text_1);
        assert_eq!(match_positions_1, vec![2, 5]);

        // Test case 2
        let text_2 = vec!['a', 'b', 'c', 'a', 'b', 'c', 'a'];
        let pattern_2 = vec!['a'];
        let match_positions_2 = extended_bom(&pattern_2, &text_2);
        assert_eq!(match_positions_2, vec![0, 3, 6]);
    }

    #[test]
    fn empty_pattern() {
        let text_3 = vec!['a', 'b', 'c', 'a', 'b', 'c', 'a'];
        let pattern_3 = vec![];
        let match_positions_3 = extended_bom(&pattern_3, &text_3);
        assert_eq!(match_positions_3, Vec::<usize>::new());
    }

    #[test]
    fn pat_string() {
        let text = "abcabcabc";
        let pattern = "c";

        let text_chars: Vec<char> = text.chars().collect();
        let pattern_chars: Vec<char> = pattern.chars().collect();

        let match_positions = extended_bom(&pattern_chars, &text_chars);

        let expected_positions: Vec<usize> = vec![2, 5, 8];

        assert_eq!(match_positions, expected_positions);
    }
}
