use std::io::{self, Read};
use std::fs::File;

fn main() {
    // panic!("hello panic!");
    let res = read_a_file();
    println!("{:#?}", res);


    let res = return_err_spread_simply();
    println!("{:#?}", res);
}

// 简单读取一个文件并捕获错误向上传播
fn read_a_file() ->Result<String,io::Error> {
    let f = File::open("src/study/testCrate/testCrate.rs");
    let mut file = match f {
        Ok(file)=>file,
        Err(e) => return Err(e),
    };
    let mut str1 = String::new();
    match file.read_to_string(&mut str1) {
        Ok(_) => Ok(str1),
        Err(e) => Err(e),
    }
}

// 传播错误的简写方式
fn return_err_spread_simply() ->Result<String, io::Error> {
    let mut str1  = String::new();
    File::open("src/study/testCrate/testCrate.rs")?.read_to_string(&mut str1);
    Ok(str1)
}


/*
## panic错误
* 当程序发生错误时，可使用`panic!`宏打印相关的堆栈信息

```
fn main() {
    panic!("hello panic!");
}
```

* 当尝试运行这段代码时，结果提示报错信息：

```
thread 'main' panicked at 'hello panic!', src/study/panic.rs:2:5
note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
```

* 为了获得更加详细的信息，我们调整环境变量`RUST_BACKTRACE=1`
* 通过调用栈，找到你的代码，并修复错误。

### 错误的传播
* 当函数a中调用了另一个方法a1时，a1执行中，捕获了错误，并将错误返回给a，此时，a将错误继续向上返回给调用者，这种方式叫做错误传播。

#### 传播错误的简写方式
* 使用`?`

```
fn return_err_spread_simply() ->Result<String, io::Error> {
    let mut str1  = String::new();
    File::open("src/study/testCrate/testCrate.rs")?.read_to_string(&mut str1);
    Ok(str1)
}
```

### 何时使用panic和Result
* 首先需要知道，使用panic，就不能让程序继续执行了，也就是无法恢复。
* 我们可以选择使用panic，只是这样的话，就代替调用者决定了这是不可恢复的
* 就像是在PHP中，可以在函数或方法`func1()`中遇到错误并显示后，然后使用`exit();`进行退出，只是这样，调用者就无法知道调用你遇到错误直接退出，而无法执行它原本后续的逻辑代码，这样就显得很粗暴。
* 选择Result，就意味着将选择权交给了调用方，调用方决定是退出，还是显示异常信息后继续执行。


*/