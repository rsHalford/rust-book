fn main() {
    let s = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial of whole
    let word1 = first_word(&s[..6]);
    let word2 = first_word(&s[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word3 = first_word(&s);

    let lit = "hello world";

    // `first_word` works on slices of string literals, whether partial of whole
    let word4 = first_word(&lit[..6]);
    let word5 = first_word(&lit[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word6 = first_word(&lit);

    println!("{word1}, {word2}, {word3}, {word4}, {word5}, {word6}.");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
