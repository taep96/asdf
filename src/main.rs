use std::process;

fn main() {
    if let Err(e) = asdf::run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
