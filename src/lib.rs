pub fn is_nice(s: &str) -> bool {
    if s.contains("69") {
        return true
    }

    return false
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("", false)]
    #[case("69", true)]
    fn test_is_nice(#[case] s: &str, #[case] expected: bool) {
        assert_eq!(is_nice(s), expected);
    }
}