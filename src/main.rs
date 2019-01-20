use std::io::Result;

use structopt::StructOpt;

use self::command::Command;
use self::handler::handle;
// use rand::Rng;
// use std::cmp::Ordering;

mod command;
mod handler;
mod service;

struct User {
    id: i32,
    name: String,
    email: String,
    active: bool,
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColour(i32, i32, i32),
}

fn main() -> Result<()> {
    let s1 = String::from("who are u.|");
    //let (x,y) = multipleReturnVal(s1);
    //let x = reference_example(&s1);
    // mutable_ref_example(&mut s1);
    //let s1_1 = ownership_change();
    //about_slice();
    //let s2 = first_word(&s1);
    let s3 = s1;
    slice_show(&s3);

    // debug变量的宏
    println!("{}",s3);

    // todo list命令行版
    if false {
        handle(Command::from_args());
    }
    Ok(())

    // let y = {
    //     let x = 1;
    //     x+1
    // };
    // println!("y value is :{}", y);
    // if false {
    //     guess_number();
    // }
}

fn slice_show(s: &str) {
    let my_str1 = String::from("this is a slice example..");
    let word1 = &my_str1[..];
    let my_str2 = "this is another string ...";
    let word1 = first_word2(my_str2);
    println!("{}", word1);
}

//eg:传入字符串slice，返回字符串slice slice是字符串的部分引用
fn first_word2(s:&str)->&str{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate()  {
        if item == b' ' {
            return &s[0..i]
        }
    }
    &s[..]
}

fn first_word(s:&String)->&str{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate()  {
        if item == b' ' {
            return &s[0..i]
        }
    }
    &s[..]
}

// slice
fn about_slice(){
    let s1 = String::from("hello world!");
    let s1_1 = &s1[0..5];
    let s1_2 = &s1[4..];
    let s1_3 = &s1[4..];
    dbg!(s1_1);
    dbg!(s1_2);
    dbg!(s1_3);
}

// 所有权移出当前函数
fn ownership_change()->String{
    let s1 = String::from("this is a string...");
    s1
}

// 可变引用
fn mutable_ref_example(s:&mut String){
    //改变字符串 
    s.push_str("this is append text...\n");
}

//借用和引用
fn reference_example(s:&String)->usize{
    s.len()
}

// 多返回值问题
fn multipleReturnVal(s:String)->(String, usize){
    let x = s.len();
    (s, x)
}

// 多个字符串的复制（深拷贝）
fn copyDeeply() {
    let x = String::from("who are u.....");
    let x1 = x.clone();
    println!("the return value is:{}\n", x);
    println!("the return value is:{}\n", x1);
}

//语句和表达式的区别 
fn five()->i32{
    // 语句
    let a = 5+5;
    println!("a value is {}\n", a);
    // 表达式
    5+4
}

// fn guess_number() {
//     let mut res: u32;
//     loop {
//         println!("please input a number you guess:");
//         res = test2();
//         match res {
//             2 => {
//                 continue;
//             }
//             3 => break,
//             _ => continue,
//         }
//     }
// }

// fn test2() -> u32 {
//     let secret_number = 21;
//     let mut guess = String::new();
//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Fail to read line");
//     println!("the input is:{}", guess);
//     let guess: u32 = guess.trim().parse().expect("please input a number!");
//     match guess.cmp(&secret_number) {
//         Ordering::Equal => {
//             println!("this is equal...so you win!");
//             return 3;
//         }
//         Ordering::Greater => println!("this is greater"),
//         Ordering::Less => println!("this is less"),
//     }

//     return 2;
// }

// fn test1(){
//     println!("guess number...");
//     println!("please enter your number:");
//     let mut str = String::new();
//     io::stdin().read_line(&mut str).expect("something error...");
//     println!("You guess: {}", str);
// }
