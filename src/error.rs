#[derive(PartialEq, Debug)]
pub enum MyError {
    FewArgsErr,
    ManyArgsErr,
    InvalidPathErr,
    NoDelimPatternErr,
    NoPatternErr,
}

impl MyError {
    pub fn handle_error(err: &MyError) {
        eprint!("\n{:?}: ", *err);

        match *err {
            MyError::FewArgsErr         => Self::few_args_err_messg(),
            MyError::ManyArgsErr        => Self::many_args_err_messg(),
            MyError::InvalidPathErr     => Self::invalid_path_err_messg(),
            MyError::NoDelimPatternErr  => Self::no_delim_pattern_err_messg(),
            MyError::NoPatternErr       => Self::no_pattern_err_messg(),
        }

        eprintln!("usage: rgrep FILEPATH 'PATTERN'");

    }

    fn few_args_err_messg() {
        eprintln!("Received too few arguments.");
    }

    fn many_args_err_messg() {
        eprintln!("Received too many arguments.");
    }

    fn invalid_path_err_messg() {
        eprintln!("The given path does not exist or cannot be accessed \
                   due to permission reasons.");
    }

    fn no_delim_pattern_err_messg() {
        eprintln!("The PATTERN should be enclosed like this: 'PATTERN'.");
    }

    fn no_pattern_err_messg() {
        eprintln!("Received no PATTERN.");
    }
}