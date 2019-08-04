
pub trait Draw {
    fn draw(&self);
}

// pub struct ScreenOld1 {
//     pub components: Vec<Box<dyn Draw>>,
// }

#[derive(Debug)]
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T> 
    where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// 声明一个结构体 Button
#[derive(Debug)]
struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self){
        println!("this is a button...");
    }
}

fn show_button() {
    let screen = Screen {
        components: vec![
            Button{
                width: 210,
                height: 60,
                label: String::from("submit"),
            },
        ],
    };
    screen.run();
}

#[derive(Debug)]
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

#[derive(Debug)]
pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft{})),
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post:&'a Post) ->&'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview{})
    }

    fn approve (self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published{})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) ->Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

fn demo_for_post() {
    let post = Post::new();
}

fn main() {
    // show_button();
    demo_for_post();
}

/*
## 面向对象语言的特征
* 面向对象编程语言所共享的一些特性往往是对象、封装和继承。

### 面向对象的一个定义
>面向对象的程序是由对象组成的。一个 对象 包含数据和操作这些数据的过程。这些过程通常被称为 方法 或 操作。

* Rust 中，结构体和枚举包含数据而 impl 块提供了在结构体和枚举之上的方法。虽然带有方法的结构体和枚举并不被 称为 对象，但是他们提供了与对象相同的功能
* 使用 pub 关键字来决定模块、类型、函数和方法是公有的，而默认情况下其他一切都是私有的。

### 封装
* 对象的实现细节不能被使用对象的代码获取到。所以唯一与对象交互的方式是通过对象提供的公有 API；使用对象的代码无法深入到对象内部并直接改变数据或者行为。封装使得改变和重构对象的内部时无需改变使用对象的代码。
* Rust 中可以很好的通过声明结构体以及对应的 impl 块实现封装行为。

### 继承
* 一个对象可以定义为继承另一个对象的定义，这使其可以获得父对象的数据和行为，而无需重新定义。
* 选择继承有两个主要的原因。第一个是为了重用代码：一旦为一个类型实现了特定行为，继承可以对一个不同的类型重用这个实现。
* 第二个使用继承的原因与类型系统有关：表现为子类型可以用于父类型被使用的地方。这也被称为 多态（polymorphism），这意味着如果多种对象共享特定的属性，则可以相互替代使用。
* Rust 中没有明确的继承。因为在很多有继承特性的编程语言中，继承有着很大的局限性。因而，Rust 中使用 trait 对象来替代继承。


*/