use std::fs;
use std::error::Error;
use std::env;


pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
    args.next();

    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a query string"),
    };

    let filename = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a file name"),
    };

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

  contents
    .lines()
    .map(|line| line.trim())
    .filter(|line| {
      if case_sensitive {
        line.contains(&query)
      } else {
        line.to_lowercase().contains(&query)
      }
    })
    .collect()
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
