extern crate regex;
extern crate reqwest;
extern crate serde_json;
extern crate select;

mod kyu;

fn main() {
    use std::env::args;

    use kyu::Kyu;

    let args: Vec<_> = args().collect();
    let mut kyu = match args.len() {
        1 => panic!("please specify address"),
        2 => Kyu::new(&args[1]),
        _ => panic!(format!("invalid arguments {}", args[3]))
    };

    kyu.war_time();
}
