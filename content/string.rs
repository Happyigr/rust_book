fn main() {
    let data = "initial data";

    let s = data.to_string();
    let s = "data".to_string();
    let mut s = String::from("some");
    let s2 = String::from(" data");
    s.push_str(&s2);
    // "push" push a char into a string
    s.push('.');
    let s1 = String::from("Hello ");
    let s2 = String::from("World!");
    // s1 is now not useable, because of the ownership
    let s3 = s1 + &s2;

    let s1 = "data".to_string();
    let s2 = " is not ".to_string();
    let s3 = "helpful".to_string();
    // the same as the concatinate operator but more readeble
    // and this doesnt take the ownership of any of the pararmeters
    let s3 = format!("{s1}-{s2}-{s3}");

    // the index gets the bytes with these numbers not the chars, because the utf-8 stores the
    // string not always with constant amount of bytes
    let ch = s3[0..4];

    // so can we iterate throw the chars
    for c in s3.chars() {
        println!("{c}");
    }

    // so can we iterate throw the bytes
    for b in s3.bytes() {
        println!("{b}");
    }
}
