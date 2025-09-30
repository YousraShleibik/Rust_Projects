use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn read_program_file(filename: &str) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|res| res.map(|line| line.trim_end_matches('\r').to_string()))
        .collect()
}


pub fn is_keyword(word: &str) -> bool {
    matches!(
        word,
        "and"
            | "class"
            | "else"
            | "false"
            | "for"
            | "fun"
            | "if"
            | "nil"
            | "or"
            | "print"
            | "return"
            | "super"
            | "this"
            | "true"
            | "var"
            | "while"
    )
}

fn main() {
    match read_program_file("program.txt") {
        Ok(lines) => {
            println!(" Successfully read 'program.txt' ({} line(s)).", lines.len());
            for (i, line) in lines.iter().enumerate() {
                println!("{i}: {line}");


                for word in line.split_whitespace() {
                    if is_keyword(word) {
                        println!("   -> '{word}' is a keyword");
                    } else {
                        println!("   -> '{word}' is NOT a keyword");
                    }
                }
            }
            
        }
        Err(e) => {
            eprintln!("Could not open/read 'program.txt': {e}");
        }
    }
}