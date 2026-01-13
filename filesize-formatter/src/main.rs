use std::env;

enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size as f64 / 1000.0),
        1_000_000..=999_999_999 => FileSize::Megabytes(size as f64 / 1_000_000.0),
        _ => FileSize::Gigabytes(size as f64 / 1_000_000_000.0),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
    }
}

fn parse_size_input(input: &str) -> Result<(f64, String), String> {
    let input = input.trim().to_lowercase();
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    if parts.len() != 2 {
        return Err("Invalid format. Expected: <number> <unit> (e.g., '24 mb')".to_string());
    }
    
    let size: f64 = parts[0].parse()
        .map_err(|_| format!("Invalid number: {}", parts[0]))?;
    
    let unit = parts[1].to_string();

    Ok((size, unit))
}

fn convert_to_all_representations(size: f64, unit: &str) -> Result<Sizes, String> {
    let bytes = match unit {
        "bytes" | "byte" | "b" => size as u64,
        "kilobytes" | "kilobyte" | "kb" | "k" => (size * 1000.0) as u64,
        "megabytes" | "megabyte" | "mb" | "m" => (size * 1_000_000.0) as u64,
        "gigabytes" | "gigabyte" | "gb" | "g" => (size * 1_000_000_000.0) as u64,
        _ => return Err(format!("Unknown unit: {}. Supported units: bytes, kilobytes, megabytes, gigabytes", unit)),
    };
    
    let kb = bytes as f64 / 1000.0;
    let mb = bytes as f64 / 1_000_000.0;
    let gb = bytes as f64 / 1_000_000_000.0;
    
    Ok(Sizes {
        bytes: format!("{} bytes", bytes),
        kilobytes: format!("{} kilobytes", kb as u64),
        megabytes: format!("{} megabytes", mb as u64),
        gigabytes: format!("{} gigabytes", gb as u64),
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        // Original behavior if no arguments
        let result = format_size(6888837399);
        println!("{}", result);
        return;
    }
    
    let input = &args[1];
    match parse_size_input(input) {
        Ok((size, unit)) => {
            match convert_to_all_representations(size, &unit) {
                Ok(sizes) => {
                    println!("{}", sizes.bytes);
                    println!("{}", sizes.kilobytes);
                    println!("{}", sizes.megabytes);
                    println!("{}", sizes.gigabytes);
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
