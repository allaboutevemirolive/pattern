use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, Copy)]
enum OracleValue {
    Undefined,
    State(usize),
}

struct FactorOracle<T: Eq + Hash + Clone> {
    transitions: HashMap<(usize, T), OracleValue>,
}

impl<T: Eq + Hash + Clone> FactorOracle<T> {
    fn new(pattern: &[T]) -> Self {
        let mut transitions = HashMap::new();

        let m = pattern.len();

        for state in 0..=m {
            for a in pattern.iter().cloned() {
                let q = if state == m {
                    OracleValue::Undefined
                } else {
                    OracleValue::State(state)
                };
                transitions.insert((state, a), q);
            }
        }

        FactorOracle { transitions }
    }

    fn transition(&self, state: usize, character: T) -> OracleValue {
        *self.transitions.get(&(state, character)).unwrap_or(&OracleValue::Undefined)
    }
}

fn main() {
    // Example usage:
    let pattern = "abab";
    let factor_oracle = FactorOracle::new(&pattern.chars().collect::<Vec<_>>());

    // Test transitions
    for state in 0..=pattern.len() {
        for character in pattern.chars() {
            let transition = factor_oracle.transition(state, character);
            println!("δ({}, '{}') = {:?}", state, character, transition);
        }
    }
}
