use std::{env, fs};

fn read_file(path: &str) -> Vec<u8> {
    let contents: Result<Vec<u8>, std::io::Error> = fs::read(path);
    match contents {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e.to_string());
            std::process::exit(2);
        },
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("missing file operand".to_string());
    }
    let file_contents: Vec<u8> = read_file(&args[1]);
    Ok(())
}
