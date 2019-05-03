use std::env;
use std::process;
use minigrep;
use minigrep::Config;

fn main() {
      // 获取命令行参数
    let args:Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("some error:{}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(config) {
        eprintln!("App is error:{}",e);
        process::exit(1);
    }
}
