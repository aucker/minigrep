//TDD: Test-Driven Development
//1.write a test that fails and run it to make sure it fails for the reason you expect
//2.write or modify just enough code to make the new test pass
//3.refactor the code you just added or changed and make sure the tests continue to pass
//4.repeat from step 1

//use std::slice::Concat;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //vec![]
    let mut result = Vec::new();
    for line in contents.lines() {
        //do something
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}