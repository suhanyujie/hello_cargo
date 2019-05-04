use std::fs;
use std::error::Error;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filename:String,
    pub case_sensitive:bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(query) => query,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(filename) => filename,
            None => return Err("Didn't get a file name"),
        };
        // if args.len() < 3 {
        //     return Err("not enough arguments");
        // }
        // let query = &args[1];
        // let filename = &args[2];
        // env::var 函数并传递我们需要寻找的环境变量名称，CASE_INSENSITIVE。env::var 返回一个 Result，它在环境变量被设置时返回包含其值的 Ok 成员，并在环境变量未被设置时返回 Err 成员。
        // 如果CASE_INSENSITIVE 环境变量被设置为任何值，is_err 会返回 false 并将进行大小写不敏感搜索。
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query:query,
            filename:filename,
            case_sensitive:case_sensitive,
        })
    }
}

pub fn run(config:Config) -> Result<(), Box<dyn Error>>{
    // 读取文件
    let content = fs::read_to_string(&config.filename)?;
    if config.case_sensitive {
        for line in search_by_iterator(&config.query, &content) {
            println!("{:?}", line);
        }
    } else {
        for line in search_case_insensitive(&config.query, &content) {
            println!("{:?}", line);
        }
    }
    
    Ok(())
}

fn search<'a>(query:&str, content:&'a str) -> Vec<&'a str> {
    let mut result = vec![];
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

// 大小写不敏感
fn search_case_insensitive<'a>(query:&str, content:&'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = vec![];
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

// 使用迭代器的方式搜索
fn search_by_iterator<'a>(query:&str,content: &'a str) ->Vec<&'a str> {
    content.lines()
            .filter(|line| line.contains(query))
            .collect()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn one_result()  {
        let query = "duct";
        let content = "\
Rust:
 language. duct...
 what is duct?";
        let res = search(&query, &content);
        assert_eq!(res, vec![" language. duct..."," what is duct?"]);
    }

    #[test]
    fn case_insensitive()  {
         let query = "duct";
        let content = "\
Rust:
 language. Duct...
 what is duct?";
        let res = search_case_insensitive(&query, &content);
        assert_eq!(res, vec![" language. Duct..."," what is duct?"]);
    }
}

