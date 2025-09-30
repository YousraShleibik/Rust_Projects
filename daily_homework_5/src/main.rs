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


fn main() {
    match read_program_file("program.txt") {
        Ok(lines) => {
            println!(" Successfully read 'program.txt' ({} line(s)).", lines.len());
            for (i, line) in lines.iter().enumerate() {
                println!("{i}: {line}");
            }
        }
        Err(e) => {
            eprintln!("Could not open/read 'program.txt': {e}");
        }
    }
}
