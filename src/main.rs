use std::process;

fn main() {
    let error = shapes::run();
    eprintln!("{}", error);
    process::exit(1);
}
