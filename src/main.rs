use std::fs;
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir_path = if args.len() > 1 {
        &args[1]
    } else {
        "."
    };
    
    match fs::read_dir(dir_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.file_name().to_str() {
                        println!("{}", file_name);
                    }
                }
            }
        },
        Err(e) => {
            eprintln!("Error reading directory '{}': {}", dir_path, e);
            std::process::exit(1);
        }
    }
}
