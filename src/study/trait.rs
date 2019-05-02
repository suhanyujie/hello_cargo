use std::fmt::{Display,Formatter};

// 定义trait
pub trait Summary {
    fn summarize(&self)->String;

    fn summarize_2(&self)->String {
        String::from("(read more...)")
    }
}

// 声明struct类型
#[derive(Debug)]
pub struct Tweet {
    pub name:String,
    pub content:String,
}

// 为Tweet实现trait
impl Summary for Tweet {
    fn summarize (&self) ->String {
        format!("{}:{}", self.name, self.content)
    }
}

// impl Display for Tweet {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         println!("\n{:?}", self);
       
//     }
// }

// blanket implementation 为实现了`Summary` trait的类型来实现`ToString` trait
pub trait MyToSomething {
    fn to_string();
}
impl<T:Summary> MyToSomething for T {
    fn to_string() {
        println!("this is blanket implementation for Summary");
    }
}

fn main() {
    // 声明一个Tweet类型的变量绑定 
    let t1 = Tweet {
        name:String::from("Mark"),
        content:String::from("this is content"),
    };
    println!("\n{:?}", t1.summarize());
    // trait作为参数的方式调用
    let res = type_limit_for_trait(&t1);
    println!("{:?}", res);
    //  trait bounds函数调用
    use_trait_bounds(&t1);
    // 多个trait的trait bounds (暂时不能运行)
    // use_trait_bounds_mul(&t1);

    // 返回trait (暂时不能运行 因为涉及到一些后续的内容)
    // let t2 = return_by_trait();
    // println!("\n{:#?}", t2);

    let list1 = vec![10,90,12,76,109,87,90,86,125,980,908,799,101];
    let largest = largest_in_fancy(&list1);
    println!("the largest data is {}", largest);

    //` blacket implementation`的调用
    Tweet::to_string();
}

// trait作为参数的方式
fn type_limit_for_trait(t1:&impl Summary) ->String {
    t1.summarize()
}

// trait bounds函数
pub fn use_trait_bounds<T:Summary>(item:&T) {
    println!("\n{:?}", item.summarize());
}

pub fn use_trait_bounds_mul<T:Summary+Display>(item:&T){
    println!("\n{:?}", item.summarize());
}

fn return_by_trait() -> impl Summary {
    Tweet {
        name: String::from("Adam"),
        content: String::from("Adam is a management..."),
    }
}

// 使用泛型和`trait bounds`定义函数 获取最大值
fn largest_in_fancy<T:Copy+PartialOrd>(list:&[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}



/*
## trait
* 它的作用是实现某些目的所必需的行为（方法）的集合。
* `trait` 体中可以有多个方法：一行一个方法签名且都以分号结尾。
* `trait`如果需要被其他库引用，则需要添加`pub`关键字，如下：

```
pub trait Summary {
    fn summarize()->String;
}
```

* 当然，也可以在定义trait中方法的默认行为：

```
pub trait Summary {
    fn summarize(&self)->String;

    fn summarize_2(&self)->String {
        String::from("this is default string content...")
    }
}
```

* 为一个struct类型实现trait，使用关键字`impl SomeTrait for SomeStruct`，例如：

```
impl Summary for Tweet {
    fn summarize (&self) ->String {
        format!({}:{}, self.name,self.content);
    }
}
```

* 如果一个trait中的某个方法已经有默认实现了，此时在为struct实现trait时，可以重新实现这个方法，并且此时重载一个默认实现的语法与实现没有默认实现的 trait 方法时完全一样的。

## 使用`trait`作为参数
* 在Rust中可以使用`impl Trait`作为类型限定的作用。并且 `impl Trait` 语法适用于短小的例子。例如：

```
fn type_limit_for_trait(t1:impl Summary) ->String {
    t1.summarize()
}
```

* 还可以指定多个trait，使用`+`操作符，如:`fn type_limit_for_trait(t1:impl Summary + Display) ->String {`


### trait bounds
* `impl Trait`的使用结合上泛型，就是trait bounds，它的格式如下：

```
pub fn use_trait_bounds<T:Summary>(item:&T) {
    println!("\n{:?}", item.summarize());
}
```

* 针对多个trait的trait bounds传参方式：

```
pub fn use_trait_bounds_mul<T:Summary+Display>(item:&T){
    println!("\n{:?}", item.summarize());
}
```

### where从句
* 为了针对多个`trait bounds`导致方法签名阅读不友好，可以使用where的方式，使代码更利于阅读：

```
pub fn use_trait_bounds_mul<T:Summary+Display,U:Display>(t:T,u:U)
    where T:Summary+Display,
          U:Display
{
    // todo
}
```

## 返回trait
* 就像在参数传递中使用`impl Trait`，在返回值中也能使用`impl Trait`

```
fn return_by_trait() -> impl Summary {
    Tweet {
        name: String::from("Adam"),
        content: String::from("Adam is a management..."),
    }
}
```

## 回到defineFuncByFancy.rs
* 回到`defineFuncByFancy.rs`文件中，我们利用泛型的impl Trait来编写 `largest_in_fancy` 函数

```
fn largest_in_fancy<T:Copy+PartialOrd>(list:&[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

## blacket implementation
* 对任何满足特定`trait bound`的类型实现trait的这种行为被称为`blanket implementation`

## 注意事项
* trait在实现时，签名要和声明trait时的签名保持一致，包括参数声明
* 还有一种泛型，我们一直在使用它甚至都没有察觉它的存在，这就是 生命周期（lifetimes）。---在Rust中，生命周期是一种泛型


*/