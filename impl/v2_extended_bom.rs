// Define a struct for the trie node
#[derive(Debug)]
struct TrieNode {
    character: char,
    transitions: Vec<TrieNode>,
}

impl TrieNode {
    // Check if the current node is a final state
    fn is_final(&self) -> bool {
        self.transitions.is_empty()
    }
}

// Define a struct for the pattern and its trie
struct PatternAndTrie {
    pattern: Vec<char>,
    trie: TrieNode,
}

// Function to build the trie from the pattern
fn build_trie(pattern: &Vec<char>) -> TrieNode {
    let mut root_node = TrieNode {
        character: '\0',
        transitions: Vec::new(),
    };

    let mut current_node = &mut root_node;

    for &ch in pattern {
        let new_node = TrieNode {
            character: ch,
            transitions: Vec::new(),
        };

        current_node.transitions.push(new_node);
        let bind = &mut current_node.transitions;
        current_node = bind.last_mut().unwrap();
    }

    let mut current_node = &mut root_node;
    for &ch in pattern.iter().rev().skip(1) {
        let index = (ch as usize - 'a' as usize) % current_node.transitions.len();
        if current_node.transitions.is_empty() {
            current_node.transitions.push(TrieNode {
                character: ch,
                transitions: Vec::new(),
            });
        }
        current_node.transitions[index] = current_node.transitions[index]
            .transitions
            .pop()
            .unwrap_or_else(|| TrieNode {
                character: ch,
                transitions: Vec::new(),
            });

        current_node = &mut current_node.transitions[index];
    }

    root_node
}

// Function to perform the Extended-BOM algorithm
fn extended_bom(pattern: &Vec<char>, text: &Vec<char>) -> Vec<usize> {
    // Build the trie from the pattern
    let mut pattern_and_trie = PatternAndTrie {
        pattern: pattern.to_owned(),
        trie: build_trie(pattern),
    };

    // Initialize variables for the current position in the text and the current state of the automaton
    let mut current_position = 0;
    let mut current_state = &mut pattern_and_trie.trie;
    let mut match_position = Vec::new();

    // Iterate through the text, updating the current position and state as needed
    while let Some(&c) = text.get(current_position) {
        // Perform the fast-loop
        for index in 0..current_state.transitions.len() {
            if current_state.transitions[index].character == c {
                current_state = &mut current_state.transitions[index];
                break;
            }
        }

        // Check if a match is found
        if current_state.is_final() {
            match_position.push(current_position);
            // Move the position after the matched pattern
            current_position += pattern.len();
            // Reset to the root of the trie for the next search
            current_state = &mut pattern_and_trie.trie;
        } else {
            // If no transition is found in the fast loop, move one position forward
            current_position += 1;
            // Reset to the root of the trie if there is no transition for the current character
            current_state = &mut pattern_and_trie.trie;
        }
    }

    match_position
}

// Function to print the match positions
fn print_match_positions(match_positions: &Vec<usize>) {
    println!("Match positions:");
    for (i, position) in match_positions.iter().enumerate() {
        println!("Match {}: {}", i + 1, position);
    }
}

fn main() {
    let text = vec!['a', 'b', 'c', 'a', 'b', 'c', 'a'];
    let pattern = vec!['c'];

    let match_positions = extended_bom(&pattern, &text);
    print_match_positions(&match_positions);
}
