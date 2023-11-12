#[cfg(test)]
mod tests {
    use crate::ebom::extended_bom;

    #[test]
    fn ebom_1() {
        let text_1 = vec!['a', 'b', 'c', 'a', 'b', 'c', 'a'];
        let pattern_1 = vec!['c'];
        let match_positions_1 = extended_bom(&pattern_1, &text_1);
        assert_eq!(match_positions_1, vec![2, 5]);
    }

    #[test]
    fn ebom_2() {
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

        // FIXME
        let text_chars: Vec<char> = text.chars().collect();
        let pattern_chars: Vec<char> = pattern.chars().collect();

        let match_positions = extended_bom(&pattern_chars, &text_chars);
        let expected_positions: Vec<usize> = vec![2, 5, 8];
        assert_eq!(match_positions, expected_positions);
    }

    #[test]
    fn ebom_spacing() {
        let text = "abca  bcabc";
        let pattern = "a";

        // FIXME
        let text_chars: Vec<char> = text.chars().collect();
        let pattern_chars: Vec<char> = pattern.chars().collect();

        let match_positions = extended_bom(&pattern_chars, &text_chars);
        let expected_positions: Vec<usize> = vec![0, 3, 8];
        assert_eq!(match_positions, expected_positions);
    }
}
