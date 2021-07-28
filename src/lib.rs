use std::fs;
use std::error::Error;
use std::env;


pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
      if args.len() < 3 {
          return Err("Not enough arguments");
      }

      let query = args[1].clone();
      let filename = args[2].clone();

      let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
  
      Ok(Config { query, filename, case_sensitive })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  for line in search(&config.query, &contents, config.case_sensitive) {
    println!("{}", line);
  }

  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
  let query = if case_sensitive { String::from(query) } else { query.to_lowercase() };

  let mut result_list = Vec::new();

  if case_sensitive {
    for line in contents.lines() {
      if line.contains(&query) {
        result_list.push(line.trim());
      }
    }
  } else {
    for line in contents.lines() {
      if line.to_lowercase().contains(&query) {
        result_list.push(line.trim());
      }
    }
  }
  
  result_list
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
      Pick three.
      Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents, true));
  }

  #[test]
  fn case_insensitive() {
    let query = "rUst";
    let contents = "\
      Rust:
      safe, fast, productive.
      Pick three.
      Trust me.";
    
    assert_eq!(vec!["Rust:", "Trust me."], search(query, contents, false));
  }
}
