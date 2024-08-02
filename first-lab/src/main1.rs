mod main2;
mod main;

fn main() {
    println!("주소값 같음 애초에 &str은 참조이기 때문");
    let s1 = "Hello";
    let s2 = s1;
    println!("{:p} {:p}", s1, s2);

    println!("주소값 같음");
    let s1 = String::from("Hello");
    println!("{:?}", &s1 as *const String);
    let s2 = &s1;
    println!("{:?}", s2 as *const String);

    println!("주소값 다름 - clone 했기 때문ㅇ");
    let s1 = String::from("Hello");
    println!("{:?}", &s1 as *const String);
    let s2 = s1.clone();
    println!("{:?}", &s2 as *const String);

    let i = 10;
    take_owner(i);
    println!("{i}");

    let s = String::new();
    take_str_owner(s);
    // println!("{s}"); // ERROR 소유권이 task_str_owner 함수로 move됨

    let s = give_owner();
    println!("{}", s);

    let s = t_n_g(s);
    println!("{}", s);

    let l = len(&s);

    println!("{} {}", s, l);

    // 가변 참조자(&mut)는 같은 스코프 내에서 한번만 사용 가능
    let mut s = s;
    wow(&mut s);

    println!("{s}");

    let mut s1 = s;
    wow(&mut s1);


    let mut value = 32;
    let _r = &value;
    let _r1 = &mut value;
    let r2 = &mut value;
    // println!("{}{}", r1, r2); // ERROR
    println!("{}", r2);

    let mut block = Block {
        id: 0,
        name: String::from("block"),
    };
    block.id = 1;

    let mut b = Block {
        id: 1,
        name: String::from("b"),
    };

    let mut b2 = Block {
        id: 2,
        ..b
    };

    println!("{}", block.id);

    b2.id = 10;
    println!("{}", b2.id);

    let mut black = Color(0, 0, 0);
    let _white = Color(255, 255, 255);
    black.0 = 100;
    println!("{:#?}", black);

    let nm = block.get_name();
    println!("{}", nm);

    println!("{}", block.wow_name(&b2));

    let v4 = Ip::V4;
    if let Ip::V4 = v4 {
        println!("ㅇㅇ");
    }

    match v4 {
        Ip::V4 => println!("ㅇㅇㅇㅇ"),
        _ => println!("ㄴㄴ")
    };
}

fn take_owner(_a: i32) {}

fn take_str_owner(_str: String) {}

fn give_owner() -> String {
    let s = String::from("wow");
    s
}

fn t_n_g(s: String) -> String {
    s
}

fn len(s: &String) -> usize {
    s.len()
}

fn wow(s: &mut String) {
    s.push('a');
}

// fn dangle() -> &String {
//     let s = String::new();
//     &s
// }

struct Block {
    id: i32,
    name: String,
}

impl Block {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn wow_name(&self, other: &Block) -> String {
        let mut s = self.name.clone();
        s.push_str(&other.name);
        s.clone()
    }
}

#[derive(Debug)]
struct Color(i32, i32, i32);

enum Ip {
    V4,
    V6
}