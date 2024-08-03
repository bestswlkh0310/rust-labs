use std::collections::HashMap;
use std::io;
use std::ops::Add;

mod modulea;

fn main() {
    let user = User {
        username: "Hello World"
    };

    let s = String::new();
    let v = [vec![1, 2, 3], vec![1, 23, 3]].concat();
    let s = [String::from("Hello, "), String::from("World")].concat();
    println!("{s}");

    let s = "Wow".to_string();
    let s = String::from("Hello, ").add(&s[..]); // equals '+' operator

    println!("{}", s);

    let s = "HEllo".to_string();
    // println!("{}", s[   0]); // Error

    println!("{}", user.username);
    let wow = modulea::submodule::Wow;

    let s = "H";
    a(s);

    println!("{s}");

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{s}"); // Зд
    // let s = &hello[0..1]; // Error

    let s = "안녕하세요";
    println!("{}", &s[0..3]); // 안

    println!("----");
    for c in "Здравствуйте".chars() {
        println!("{c}");
    }

    println!("----");
    for c in "नमस्ते".chars() {
        println!("{c}");
    }

    println!("----");
    for byte in "Зд".bytes() {
        println!("{byte}");
    }

    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("A"), 32);
    scores.insert(String::from("B"), 19);

    let mut s = String::new();
    if let Err(why) = io::stdin().read_line(&mut s) {
        println!("{}", why);
        return;
    };

    println!("{}", s);



    println!("{:#?}", scores);

    let s = 10;
    let s = Some(&s).copied().unwrap();
    println!("{}", s);
}

fn a(s: &str) {}

struct User<'a> {
    username: &'a str,
}