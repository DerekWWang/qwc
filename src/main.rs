use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::env;

fn file_byte_count(file_name: &String) -> usize {
    let bytes: Vec<u8> = fs::read(file_name).expect("Error Reason");
    bytes.len()
}

fn file_line_count(file_name: &String) -> usize {
    let path = Path::new(file_name);
    let file = fs::File::open(path).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut lc: usize = 0;
    for lr in reader.lines() {
        lc += 1;
    }
    lc
}

fn file_word_count(file_name: &String) -> usize {
    let path = Path::new(file_name);
    let file = fs::File::open(path).expect("Could not open file");
    let reader = BufReader::new(file);

    let mut wc: usize = 0;

    for lr in reader.lines() {
        let line = lr.expect("Could not read line");
        let split: Vec<&str>= line.split_whitespace().collect();
        wc += split.len();
    }

    wc
}

struct Arguments {
    flag: String,
    file_path: String,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Incorrect number of arguments");
        }

        if args.len() == 2 {
            return Ok(Arguments {
                    flag: "d".to_string(),
                    file_path: args[1].clone(),
                }
            )
        } else {
            return Ok(Arguments {
                flag: args[1].clone(),
                file_path: args[2].clone(),
            })
        }
    }
}

fn main() {
    println!("Read {} bytes from the file", file_byte_count(&"/Users/derekzhu/Code/Quant/qwc/src/main.rs".to_string()));

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        eprintln!("Usage: {} <flag> <file_path>", program);
        std::process::exit(0);
    });

    if arguments.flag == "-c" {
        println!("\t {} {}", file_byte_count(&arguments.file_path), arguments.file_path);
    } else if arguments.flag == "-l" {
        println!("\t {} {}", file_line_count(&arguments.file_path), arguments.file_path);
    } else if arguments.flag == "-w" {
        println!("\t {} {}", file_word_count(&arguments.file_path), arguments.file_path);
    } else if arguments.flag == "d" {
        let fbc = file_byte_count(&arguments.file_path);
        let flc = file_line_count(&arguments.file_path);
        let fwc = file_word_count(&arguments.file_path);
        println!("\t {} {} {} {}", flc, fwc, fbc, arguments.file_path);
    }
}
