use std::fs::File;

fn main() {
    // panic_test();
    // panic_index_out_of_bounds();

    let file = match File::open("hello.txt") {
        Ok(v) => v,
        Err(why) => {
            println!("Error - {}", why);
            return;
        }
    };
    println!("{:?}", file);
}

fn panic_test() {
    panic!("Panic test");
}

fn panic_index_out_of_bounds() {
    let v = vec![1, 2, 3];
    println!("{}", &v[100]);
}