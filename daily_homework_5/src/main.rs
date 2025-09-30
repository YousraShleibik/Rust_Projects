
pub fn read_program_file(filename: &str) -> Vec<String> {
    match std::fs::read_to_string(filename) {
        Ok(text) => text.lines().map(|s| s.to_string()).collect(),
        Err(err) => {
            eprintln!("Failed to read '{}': {}", filename, err);
            Vec::new()
        }
    }
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




pub fn split_string(s: &str) -> Vec<String> {
    s.split_whitespace().map(|w| w.to_string()).collect()
}

pub fn split_file_into_words(filename: &str) -> Vec<String> {
    match std::fs::read_to_string(filename) {
        Ok(text) => split_string(&text),
        Err(err) => {
            eprintln!("Failed to read '{}': {}", filename, err);
            Vec::new()
        }
    }
}




fn main() {
    let lines = read_program_file("program.txt");
    if lines.is_empty() {
        eprintln!("(No lines read — file missing, empty, or unreadable.)");
    } else {
        println!(" Successfully read 'program.txt' ({} line(s)).", lines.len());
        for (i, line) in lines.iter().enumerate() {
            println!("{i}: {line}");
            for w in line.split_whitespace() {
                println!("   -> '{w}' keyword? {}", is_keyword(w));
            }
        }
    }

    let words = split_file_into_words("program.txt");
    println!("Total words found: {}", words.len());
}
