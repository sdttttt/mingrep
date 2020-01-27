use std::env;
use std::fs;
use std::process;

struct Config {
    query: String,
    file_name: String,
}

impl Config {
    fn new(args: Vec<String>) -> Result<Config, String> {

        if args.len() < 3 {
            return Err(String::from("Fuck you!"));
        }

        Ok(Self {
            query: args[1].clone(),
            file_name: args[2].clone(),
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    let contents = fs::read_to_string(config.file_name).expect("有问题");

    println!("{},{}", config.query, contents);
}


