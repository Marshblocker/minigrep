use std::env;
use std::path::{Path, PathBuf};
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;

use minigrep::err::{MyError::*, ErrMsg};


fn main() {
    
    std::process::exit(match app() {
        Ok(_)       => 0,
        Err(errmsg) => {
            eprintln!("{}", errmsg);
            1
        }
    })
}

fn app() -> Result<(), ErrMsg> {
    
    let console_args: Vec<String> = env::args().collect();

    let (path, pattern): (PathBuf, &String) = match validate_args(&console_args) {
        Ok((path, pattern)) => (path, pattern),
        Err(errmsg)         => return Err(errmsg),
    };

    if let Err(errmsg) = grep_file(path, pattern) {
        return Err(errmsg);
    }

    Ok(())
}

fn validate_args<'a>(args: &'a Vec<String>, ) -> Result<(PathBuf, &'a String), ErrMsg> {
    
    match args.len() {
        3     => (),
        0..=2 => return Err(FewArgsErr.to_str()),
        4..   => return Err(ManyArgsErr.to_str()),
        _     => panic!("Not allowed to go here!"),
    };

    let path: &Path = Path::new(args[1].as_str());
    dbg!(&path);
    let path: PathBuf = if !path.exists() {
        return Err(InvalidPathErr.to_str())
    } else {
        path.to_path_buf()
    };

    let pattern: &String = if !args[2].as_str().is_ascii() {
        return Err(NonAsciiPatternErr.to_str())
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

    println!("\nFinding '{}' in {:?}...\n", pattern, path);

    let mut c: u32 = 0;
    for (i, line) in file.lines().enumerate() {
        let line: String = match line {
            Ok(line) => line,
            Err(_)   => return Err(ReadErr.to_str()),
        };

        if line.contains(pattern) {
            c += 1;
            println!("Line {}: {}", i+1, line);
        }
    }

    match c {
        0   => println!("\nNo lines that match the pattern '{}'.", pattern),
        1   => println!("\nFound 1 line that matches the pattern '{}'.", pattern),
        2.. => println!("\nFound {} lines that match the pattern '{}'.", c, pattern),
    }

    Ok(())
}