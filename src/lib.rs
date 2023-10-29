use std::{env, error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    for line in search(&config.query, &contents, config.ignore_case) {
        println!("{line}");
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(query) => query,
            None => return Err("Query is required"),
        };

        let file_path = match args.next() {
            Some(file_path) => file_path,
            None => return Err("File path is required"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn search<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| {
            ignore_case && line.to_lowercase().contains(query.to_lowercase().as_str())
                || line.contains(query)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_build() {
        let args = [
            "minigrep".to_string(),
            "test".to_string(),
            "test.txt".to_string(),
        ]
        .into_iter();
        let config = Config::build(args).unwrap();
        assert_eq!(config.query, "test");
        assert_eq!(config.file_path, "test.txt");
    }

    mod case_sensitive {
        use super::*;

        #[test]
        fn one_result() {
            let query = "duct";
            let contents = "\
Rust:
safe, fast, productive.
Pick three.";

            assert_eq!(
                vec!["safe, fast, productive."],
                search(query, contents, false)
            );
        }

        #[test]
        fn multiple_results() {
            let query = "nobody";
            let contents = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!";

            assert_eq!(
                vec!["I'm nobody! Who are you?", "Are you nobody, too?"],
                search(query, contents, false)
            );
        }
    }

    mod case_insensitive {
        use super::*;

        #[test]
        fn one_result() {
            let query = "rUsT";
            let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

            assert_eq!(vec!["Rust:", "Trust me."], search(query, contents, true));
        }

        #[test]
        fn multiple_results() {
            let query = "nObOdY";
            let contents = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!";

            assert_eq!(
                vec!["I'm nobody! Who are you?", "Are you nobody, too?"],
                search(query, contents, true)
            );
        }
    }
}
