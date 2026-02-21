pub fn find<T: Ord, V: AsRef<[T]>>(x: V, item: T) -> Option<usize> {
    let x = x.as_ref();
    let mut low = 0;
    let mut high = x.len();
    while low < high {
        let mid = (low + high) / 2;
        if x[mid] > item {
            high = mid;
        } else if x[mid] < item {
            low = mid + 1;
        } else {
            return Some(mid);
        }
    }
    None
}
