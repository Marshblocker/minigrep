use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};

mod err;

use err::{MyError::*, *};

pub fn app() -> Result<(), ErrMsg> {
    let console_args: Vec<String> = env::args().collect();

    let (path, pattern): (PathBuf, &String) = match validate_args(&console_args) {
        Ok((path, pattern)) => (path, pattern),
        Err(errmsg) => return Err(errmsg),
    };
    
    if let Err(errmsg) = grep_file(path, pattern) {
        return Err(errmsg);
    }

    Ok(())
}

fn validate_args<'a>(args: &'a Vec<String>) -> Result<(PathBuf, &'a String), ErrMsg> {
    match args.len() {
        3 => (),
        0..=2 => return Err(FewArgsErr.to_str()),
        4.. => return Err(ManyArgsErr.to_str()),
        _ => panic!("Not allowed to go here!"),
    };

    let path: &Path = Path::new(args[1].as_str());
    let path: PathBuf = if !path.exists() {
        return Err(InvalidPathErr.to_str());
    } else {
        path.to_path_buf()
    };

    let pattern: &String = if !args[2].as_str().is_ascii() {
        return Err(NonAsciiPatternErr.to_str());
    } else {
        &args[2]
    };

    Ok((path, pattern))
}

fn grep_file(path: PathBuf, pattern: &String) -> Result<(), ErrMsg> {
    let file: File = match File::open(path.as_path()) {
        Ok(file) => file,
        Err(_) => return Err(IOErr.to_str()),
    };

    let file = BufReader::new(file);

    println!("\nFinding '{}' in {}...\n", pattern, path.to_str().unwrap());

    let mut matched_count: u32 = 0;
    for (i, line) in file.lines().enumerate() {
        let line: String = match line {
            Ok(line) => line,
            Err(_) => return Err(ReadErr.to_str()),
        };

        let line_len = line.len();
        let ptn_len = pattern.len();
        let mut matched_substring_indices: Vec<usize> = Vec::new();

        for i in 0..=line_len - ptn_len {
            if &line[i..i + ptn_len].to_string() == pattern {
                matched_substring_indices.push(i);
            }
        }

        if matched_substring_indices.len() > 0 {
            matched_count += 1;
            println!("Line {}: {} {:?}", i, line, matched_substring_indices);
        }
    }

    match matched_count {
        0 => println!("\nNo lines that match the pattern '{}'.", pattern),
        1 => println!("\nFound 1 line that matches the pattern '{}'.", pattern),
        2.. => println!(
            "\nFound {} lines that match the pattern '{}'.",
            matched_count, pattern
        ),
    }

    Ok(())
}
