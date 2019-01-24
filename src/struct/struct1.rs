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
    let mut rec = Rectangle {
        width:21,
        height:19,
    };
    println!("{:#?}", rec);
    println!("1 the rectangle area is :{}\n", rec.area());
    // 改变rectangle的属性
    rec.change_width();
    println!("2 the rectangle area is :{}\n", rec.area());
    

    println!("hello world...");
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height:u32,
}

impl Rectangle {
    fn area(&self)->u32 {
        self.width * self.height
    }

    fn change_width(&mut self){
        self.width = self.width + 2;
    }
}

fn area(rec:Rectangle)->u32 {
    rec.width * rec.height
}
