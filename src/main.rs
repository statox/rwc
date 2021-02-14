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
    let mut prev_was_space = false;
    let mut prev_was_newline = false;

    for byte in contents.iter() {
        if *byte == b' ' {
            if !prev_was_space && !prev_was_newline {
                words += 1;
            }
            prev_was_space = true;
        } else if *byte == b'\n' {
            if !prev_was_space && !prev_was_newline {
                words += 1;
            }
            lines += 1;
            prev_was_newline = true;
        } else {
            prev_was_space = false;
            prev_was_newline = false;
        }
    }

    println!("{} {} {} {}", lines, words, bytes, filename);
}

fn parse_config(args: &[String]) -> &str {
    let filename = &args[1];

    filename
}
