use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    // Option control variables
    let mut bytes: bool = false;
    let mut chars: bool = false;
    let mut lines: bool = false;
    let mut words: bool = false;

    // Counter variables
    let mut byte_counter: u64 = 0;
    let mut char_counter: u64 = 0;
    let mut line_counter: u64 = 0;
    let mut word_counter: u64 = 0;

    // Filepath
    let mut filepath: String = String::new();

    // Contents from file or stdin
    let mut contents: String = String::new();

    // Output string
    let mut output: String = String::new();

    // Arguments reading
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    // Arguments parsing
    for arg in args.iter() {
        match arg.as_str() {
            "-c" => bytes = true,
            "-l" => lines = true,
            "-w" => words = true,
            "-m" => chars = true,
            _ => {
                if filepath.as_str() == "" {
                    filepath = arg.to_string();
                } else {
                    panic!("ERROR: Only one file can be processed at a time.");
                }
            }
        };
    }

    // Condition: No flags
    let no_additional_args: bool = !lines && !words && !chars && !bytes;

    // File (if source is not stdin)
    let file: File;

    //
    let mut source: Box<dyn BufRead>;

    if filepath.as_str() == "" {
        source = Box::new(io::stdin().lock());
    } else {
        if !Path::new(&filepath).exists() {
            panic!("ERROR: File doesn't exist.");
        } else {
            file = File::open(&filepath).expect("ERROR: Could not open file.");
            source = Box::new(BufReader::new(file));
        }
    }

    while let Ok(n) = source.read_line(&mut contents) {
        if n == 0 {
            break;
        }
        if lines || no_additional_args {
            line_counter += 1;
        }
        if words || no_additional_args {
            word_counter += contents.split_whitespace().collect::<Vec<&str>>().len() as u64;
        }
        if chars {
            char_counter += contents.chars().collect::<Vec<char>>().len() as u64;
        }
        if bytes || no_additional_args {
            byte_counter += contents.as_bytes().len() as u64;
        }
        contents.clear();
    }

    if lines || no_additional_args {
        output.push_str(format!("{} ", line_counter).as_str());
    }

    if words || no_additional_args {
        output.push_str(format!("{} ", word_counter).as_str());
    }

    if chars {
        output.push_str(format!("{} ", char_counter).as_str());
    }

    if bytes || no_additional_args {
        output.push_str(format!("{} ", byte_counter).as_str());
    }

    output.push_str(&filepath);

    println!("{}", output);
}
