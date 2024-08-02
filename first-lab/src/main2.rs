fn main() {
    let s = String::from("hello world");
    let hello = &s[..5];
    let world = &s[6..];
    println!("{hello}");
    println!("{world}");

    let world = first_word(&s);
    println!("{world}");

    let s = "hello world";

    let world = first_word(&s);
    println!("{world}");

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (idx, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..idx];
        }
    }
    &s[..]
}