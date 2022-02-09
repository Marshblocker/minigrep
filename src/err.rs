pub type ErrMsg = String;

#[derive(Debug)]
pub enum MyError {
    FewArgsErr,
    ManyArgsErr,
    InvalidPathErr,
    NonAsciiPatternErr,
    IOErr,
    LongPatternErr,
}

impl MyError {
    pub fn to_str(&self) -> ErrMsg {
        use self::MyError::*;

        let mut err = String::from(format!("{:?}: ", self));

        err += match self {
            FewArgsErr         => "Too few arguments. Must exactly have two arguments.\n",
            ManyArgsErr        => "Too many arguments. Must exactly have two arguments. \
                                   Make sure the FILEPATH or PATTERN has no whitespace within them.\n",
            InvalidPathErr     => "The given FILEPATH does not exist or cannot be accessed due to permission reasons.\n",
            NonAsciiPatternErr => "The given PATTERN has non-ASCII values.\n",
            IOErr              => "Failed to access the given file.\n",
            LongPatternErr     => "The given PATTERN exceeds the standard length of a file line.\n"
        };
        err += "Usage: cargo run FILEPATH PATTERN\n";

        err
    }
}
