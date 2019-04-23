#[derive(Debug)]
struct User {
    name:String,
    age:i8,
    height:i16,
}

fn main(){
    let user1 = User {
        name:String::from("Samthon"),
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
    
    // 通过参数名和属性名一致，来简化实例化一个结构体
    let user1 = build_user(String::from("Mark"), 28);
    println!("mark struct is:{:?}\n", user1);

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

#[allow(dead_code)]
fn area(rec:Rectangle)->u32 {
    rec.width * rec.height
}

#[allow(dead_code)]
fn build_user(name:String, age:i8) -> User {
    User{
        name,
        age,
        height:170
    }
}

/*
## 结构体
* 定义结构体，需要使用 struct 关键字并为整个结构体提供一个名字。结构体的名字需要描述它所组合的数据的意义。
* 一旦定义了结构体后，为了使用它，通过为每个字段指定具体值来创建这个结构体的 实例

### 元组结构体
* 元组结构体有着结构体名称提供的含义，但没有具体的字段名，只有字段的类型。

### 类单元结构体
* 我们也可以定义一个没有任何字段的结构体！它们被称为 类单元结构体（unit-like structs）因为它们类似于 ()，即 unit 类型。


*/
