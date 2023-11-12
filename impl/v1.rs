use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::HashBuilder;
use std::iter::repeat;
use std::iter::repeat_with;
use std::iter::TakeWhile;
use std::iter::Peekable;
use std::iter::FromIterator;
use std::iter::IntoIterator;
use std::ops::Range;
use std::str::Bytes;
use std::str::SplitWhitespace;

type State = usize;
type Transition = (State, char);
type TrieNode = HashMap<char, State>;

struct Trie {
    root: State,
    nodes: HashMap<State, TrieNode>,
}

impl Trie {
    fn new() -> Self {
        Trie { root: 0, nodes: HashMap::new() }
    }

    fn insert(&mut self, state: State, transition: Transition) {
        let mut current_node = self.root;
        for (i, (c, next_state)) in transition.iter().enumerate() {
            if current_node.contains_key(&c) {
                current_node = current_node.get(&c).unwrap();
            } else {
                current_node.insert(c, next_state);
            }
            if i == transition.len() - 1 {
                self.nodes.insert(next_state, TrieNode::new());
            }
        }
    }
}

struct ExtendedBOM {
    trie: Trie,
    pattern: Vec<u8>,
    text: Vec<u8>,
    state: State,
    match_length: usize,
    match_start: usize,
}

impl ExtendedBOM {
    fn new(pattern: &[u8]) -> Self {
        let mut trie = Trie::new();
        let pattern = pattern.iter().copied().collect();
        let mut state = 0;
        let mut transitions = vec![];
        for (i, c) in pattern.iter().enumerate() {
            transitions.push((state, c));
            state = trie.insert(state, (c, i + 1));
        }
        ExtendedBOM {
            trie,
            pattern,
            text: Vec::new(),
            state,
            match_length: 0,
            match_start: 0,
        }
    }

    fn process_text(&mut self, text: &[u8]) -> Vec<(usize, usize)> {
        let mut state = self.state;
        let mut match_length = self.match_length;
        let mut match_start = self.match_start;
        let mut matches = Vec::new();
        let mut text_iter = text.iter().peekable();
        let mut pattern_iter = self.pattern.iter().peekable();

        loop {
            let current_pattern_char = pattern_iter.next();
            let current_text_char = text_iter.next();

            if let Some(pattern_char) = current_pattern_char {
                if let Some(text_char) = current_text_char {
                    if *pattern_char == *text_char {
                        match_length += 1;
                    } else {
                        state = self.trie.root;
                        match_length = 0;
                    }
                } else {
                    state = self.trie.root;
                    match_length = 0;
                }
            } else {
                break;
            }

            if let Some(state) = self.trie.nodes.get(&state) {
                if match_length >= self.pattern.len() {
                    matches.push((match_start, match_length));
                    match_length = 0;
                    match_start = 0;
                }
            } else {
                state = self.trie.root;
                match_length = 0;
            }
        }

        matches
    }
}

fn main() {
    let pattern = b"example";
    let text = b"This is an example of a text.";
    let mut ext_bom = ExtendedBOM::new(pattern);
    let matches = ext_bom.process_text(text);
    println!("Matches: {:?}", matches);
}