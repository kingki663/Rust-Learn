use dotenv::dotenv;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensetive: bool,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Self, &'static str> {
        dotenv().ok();
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Self {
            query: args[1].clone(),
            filename: args[2].clone(),
            case_sensetive: std::env::var("CASE_SENSETIVE")
                .is_ok_and(|v| v == "true" || v == "ok" || v == "1"),
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let results = if config.case_sensetive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for (linenum, line) in results {
        println!("line: {} contents: {}", linenum, line)
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> HashMap<usize, &'a str> {
    let mut results = HashMap::new();

    for (line_num, line) in contents.lines().enumerate() {
        if line.contains(query) {
            results.insert(line_num + 1, line);
        }
    }

    results
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> HashMap<usize, &'a str> {
    let query = query.to_lowercase();
    let mut results = HashMap::new();

    for (line_num, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&query) {
            results.insert(line_num + 1, line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;

    #[test]
    fn test_config_new() {
        let args = vec![
            format!("some_exe.exe"),
            format!("hello"),
            format!("poem.txt"),
        ];
        let config = Config::new(&args).unwrap();
        assert_eq!(config.query, args[1]);
        assert_eq!(config.filename, args[2]);
    }

    #[test]
    fn test_config_new_insufficient_arguments() {
        let args = vec![format!("some_exe.exe"), format!("hello")];
        let result = Config::new(&args);
        assert!(result.is_err()); // 确保返回了错误结果
        assert_eq!(result.unwrap_err(), "not enough arguments"); // 确保错误信息匹配
    }

    #[test]
    fn test_run() {
        let config = Config {
            query: String::from("hello"),
            filename: String::from("poem.txt"),
            case_sensetive: false,
        };

        let result = run(&config);

        assert!(result.is_ok()); // 确保执行成功
    }

    #[test]
    fn test_run_incorrect_arguments() {
        let config = Config {
            query: String::from("hello"),
            filename: String::from("NOTEXSITS.TXT"),
            case_sensetive: false,
        };

        let result = run(&config);

        assert!(result.is_err()); // // 确保返回了错误结果
    }

    #[test]
    fn test_search() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        // let mut expected: HashMap<usize, &str> = HashMap::new();
        // expected.insert(2, "safe, fast, productive.");
        let expected = hashmap! {
            2 => "safe, fast, productive."
        };
        assert_eq!(expected, search(query, contents));
    }

    #[test]
    fn test_search_case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        let expected = hashmap! {
            1 => "Rust:",
            4 => "Trust me."
        };
        assert_eq!(expected, search_case_insensitive(query, contents));
    }
}
