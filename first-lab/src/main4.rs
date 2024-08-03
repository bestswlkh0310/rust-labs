use std::collections::HashMap;
use std::io;

fn main() {
    let mut s = String::new();
    if let Err(why) = io::stdin().read_line(&mut s) {
        println!("{}", why);
        return;
    };

    for word in s.split_whitespace() {
        println!("{}", word);
    }

    for word in s.split(char::is_whitespace) {
        if !word.is_empty() {
            println!("{}", word);
        }
    }

    let mut hash_map = HashMap::new();

    for word in s.split_whitespace() {
        let value = hash_map.entry(word).or_insert(10);
        *value *= 10;
    }

    println!("{:?}", hash_map);
}