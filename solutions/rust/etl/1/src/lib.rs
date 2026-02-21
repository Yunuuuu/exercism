use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut out: BTreeMap<char, i32> = BTreeMap::new();

    for (key, value) in h {
        value.into_iter().copied().for_each(|k| {
            out.insert(k.to_ascii_lowercase(), key.clone());
        })
    }
    out
}
