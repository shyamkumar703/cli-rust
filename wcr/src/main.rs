use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::error::Error;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Show line count
    #[arg(short, long)]
    lines: bool,

    /// Show word count
    #[arg(short, long)]
    words: bool,

    /// Show byte count
    #[arg(short('c'), long)]
    bytes: bool,

    /// Show character count
    #[arg(short('m'), long,
    conflicts_with("bytes"))]
        chars: bool,
}

struct FileCount {
    words: usize,
    lines: usize,
    bytes: usize,
    characters: usize,
}

fn main() {
   run(&Args::parse()); 
}

fn run(args: &Args) {
    let mut file_count_vec: Vec<FileCount> = vec![];
    for file in &args.files {
        match open(file) {
            Ok(bufreader) => {
                let file_count = get_file_count(bufreader);
                print_file_count(Some(file), &file_count, args);
                file_count_vec.push(file_count);
            },
            Err(error) => eprintln!("{}", error),
        }
    }

    print_file_count(Some("total"), &sum_file_count(file_count_vec), args);
}

fn sum_file_count(file_count_vec: Vec<FileCount>) -> FileCount {
    let mut line_count: usize = 0;
    let mut word_count: usize = 0;
    let mut byte_count: usize = 0;
    let mut char_count: usize = 0;

    for file_count in file_count_vec {
        line_count += file_count.lines;
        word_count += file_count.words;
        byte_count += file_count.bytes;
        char_count += file_count.characters;
    }

    FileCount {
        words: word_count,
        lines: line_count,
        bytes: byte_count,
        characters: char_count
    }
}

fn print_file_count(file: Option<&str>, file_count: &FileCount, args: &Args) {
    // lines, words, bytes/chars
    let mut display_string = String::from("\t");
    if args.lines {
        let line_str: &str = &format!("\t{}", file_count.lines);
        display_string.push_str(line_str);
    }

    if args.words {
        let word_str: &str = &format!("\t{}", file_count.words);
        display_string.push_str(&word_str);
    }

    if args.bytes {
        let byte_str: &str = &format!("\t{}", file_count.bytes);
        display_string.push_str(&byte_str);
    } else if args.chars {
        let char_str: &str = &format!("\t{}", file_count.characters);
        display_string.push_str(&char_str);
    }

    if let Some(file) = file {
        display_string.push_str("\t");
        display_string.push_str(file);
    }

    println!("{}", display_string);
}

fn get_file_count(bufreader: Box<dyn BufRead>) -> FileCount {
    let mut line_count: usize = 0;
    let mut word_count: usize = 0;
    let mut byte_count: usize = 0;
    let mut char_count: usize = 0;

    for line in bufreader.lines().filter_map(|line| line.ok()) {
        line_count += 1;
        byte_count += line.bytes().count();
        char_count += line.chars().count();
        word_count += line.split(' ').count();
    }

    FileCount {
        words: word_count,
        lines: line_count,
        bytes: byte_count,
        characters: char_count
    }
}

fn open(file: &str) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match file {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(file)?))),
    }
} 
