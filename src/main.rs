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
    let x = copyDeeply();

    dbg!(x);

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
