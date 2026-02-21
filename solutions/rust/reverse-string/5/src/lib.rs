pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

mod test {
    use super::reverse;
    #[test]
    fn reverse_works() {
        assert_eq!(reverse("stressed"), "desserts");
        assert_eq!(reverse("strops"), "sports");
        assert_eq!(reverse("racecar"), "racecar");
    }
}
