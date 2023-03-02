use std::fs;
use std::error::Error;
use clap::Parser;

#[derive(Parser)]
pub struct Input {
    #[arg(long)]
    #[arg(help = "ignore case distinctions in patterns and data")]
    ignore_case: bool,
    #[arg(help = "the pattern to search")]
    pattern: String,
    #[arg(help = "the file to read")]
    file: std::path::PathBuf,
}

// impl Input {
//     pub fn build(args: &[String]) -> Result<Input, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }

//         let pattern: String = args[1].clone();
//         let path: String = args[2].clone();
    
//         Ok(Input { pattern, path })
//     }    
// }

pub fn run(input: Input) -> Result<(), Box<dyn Error>> {
    let contents: String = fs::read_to_string(&input.file)?;
    println!("searching for \"{}\" in {:?}.", input.pattern, input.file);

    let results: Vec<&str> = if input.ignore_case {
        search_case_insensitve(&input.pattern, &contents)
    } else {
        search(&input.pattern, &contents)
    };
    
    for line in results {
        println!("{line}");
    }
    
    Ok(())
}

pub fn search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(pattern))
        .collect()
}

pub fn search_case_insensitve<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    // let pattern: String = pattern.to_lowercase();
    // let mut results: Vec<&str> = Vec::new();

    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&pattern) {
    //         results.push(line)
    //     }
    // }

    // results

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&pattern.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let pattern: &str = "duct";
        let contents: &str = "\
rust:
safe, fast, productive.
others:
pick one.";

        assert_eq!(vec!["safe, fast, productive."], search(pattern, contents));
    }
}
