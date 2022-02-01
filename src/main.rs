use rgrep::receiver::Receiver;
use rgrep::error::MyError;

fn main() {
    std::process::exit(match run_app() {
        Ok(_)    => 0,
        Err(err) => {
            MyError::handle_error(&err);
            1
        }
    });
}

fn run_app() -> Result<(), MyError> {
    let receiver = match Receiver::new() {
        Ok(receiver_) => receiver_,
        Err(err)      => return Err(err),
    };

    let (path, pattern) = receiver.get_fields();

    println!("path: {}\npattern: {}", path, pattern);

    Ok(())
}
