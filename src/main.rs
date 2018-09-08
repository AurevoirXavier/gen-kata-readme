extern crate regex;
extern crate reqwest;
extern crate serde_json;
extern crate select;

mod kata;

fn main() {
    use std::env::args;

    use kata::Kata;

    let args: Vec<_> = args().collect();
    let mut kyu = match args.len() {
        1 => panic!("please specify address"),
        2 => Kata::new(&args[1]),
        _ => panic!(format!("invalid arguments {}", args[3]))
    };

    kyu.war_time();
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//
//}
