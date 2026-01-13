use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        print!("Enter path to file to read: ");
        io::stdout().flush().expect("failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read from stdin");
        input.trim().to_string()
    });

    if path.is_empty() {
        panic!("No file path provided.");
    }

    match File::open(&path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                match line {
                    Ok(line) => println!("{}", line),
                    Err(error) => {
                        panic!("Error reading line: {}", error)
                    }
                }
            }
        }
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File not found: {}", error)
            }
            _ => {
                panic!("Error opening file: {}", error)
            }
        },
    }
}
