#[derive(Debug)]
struct User {
    name:String,
    age:i8,
    height:i16,
}

fn main(){
    let user1 = User {
        name:String::from("suhanyu"),
        age:26,
        height:168,
    };
    //更新部分，其他值使用原先的值，使用语法`..user1`
    let user2 = User {
        name:String::from("shenshuo"),
        age:27,
        ..user1
    };
    println!("{}", user1.name);
    println!("{}", user2.height);

    println!("hello world...");
}
