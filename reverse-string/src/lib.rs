pub fn reverse(input: &str) -> String {
    // use unicode_reverse::reverse_grapheme_clusters_in_place;
    // let mut s = String::from(input);
    // reverse_grapheme_clusters_in_place(&mut s);
    // String::from(s)
    input.rsplit("").collect()
}
