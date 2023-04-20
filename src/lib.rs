use once_cell::sync::Lazy;
use regex::Regex;

static NICE_MATCHER: Lazy<Regex> = Lazy::new(|| {
    const PATTERN: &str = r"(?i)6.9|69|(sixty(\s+|-)nine)|(soixante(\s+|-)neuf)|LXIX|ⅬⅩⅨ|ⅼⅹⅸ|‘’|“”|６９|六十九|ξθʹ|⑥⑨|⑹⑼|⓺⓽|🕕🕘|6️⃣9️|1000101";
    Regex::new(PATTERN).unwrap()
});

/// Returns true if the given string is nice. A string is nice if it has a substring in which its
/// numerical form is equivalent to 69.
pub fn is_nice(s: &str) -> bool {
    let number: f64 = s.parse().unwrap_or(f64::NAN);
    if number == 69.0 {
        return true;
    }

    NICE_MATCHER.find(s).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("", false)]
    #[case("not nice", false)]
    #[case("68", false)]
    #[case("69", true)]
    #[case("6.9", true)]
    #[case("6.91", true)]
    #[case("16.91", true)]
    #[case("96", false)]
    #[case("9696", true)]
    #[case("6.9e1", true)]
    #[case("sixty nine", true)]
    #[case("sixty    nine", true)]
    #[case("sixty-nine", true)]
    #[case("SIXTY-NINE", true)]
    #[case("soixante-neuf", true)]
    #[case("LXIX", true)]
    #[case("ⅬⅩⅨ", true)]
    #[case("ⅼⅹⅸ", true)]
    #[case("rustc 1.69.0", true)]
    #[case("‘’", true)]
    #[case("“”", true)]
    #[case("６９", true)]
    #[case("⑥⑨", true)]
    #[case("⑹⑼", true)]
    #[case("⓺⓽", true)]
    #[case("🕕🕘", true)]
    #[case("6️⃣9️", true)]
    fn test_is_nice(#[case] s: &str, #[case] expected: bool) {
        assert_eq!(is_nice(s), expected, "{}", s);
    }
}
