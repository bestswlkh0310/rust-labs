fn main() {
    let user = User {
        username: "Hello World"
    };

    println!("{}", user.username);
}

struct User<'a> {
    username: &'a str
}