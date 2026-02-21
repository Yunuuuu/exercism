pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

mod test_grapheme_clusters {
    use super::*;
    #[test]
    fn reverse_workds() {
        assert_eq!(reverse("stressed"), "desserts");
        assert_eq!(reverse("strops"), "sports");
        assert_eq!(reverse("racecar"), "racecar");
    }
}