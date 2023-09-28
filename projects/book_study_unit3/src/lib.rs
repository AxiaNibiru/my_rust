use std::env;
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // 跳过文件名称
        let _application_name: Option<String> = args.next();
        // 获取查询字符串
        let query = args.next().expect("Didn't get a query string");
        // 获取查询路径
        let file_path: String = args.next().expect("Didn't get a file path");
        // 是否忽略大小写参数
        let ignore_case: bool = env::var("IGNORE_CASE").map_or(
            args.any(|arg| arg.to_lowercase().contains("-i")),
            |env_value| env_value.eq("1"),
        );

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // 获取文件内容
    let contents: String = std::fs::read_to_string(&config.file_path)?;

    // 因为String实现了Deref trait所以加上&之后能够自动变为&str类型
    let find_str: Vec<&str> = match config.ignore_case {
        true => search(&config.query, &contents),
        false => search_case_insensitive(&config.query, &contents),
    };

    //输出查找结果
    find_str.iter().for_each(|&x| println!("{x}"));
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|&x| x.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|&x| x.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // case_sensitive
    fn one_result() {
        let query: &str = "duct";
        let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
