use minigrep_1::search;
use minigrep_1::search_case_insensitive;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

/// config struct that holds
/// search query, file path and case sensitive/insensitive flag
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

/// build is an associated function to create a new config
impl Config {
    fn build(mut params: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        params.next();
        let query = match params.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string !"),
        };
        let file_path = match params.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path !"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn main() {
    // let args = env::args().collect::<Vec<String>>();
    //    let config = match Config::build(&args) {
    //        Ok(conf) => conf,
    //        Err(e) => {
    //            println!("error parsing arguments: {}", e);
    //            process::exit(1);
    //        }
    //    };
    let config = Config::build(env::args()).unwrap_or_else(|err| {
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
    let results: Box<dyn Iterator<Item = &str> + '_> = if config.ignore_case {
        Box::new(search_case_insensitive(&config.query, &contents))
    } else {
        Box::new(search(&config.query, &contents))
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}
