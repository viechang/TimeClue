#[macro_use]
extern crate lazy_static;

fn say_what(name: &str, func: fn(&str)) {
    func(name);
}

fn test(string: &str) {
    println!("{}", string);
}

fn foo(_: &str) {
    println!("foobar.");
}

lazy_static!{
    static ref VEC: Vec<(&'static str, fn(&str))> = vec![
        ("Hello, Rust!", test),
        ("null", foo)
    ];
}

fn main() {
//    let vec: Vec<(&str, fn(&str))> = vec![
//        ("Hello, Rust!", test),
//        ("null", foo)
//    ];
    for i in VEC.iter() {
        say_what(i.0, i.1);
    }
}