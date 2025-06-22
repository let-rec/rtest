/// These public fns for main.rs commented code -- JUST READ FILE CLI -- 
use std::error::Error;
use std::fs;
use std::env;




pub fn run(config: Config) -> Result<(), Box<dyn Error> > {
    let contents = fs::read_to_string(config.filename)?;

    let res = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_instentive(&config.query, &contents)
    };

    for line in res{
        println!("{}", line)
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {    
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough argumenrs");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
        Ok(Config {query,filename, case_sensitive})

    }    
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }
    res
}

pub fn search_case_instentive<'a> (
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut res = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "docs";
        let contents = "\
        Rust: 
        safe, fast, productive.
        three pluses";
        assert_eq!(vec!["safe,fast,productive."], search(query,contents));
    }
    #[test]
    fn case_insensitive(){
        let query = "docs";
        let contents = "\
        Rust: 
        safe, fast, productive.
        three pluses";
        assert_eq!(vec!["safe,fast,productive."], search_case_instentive(query,contents));
    }
}
