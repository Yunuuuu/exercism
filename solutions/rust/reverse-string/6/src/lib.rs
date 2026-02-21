extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
}

#[cfg(test)]
mod test {
    use super::reverse;
    #[test]
    fn reverse_works() {
        assert_eq!(reverse("stressed"), "desserts");
        assert_eq!(reverse("strops"), "sports");
        assert_eq!(reverse("racecar"), "racecar");
    }
}
