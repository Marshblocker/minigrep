use std::fs;

use crate::err::{MyError::*, *};

#[derive(Debug, PartialEq)]
pub struct Config {
    pub path: String,
    pub pattern: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, ErrMsg> {
        match args.len() {
            3 => (),
            0..=2 => return Err(FewArgsErr.to_str()),
            4.. => return Err(ManyArgsErr.to_str()),
            _ => panic!("Not allowed to go here!"),
        };

        let path = args[1].clone();
        let pattern = args[2].clone();

        if !pattern.is_ascii() {
            return Err(NonAsciiPatternErr.to_str());
        }

        Ok(Config { path, pattern })
    }
}

#[derive(Debug, PartialEq)]
pub struct Reader {
    pub file_content: String,
}

impl Reader {
    pub fn new(path: &String) -> Result<Reader, ErrMsg> {
        let file_content: String = match Self::get_file_content(&path) {
            Ok(content) => content,
            Err(errmsg) => return Err(errmsg),
        };

        Ok(Reader { file_content })
    }

    fn get_file_content(path: &String) -> Result<String, ErrMsg> {
        let file_content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(_errmsg) => {
                return Err(IOErr.to_str());
            }
        };

        Ok(file_content)
    }
}

pub struct Grepper {
    pub file_content: String,
    pub pattern: String,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_test() {
        let args = vec![String::from("/path/to/executable")];
        assert_eq!(Err(MyError::FewArgsErr.to_str()), Config::new(&args));

        let args = vec![
            String::from("/path/to/executable"),
            String::from("this"),
            String::from("has"),
            String::from("too"),
            String::from("many args"),
        ];
        assert_eq!(Err(MyError::ManyArgsErr.to_str()), Config::new(&args),);

        let args = vec![
            String::from("/path/to/executable"),
            String::from("/path/to/target/file"),
            String::from("ӫⲀӍ骜k_ψ񢁧賴O򾢈p葽*㜗ѭ)͖􊀻̺ېާT𘎤~٢򭱨V⥩ގQ"),
        ];
        assert_eq!(
            Err(MyError::NonAsciiPatternErr.to_str()),
            Config::new(&args),
        );
    }
}
