use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = parse_config(&args);

    println!("In file {}", filename);

    // let contents = fs::read_to_string(filename).expect("Error while reading file");
    let contents = fs::read(filename).expect("Error while reading file");
    let bytes = contents.len();
    let mut words = 0;
    let mut lines = 0;

    for byte in contents.iter() {
        if *byte == b' ' {
            words += 1;
        }
        if *byte == b'\n' {
            lines += 1;
            words += 1;
        }
    }

    println!("{} {} {} {}", lines, words, bytes, filename);
}

fn parse_config(args: &[String]) -> &str {
    let filename = &args[1];

    filename
}
