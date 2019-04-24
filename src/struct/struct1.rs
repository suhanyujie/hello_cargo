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

    // 关联函数的调用
    let rect1 = Rectangle::get_rect(3);
    println!("{:#?}", rect1);

    println!("hello world...");
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height:u32,
}

// 为结构体实现方法
impl Rectangle {
    fn area(&self)->u32 {
        self.width * self.height
    }

    fn change_width(&mut self){
        self.width = self.width + 2;
    }
    #[allow(dead_code)]
    fn can_hold(&self, rect2: &Rectangle) -> bool {
        return self.width > rect2.width && self.height > rect2.height;
    }

    // 关联函数
    fn get_rect(size:u32) -> Rectangle {
        return Rectangle{width:size,height:size};
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
* 结构体有几大类，主要分为：类单元结构体，元组结构体，

### 元组结构体
* 元组结构体有着结构体名称提供的含义，但没有具体的字段名，只有字段的类型。

### 类单元结构体
* 我们也可以定义一个没有任何字段的结构体！它们被称为 类单元结构体（unit-like structs）因为它们类似于 ()，即 unit 类型。

### 方法
* 在结构体方法中，获取当前实例化使用`self`，如果要使用对应的数据，可以使用`&self`
* 如果对属性或者其他信息做修改，则需要使用`&mut self`
* Rust 有一个叫 自动引用和解引用（automatic referencing and dereferencing）的功能。其实Golang也有类似的功能“：自动解引用

### 关联函数
* 如果把PHP中的类比作Rust中的一个结构体，那么Rust中的关联函数就是PHP中的静态方法
* 关联函数的调用方式也是使用`::`运算符

### 总结
* 方法允许为结构体实例指定行为，而关联函数将特定功能置于结构体的命名空间中并且无需一个实例。

*/
