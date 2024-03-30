use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Parser)]
struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,
    /// Should number lines
    #[arg(
        short('n'),
        long("number"),
        conflicts_with("number_nonblank_lines")
    )]
    number_lines: bool,
    /// Should number non-blank lines
    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,
}

impl Args {
    fn get_line_number_option(&self) -> LineNumberOption {
        if self.number_lines {
            return LineNumberOption::NumberAll;
        } else if self.number_nonblank_lines {
            return LineNumberOption::NumberOnlyNonBlank;
        } else {
            return LineNumberOption::NumberNone;
        }
    }
}

enum LineNumberOption {
    NumberAll,
    NumberOnlyNonBlank,
    NumberNone,
}

fn main() {
    run().unwrap();
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let line_number_option: LineNumberOption = args.get_line_number_option();
    let input_files = read_input_files(args);
    for file in input_files {
        print_file(file, &line_number_option);
    }

    Ok(())
}

fn print_file(lines: Vec<String>, line_number_option: &LineNumberOption) {
    let mut previous_line_number: u8 = 0;
    for line in lines {
        match line_number_option {
            LineNumberOption::NumberAll => {
                let current_line_number = previous_line_number + 1;
                println!("{}\t{}", current_line_number, line);
                previous_line_number = current_line_number;
            }
            LineNumberOption::NumberOnlyNonBlank => {
                if line.is_empty() {
                    println!("{}", line);
                } else {
                    let current_line_number = previous_line_number + 1;
                    println!("{}\t{}", current_line_number, line);
                    previous_line_number = current_line_number;
                }
            }
            LineNumberOption::NumberNone => {
                println!("{}", line);
            }
        }
    }
}

fn read_input_files(args: Args) -> Vec<Vec<String>> {
    let mut output_vec: Vec<Vec<String>> = vec![];
    for file in args.files {
        if let Ok(buf_reader) = open_file(&file) {
            let mut vector_for_file: Vec<String> = vec![];
            let lines = buf_reader.lines();
            for line in lines {
                match line {
                    Ok(line) => vector_for_file.push(line),
                    Err(error) => eprintln!("{}", error),
                }
            }
            output_vec.push(vector_for_file);
        }
    }

    output_vec
}

fn open_file(filename: &str) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
   match filename {
       "-" => Ok(Box::new(BufReader::new(io::stdin()))),
       _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
   } 
}
