use std::env;
use std::process;
use mingrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    if let Err(e) = mingrep::run(config) {
        println!("{}", e);
        process::exit(1);
    }
}
