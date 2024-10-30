use std::fs;
use std::error::Error;
use std::env;

/* pub fn old_run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query,&contents)
    } else {
        search(&config.query, &contents) 
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
} */

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let files = std::fs::read_dir(&config.dir_path)?;

    for file_path in files{
        let file_name =  file_path.unwrap().file_name().into_string().unwrap();
        println!("{dir_path}/{file}",file=file_name,dir_path=config.dir_path);
    }
    
    Ok(())
}


/// Input config struct

pub struct Config {
    pub query: String,
    pub dir_path: String,
    pub api_key: String,
}

impl Config{
    pub fn build(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3{
            return Err("not enough arguments provided");
        }

        let query = args[1].clone();
        let dir_path = args[2].clone();

        let api_key = env::var("API_KEY").unwrap();

        Ok(Config {query,dir_path,api_key,})
    }
}


/// Text Search functions

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    for lines in contents.lines(){
        if lines.contains(query){
            results.push(lines);
        }
    } 
   results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    
    for lines in contents.lines(){
        if lines.to_lowercase().contains(&query){
            results.push(lines);
        }
    }
    results
}

/// Tests

#[cfg (test)]
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



    #[test]
    fn case_sensitive() {
         let query = "duct";
         let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));

    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query,contents));
    }
}