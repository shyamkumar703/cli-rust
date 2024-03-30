use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

#[derive(Parser, Debug)]
struct Config {
    #[arg(default_value = "-", value_name = "FILE")]
    files: Vec<String>,

    /// Number of lines
    #[arg(
        short('n'),
        long,
        default_value = "10",
        value_name = "LINES",
        value_parser =
        clap::value_parser!(u64).range(1..)
        )]
        lines: u64,

        /// Number of bytes
        #[arg(
            short('c'),
            long,
            value_name
            =
            "BYTES",
            conflicts_with("lines"),
            value_parser
            =
            clap::value_parser!(u64).range(1..)
            )]
            bytes:
            Option<u64>,
}


fn main() {
    let config = Config::parse();
    run(&config);
}

fn run(config: &Config) {
    for file in &config.files {
        match open(file) {
            Ok(bufreader) => {
                println!("==> {} <==", file);
                print_file(bufreader, config);
            },
            Err(error) => eprintln!("{}", error),
        }
    }
}

fn print_file(bufreader: Box<dyn BufRead>, config: &Config) {
    if let Some(num_bytes) = config.bytes {
        print_bytes(bufreader, num_bytes);
    } else {
        print_lines(bufreader, config.lines);
    }
    println!();
}

fn print_bytes(bufreader: Box<dyn BufRead>, num_bytes: u64) {
    let mut bytes: Vec<u8> = vec![];
    for byte in bufreader.bytes().filter_map(|byte| byte.ok()) {
        bytes.push(byte);
        if bytes.len() == num_bytes.try_into().unwrap() {
            break;
        }
    }

    let display_string = String::from_utf8_lossy(&bytes);
    println!("{}", display_string);
}

fn print_lines(bufreader: Box<dyn BufRead>, num_lines: u64) {
    let mut printed_lines = 0;
    for line in bufreader.lines().filter_map(|line| line.ok()) {
        if printed_lines == num_lines {
            return;
        }
        println!("{}", line);
        printed_lines += 1;
    }
}

fn open(file: &str) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match file {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file)?))),
    }
}

pub fn parse_into_usize(string: &str) -> Result<usize, Box<dyn Error>> {
   let parsed_usize: usize = string.parse()?; 
   Ok(parsed_usize)
}

#[cfg(test)]
mod tests {
    use crate::parse_into_usize;

    #[test]
    fn test_parse_positive_int() {
        let string = String::from("5");
        parse_into_usize(&string).unwrap();
    }

    #[test]
    fn test_parse_negative_int() {
        let string = String::from("-5");
        // -5 is an invalid usize
        parse_into_usize(&string).unwrap_err();
    }

    #[test]
    fn test_parse_basic_string() {
        let string = String::from("hello, world!");
        parse_into_usize(&string).unwrap_err();
    }

}
