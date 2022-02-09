use minigrep::app;

fn main() {
    std::process::exit(match app() {
        Ok(_) => 0,
        Err(errmsg) => {
            eprintln!("{}", errmsg);
            1
        }
    });
}
