#[cfg(feature = "grapheme")]
use unicode_segmentation::UnicodeSegmentation;

#[cfg(feature = "grapheme")]
pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
}

#[cfg(not(feature = "grapheme"))]
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
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

    #[test]
    #[cfg(feature = "grapheme")]
    fn reverse_grapheme_works() {
        assert_eq!(reverse("uüu"), "uüu");
    }
}
