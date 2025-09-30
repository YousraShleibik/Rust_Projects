
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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_keyword_basic() {
        assert!(is_keyword("return"));
        assert!(!is_keyword("Return")); // case-sensitive
    }

    #[test]
    fn split_string_basic() {
        assert_eq!(split_string("  a  b\tc\nd  "), vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn read_program_file_basic() {
        // make a temp file path
        let mut path = std::env::temp_dir();
        path.push(format!("hw5_{}.txt", std::process::id()));

        // write two lines
        let content = "print 1\nvar x = 2\n";
        std::fs::write(&path, content).unwrap();

        // read lines
        let lines = read_program_file(path.to_str().unwrap());
        assert_eq!(lines, vec!["print 1".to_string(), "var x = 2".to_string()]);

        // split all words using your split_string
        let words: Vec<String> = lines.iter().flat_map(|l| split_string(l)).collect();
        assert_eq!(words, vec!["print", "1", "var", "x", "=", "2"]);

        // clean up
        let _ = std::fs::remove_file(&path);
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
