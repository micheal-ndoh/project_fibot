fn extract_numbers_from_pr(content: &str) -> Vec<u32> {
    content.replace("-",  " ")
        .split_whitespace()
        .map(|word| word.chars().filter(|c| !c.is_ascii_punctuation()).collect::<String>())
        .filter_map(|word| word.parse().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_numbers() {
        let content = "123\n, 456, 789";
        let expected = vec![123, 456, 789];
        let result = extract_numbers_from_pr(content);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_numbers_with_punctuation() {
        let content = "123,\n ab- c! 456-789?";
        let expected = vec![123, 456, 789];
        let result = extract_numbers_from_pr(content);
        assert_eq!(result, expected);
    }
}