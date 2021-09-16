/*//grep: Globally search a Regular Expression and Print

use std::env;
//use std::env::args; //return an iterator of the command line arguments that were given to minigrep
//the std::env::args will panic if any argument contains invalid Unicode
//use std::fs;    //handle files
use std::process;
use minigrep::Config;
fn main() {


    let args: Vec<String> = env::args().collect();

    //let test = &args[0];
    //let query = &args[1];
    //let filename = &args[2];

    //let (query, filename) = parse_config(&args);
    //let config = parse_config(&args);
    //let config = Config::new(&args);

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("problem parsing arguments: {}", err);
        process::exit(1);
    });

    //println!("the test is {}", test);
   // println!("searching for {}", config.query);
   // println!("In file {}", config.filename);
    //println!("{:?}", config.args);
    //println!("Hello, world!");

    //let contents = fs::read_to_string(config.filename).expect("something went wrong reading the file");
    // open the file, return the Result<String> of the file's contents
    //println!("with text:\n{}", contents);
}

/*struct Config {
    query: String,
    filename: String,
}
*/
/*impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}*/

/*impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
*/
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}*/

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // --snip--
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    /*println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
*/
    if let Err(e) = minigrep::run(config) {
        // --snip--
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
//finally, it worked! but i still do not know why?