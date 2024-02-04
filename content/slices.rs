fn main() {
    let s = String::from("Just a sentance.");

    let word = first_word(&s);

    println!("{}", word);
}
// with &String we can only pass the String in the function,
// but with &str we can pass slices in the function, and slices can be made of all types of strings
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
