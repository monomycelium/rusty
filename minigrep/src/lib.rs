use std::fs;
use std::error::Error;

pub struct Input {
    query: String,
    file_path: String,
}

impl Input {
    pub fn build(args: &[String]) -> Result<Input, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query: String = args[1].clone();
        let file_path: String = args[2].clone();
    
        Ok(Input { query, file_path })
    }    
}

pub fn run(input: Input) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(&input.file_path)?;
    println!("searching for `{}` in `{}`.", input.query, input.file_path);

    for line in search(&input.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query: &str = "duct";
        let contents: &str = "\
rust:
safe, fast, productive.
others:
pick one.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
