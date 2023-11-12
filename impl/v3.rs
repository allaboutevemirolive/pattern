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


    // FIXME
    // Investigate this purpose
    // ------------------

    // FIXNE
    // Intent for type declaration?
    // let mut current_node = &mut root_node;
    let current_node = &mut root_node;

    // FIXME
    // Does this intent for multiple pattern search?
    for &ch in pattern {
        // Extract pattern into TrieNode
        let new_node = TrieNode {
            character: ch,
            transitions: Vec::new(),
        };

        // FIXME

        // Update extracted pattern inside 'current_node'
        // where it has a longer lifetime
        current_node.transitions.push(new_node);

        // FIXME
        // Currently ommitting the code below wont produce error.

        // Address the short lifetime error
        // let bind = &mut current_node.transitions;
        // current_node = bind.last_mut().unwrap();

        // println!("current_node: {:?}", current_node);
    }

    // FIXME
    // Redundant??
    // We already clone 'root_node' inside 'current_node'
    // Then we update the current_node,

    // println!("root_node: {:?}", root_node);
    

    // Currently ommited snippet below won't produce error
    // let mut current_node = &mut root_node;



    // ------------------

    // FIXME
    // Currently ommiting the code below wont produce error
    // Probably to handle multiple pat

    // for &ch in pattern.iter().rev().skip(1) {
    //     let index = (ch as usize - 'a' as usize) % current_node.transitions.len();
    //     println!("index: {}", index);
    //     if current_node.transitions.is_empty() {
    //         current_node.transitions.push(TrieNode {
    //             character: ch,
    //             transitions: Vec::new(),
    //         });
    //     }
    //     current_node.transitions[index] = current_node.transitions[index]
    //         .transitions
    //         .pop()
    //         .unwrap_or_else(|| TrieNode {
    //             character: ch,
    //             transitions: Vec::new(),
    //         });

    //     current_node = &mut current_node.transitions[index];
    // }

    root_node
}

// Function to perform the Extended-BOM algorithm
fn extended_bom(pattern: &Vec<char>, text: &Vec<char>) -> Vec<usize> {
    // FIXME
    // Investigate why we need double pattern proceesing
    // Build the trie from the pattern
    let mut pattern_and_trie = PatternAndTrie {
        pattern: pattern.to_owned(),
        trie: build_trie(pattern),
    };

    // TODO
    // Initialize this in struct

    // Initialize variables for the current position in the text and the current state of the automaton
    let mut current_position = 0;
    let mut current_state = &mut pattern_and_trie.trie;

    println!("current_state: {:?}", current_state);

    // Our result
    let mut match_position = Vec::new();

    // If pattern is empty, return default
    if pattern.is_empty() {
        return match_position;
    }

    // Iterate through the text, updating the current position and state as needed
    while let Some(&c) = text.get(current_position) {


        println!("While loop");

        // Perform the fast-loop
        for index in 0..current_state.transitions.len() {
            if current_state.transitions[index].character == c {
                current_state = &mut current_state.transitions[index];
                println!("BREAK");
                break;
            }
        }

        // Check if a match is found
        if current_state.is_final() {


            println!("EMPTY");

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

// FIXME
// Rewrite so this algo can handle various types
fn main() {
    let text = vec!['a', 'b', 'c', 'a', 'b', 'c', 'a'];
    let pattern = vec!['c'];

    let match_positions = extended_bom(&pattern, &text);
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

        // Add more test cases as needed
    }

    #[test]
    fn no_pattern() {
        // Test case 3 (Edge case: empty pattern)
        let text_3 = vec!['a', 'b', 'c', 'a', 'b', 'c', 'a'];
        let pattern_3 = vec![];
        let match_positions_3 = extended_bom(&pattern_3, &text_3);
        assert_eq!(match_positions_3, Vec::<usize>::new());
    }
}
