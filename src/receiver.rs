use std::{env, path::Path};
use crate::error::MyError::{self, *};

const NUM_ARGS: usize = 2;
const DELIM: char = '\'';

pub struct Receiver {
    path: String,
    pattern: String,
}

impl Receiver {
    pub fn new() -> Result<Receiver, MyError> {
        let (arg1, arg2) = match Self::get_args() {
            Ok([a, b]) => (a, b),
            Err(e) => return Err(e),
        };

        let path_check: &Path = Path::new(&arg1);

        if !path_check.exists() {
            return Err(InvalidPathErr);
        }

        let path = arg1;

        let pattern = match Self::extract_pattern_str(arg2) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };

        Ok(Receiver {path, pattern})
    }

    pub fn get_fields(&self) -> (String, String) {
        (self.path.clone(), self.pattern.clone())
    }

    fn get_args() -> Result<[String; NUM_ARGS], MyError> {
        let args: Vec<String> = env::args().collect();

        // The +1 is for args[0] which is not needed.
        if args.len() < NUM_ARGS + 1 {
            return Err(FewArgsErr);
        }

        let mut i = 1;

        for (i_, arg) in args.iter().enumerate() {
            if arg.starts_with(DELIM) {
                i = i_;
            }
        }

        let arg1 = args[1..i].join(" ");

        let last_arg = &args[args.len() - 1];

        if !last_arg.ends_with(DELIM) && args.len() > NUM_ARGS + 1 {
            return Err(ManyArgsErr);
        }

        let arg2 = args[i..].join(" ");

        Ok([arg1, arg2])
    }

    fn extract_pattern_str(arg: String) -> Result<String, MyError> {
        let mut pattern = arg.as_str();

        if pattern.starts_with(DELIM) && pattern.ends_with(DELIM) {
            let no_quotes_arg = pattern.strip_prefix(DELIM)
                                       .unwrap()
                                       .strip_suffix(DELIM);

            pattern = match no_quotes_arg {
                Some(p) => match p {
                    "" => return Err(NoPatternErr),
                    p  => p,
                },
                None    => return Err(NoDelimPatternErr),
            };
        } else {
            return Err(NoDelimPatternErr);
        }

        Ok(pattern.to_string())
    }
}