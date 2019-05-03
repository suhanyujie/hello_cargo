use std::thread;
use std::time::Duration;

let expensive_closure = |num:u32| ->u32 {
    println!("calculate ...")    
    thread::sleep(Duration::from_secs(2);
    num
};

// 省略大括号的写法
let add_one_v1 = |num| num+1;

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

fn generate_workout(intensity:u32, random_number:u32)  {
    if intensity <25 {
        println!("Today,do {} pushups", simulated_expensive_cal(intensity));
    } else {
        if random_number == 3 {
            println!("take a break today, Remember to stay hydrated.");
        } else {
            println!("Today, run for {} minutes", simulated_expensive_cal(intensity))
        }
    }
}

fn simulated_expensive_cal(intensity:u32) ->u32 {
    println!("cal slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// 使用闭包重构 generate_workout
fn generate_workout_2(intensity:u32, random_number:u32)  {
    if intensity <25 {
        println!("Today,do {} pushups", expensive_closure(intensity));
        println!("Next,do {} situp", expensive_closure(intensity))
    } else {
        if random_number == 3 {
            println!("take a break today, Remember to stay hydrated.");
        } else {
            println!("Today, run for {} minutes", expensive_closure(intensity))
        }
    }
}

#[derive(Debug)]
struct Cacher<T> 
    where T:Fn(u32)->u32
{
    calculation:T,
    value:Option<u32>,
}



/*
## 闭包
* Rust 的 闭包（closures）是可以保存进变量或作为参数传递给其他函数的匿名函数。

### 闭包的定义
* 闭包的定义以一对竖线（|）开始，在竖线中指定闭包的参数；之所以选择这个语法是因为它与 Smalltalk 和 Ruby 的闭包定义类似。这个闭包有一个参数 num；如果有多于一个参数，可以使用逗号分隔，比如 |param1, param2|
* 参数之后是存放闭包体的大括号 —— 如果闭包体只有一行则大括号是可以省略的。大括号之后闭包的结尾，需要用于 let 语句的分号。
* 当然，还可以为闭包增加必要的类型注解：

```
let expensive_closure = |num:u32| ->u32 {
    println!("calculate ...")    
    thread::sleep(Duration::from_secs(2);
    num
};
```
* 省略大括号的闭包写法：

```
let add_one_v1 = |num| num+1;
```

### 带有泛型的闭包、结构体闭包
* 每一个闭包实例有其自己独有的匿名类型：也就是说，即便两个闭包有着相同的签名，他们的类型仍然可以被认为是不同。
* 所有的闭包都实现了 trait `Fn`、`FnMut` 或 `FnOnce` 中的一个。
* 一个结构体含有闭包如下所示：

```
#[derive(Debug)]
struct Cacher<T> 
    where T:Fn(u32)->u32
{
    calculation:T,
    value:Option<u32>,
}
```






*/