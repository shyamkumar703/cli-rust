use clap::Parser;
use std::{
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    error::Error,
};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `uniq`
struct Args {
    /// Input file
    #[arg(value_name = "IN_FILE", default_value = "-")]
    in_file: String,

    /// Output file
    #[arg(value_name = "OUT_FILE")]
    out_file: Option<String>,

    /// Show counts
    #[arg(short, long)]
    count: bool,
}

struct LineCount {
    line: String,
    count: usize,
}

fn main() {
    run(Args::parse());
}

fn run(args: Args) {
    match open(&args.in_file) {
        Ok(bufreader) => {
            let line_counts = get_line_counts(bufreader);
            let display_string = display(&args, line_counts);
            match args.out_file {
                Some(file) => {
                   write(&file, display_string); 
                },
                None => {
                    print!("{}", display_string);
                }
            }
        },
        Err(error) => {
            eprintln!("ERROR - {}", error);
        }
    }
}

fn write(file: &str, string: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file)
        .unwrap();
    writeln!(file, "{}", string).unwrap();
}

fn display(args: &Args, line_counts: Vec<LineCount>) -> String {
    let mut display_string = String::from("");
    for line_count in line_counts {
        if args.count {
            let formatted_string = &format!("\t{}\t{}\n", line_count.count, line_count.line);
            display_string.push_str(formatted_string);
        } else {
            let formatted_string = &format!("\t{}", line_count.line);
            display_string.push_str(formatted_string);
        }
    }

    display_string
}

fn get_line_counts(bufreader: Box<dyn BufRead>) -> Vec<LineCount> {
    let mut current_line: Option<String> = None;
    let mut current_line_count: usize = 0;
    let mut line_count_vec: Vec<LineCount> = vec![];
    for line in bufreader.lines() {
        if let Ok(line) = line {
            match current_line {
                Some(ref current_line_ref) => {
                    if &line == current_line_ref {
                        current_line_count += 1;
                    } else {
                        let line_count = LineCount {
                            line: current_line.unwrap(),
                            count: current_line_count,
                        };
                        line_count_vec.push(line_count);
                        current_line = Some(line);
                        current_line_count = 1;
                    }
                },
                None => {
                    current_line = Some(line);
                    assert!(current_line_count == 0);
                    current_line_count = 1;
                },
            }
        }
    }

    if let Some(current_line) = current_line {
        let line_count = LineCount {
            line: current_line,
            count: current_line_count,
        };
        line_count_vec.push(line_count);
    }

    line_count_vec
}

fn open(file: &str) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match file {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file)?))),
    }
}
