use minigrep::search;
use minigrep::search_case_insensitive;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    fn build(params: &[String]) -> Result<Config, &str> {
        if params.len() < 3 {
            return Err("not enough params");
        }
        let query = params[1].clone();
        let file_path = params[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    //    let config = match Config::build(&args) {
    //        Ok(conf) => conf,
    //        Err(e) => {
    //            println!("error parsing arguments: {}", e);
    //            process::exit(1);
    //        }
    //    };
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("error parsing arguments: {}", err);
        process::exit(1);
    });
    //println!("searching for {}", config.query);
    //println!("in file {}", config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    //println!("with contents:\n{contents}");
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}
