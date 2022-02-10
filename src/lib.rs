use std::env;

mod err;
mod structs;

use err::ErrMsg;
use structs::*;

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
