use std::collections::HashMap;

fn main() {
    // let needle = "abcababcabcabc";
    let needle = "ab";
    let haystack = "abcababcabcabcabcab";

    let result = extended_bom(needle, haystack);
    println!("Result: {:?}", result);
}

fn precompute_factor_oracle(needle: &str) -> HashMap<(usize, char), usize> {
    let mut oracle = HashMap::new();
    let m = needle.len();

    for j in 0..m {
        for c in needle.chars() {
            oracle.insert((j, c), 0);
        }
    }

    for j in (1..m).rev() {
        let suffix = &needle[j..];
        for c in needle.chars() {
            let mut i = 0;
            for (idx, ch) in suffix.chars().enumerate() {
                if c == ch {
                    i = idx + 1;
                }
            }
            oracle.insert((j - 1, c), i);
        }
    }

    oracle
}

fn extended_bom(needle: &str, haystack: &str) -> Vec<usize> {
    let mut i = 0;
    let mut j = 0;
    let mut result = Vec::new();
    let n = haystack.len();
    let m = needle.len();

    let factor_oracle = precompute_factor_oracle(needle);

    println!("factor_oracle: {:?}", factor_oracle);

    // while i <= n - m {
    //     while j < m && needle.chars().rev().nth(j) == haystack.chars().nth(i + j) {
    //         j += 1;
    //     }

    //     if j == m {
    //         result.push(i);
    //     }

    //     let max_j = if m <= n - i { n - i - m } else { 0 };

    //     // FIXME 
    //     // Infinite loop

    //     println!("max_j: {}", max_j);
    //     println!("j: {}", j);

    //     // FIXME
    //     // While loop never execute
    //     while j < max_j {

    //         println!("----");
    //         let _suffix = &haystack[i + j + 1..];
    //         let c = haystack.chars().nth(i + j).unwrap();

    //         if let Some(&oracle_value) = factor_oracle.get(&(j, c)) {
    //             // i = i + j - oracle_value;
    //             i = i + 1;
    //             j = oracle_value;
    //             break;
    //         }

    //         j += 1;
    //     }
    // }

    result
}
