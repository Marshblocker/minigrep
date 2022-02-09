use std::env;
use std::fs;
use std::path::{Path, PathBuf};

mod err;

use err::{MyError::*, *};

pub struct Config {
    path: String,
    pattern: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, ErrMsg> {
        match args.len() {
            3 => (),
            0..=2 => return Err(FewArgsErr.to_str()),
            4.. => return Err(ManyArgsErr.to_str()),
            _ => panic!("Not allowed to go here!"),
        };

        let path = args[0].clone();
        let pattern = args[1].clone();

        if !pattern.is_ascii() {
            return Err(NonAsciiPatternErr.to_str());
        }

        Ok(Config { path, pattern })
    }
}

pub struct Reader {
    file_content: String,
}

impl Reader {
    pub fn new(path: &String) -> Result<Reader, ErrMsg> {
        let file_path: PathBuf = match Self::get_file_path(path) {
            Ok(file_path) => file_path,
            Err(errmsg) => return Err(errmsg),
        };

        let file_content: String = match Self::get_file_content(&file_path) {
            Ok(content) => content,
            Err(errmsg) => return Err(errmsg),
        };

        Ok(Reader { file_content })
    }

    fn get_file_path(path: &String) -> Result<PathBuf, ErrMsg> {
        let file_path = Path::new(path.as_str());
        let file_path = match file_path.exists() {
            true => file_path.to_path_buf(),
            false => return Err(InvalidPathErr.to_str()),
        };

        Ok(file_path)
    }

    fn get_file_content(file_path: &PathBuf) -> Result<String, ErrMsg> {
        let file_content = match fs::read_to_string(file_path.as_path()) {
            Ok(content) => content,
            Err(_errmsg) => return Err(IOErr.to_str()),
        };

        Ok(file_content)
    }
}

pub struct Grepper {
    file_content: String,
    pattern: String,
}

impl Grepper {
    pub fn grep(&self) -> Result<(), ErrMsg> {
        let mut matched_count = 0;
        for (i, line) in self.file_content.lines().enumerate() {
            let temp = Self::find_pattern_occurrences_in_line(self, line);
            let matched_substring_indices = match temp {
                Ok(indices) => indices,
                Err(errmsg) => return Err(errmsg),
            };

            if matched_substring_indices.len() > 0 {
                matched_count += 1;
                println!("Line {}: {} {:?}", i, line, matched_substring_indices);
            }
        }

        Self::print_summary(self, matched_count);

        Ok(())
    }

    fn find_pattern_occurrences_in_line(&self, line: &str) -> Result<Vec<usize>, ErrMsg> {
        let ptn_len = self.pattern.len();
        let mut matched_substring_indices: Vec<usize> = Vec::new();

        if ptn_len > line.len() {
            return Err(LongPatternErr.to_str());
        }

        let max_range = line.len() - ptn_len;

        for i in 0..=max_range {
            if &line[i..i + ptn_len].to_string() == &self.pattern {
                matched_substring_indices.push(i);
            }
        }

        Ok(matched_substring_indices)
    }

    fn print_summary(&self, matched_count: u32) {
        match matched_count {
            0 => println!("\nNo lines that match the pattern '{}'.", self.pattern),
            1 => println!(
                "\nFound 1 line that matches the pattern '{}'.",
                self.pattern
            ),
            2.. => println!(
                "\nFound {} lines that match the pattern '{}'.",
                matched_count, self.pattern
            ),
        }
    }
}

pub fn app() -> Result<(), ErrMsg> {
    let console_args: Vec<String> = env::args().collect();

    let config = match Config::new(&console_args) {
        Ok(config) => config,
        Err(errmsg) => return Err(errmsg),
    };

    let reader = match Reader::new(&config.path) {
        Ok(reader) => reader,
        Err(errmsg) => return Err(errmsg),
    };

    let grepper = Grepper {
        file_content: reader.file_content,
        pattern: config.pattern,
    };

    if let Err(errmsg) = grepper.grep() {
        return Err(errmsg);
    }

    Ok(())
}
