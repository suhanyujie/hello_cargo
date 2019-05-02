use std::fmt::Display;

fn main() {
    let str1 = String::from("Adam is a management");
    let str2 = String::from("hello world aaaaaaaaaaaaaaaa");
    let res = lifetime_comment_for_func(&str1, &str2);
    println!("{:?}", res);
    // 部分参数使用生命周期注解
    let res = lifetime_comment_for_part_of_paramlist(&str1, &str2);
    println!("{:?}", res);
    // 结构体的生命周期
    test_struct_lifetime();
    // 结构体方法的生命周期注解
    let english_teacher1 = "Lendy";
    let stu1_desc = String::from("a good student.");
    let stu1 = Stu {
        name:String::from("Mark"),
        age:28,
        desc:&stu1_desc,
    };
    let res = stu1.study_english(&english_teacher1);
    println!("{}", res);
    // 综合使用 泛型 生命周期 trait bounds
    let x = String::from("hello");
    let y = String::from("world..");
    longest_with_an_announcement(&x, &y, &x);
}

// 函数使用生命周期注解
fn lifetime_comment_for_func<'a>(x:&'a str, y:&'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 部分参数有生命周期注解
fn lifetime_comment_for_part_of_paramlist<'a>(x:&'a str, y:&str) -> &'a str {
    x
}

// 结构体的生命周期注解
#[derive(Debug)]
struct ImportantExcept<'a> {
    part:&'a str,
}

fn test_struct_lifetime()  {
    let str1 = "hello world.";
    let st1 = ImportantExcept {
        part: &str1,
    };
    println!("{:?}", st1);
}

// 为结构体方法加上生命周期注解
#[derive(Debug)]
struct Stu<'a> {
    name: String,
    age:i32,
    desc:&'a str,
}

impl<'a> Stu<'a> {
    fn study_english(&self, teacher:&str) -> &str {
        println!("student {:?}'s english teacher is {}", self.name, teacher);
        &self.name
    }
}

// 综合使用泛型、声明周期和trait bounds
fn longest_with_an_announcement<'a,T>(x:&'a str,y:&'a str, ann:T) -> &'a str 
    where T:Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


/*
## 生命周期
### 前言
* 读完这章，需要知道以下问题和内容：
    * 什么是悬垂引用，如何避免？
    * 什么是输入生命周期，什么是输出生命周期？

### 生命周期注解表示方法
* 变量的生命周期的表示格式是单引号`'`后跟上对应的名称，例如：`'a`
* 生命周期注解一般用于变量引用，注解放在`&`操作符之后。如：`&'a str`
* 比如一个普通的i32变量引用的生命周期注解表示为`&'a i32`，或者`&'a mut i32`

#### 函数签名的生命周期注解

```
fn lifetime_comment_for_func<'a>(x:&'a str,y:&'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

* 如上面的实例代码，函数签名的注解就和泛型注解类似，放在“函数名”和“参数列表之间”的尖括号中
* 当具体的引用被传递给 `lifetime_comment_for_func` 时，被 'a 所替代的具体生命周期是 x 的作用域与 y 的作用域相重叠的那一部分。也就是x,y中生命周期较短的那一个！
* 参数列表中只有部分参数添加了生命周期注解的情况：

```
fn lifetime_comment_for_part_of_paramlist<'a>(x:'a str,y &str) -> &'a str {
    x
}
```

* 这个例子中，我们为参数 `x` 和返回值指定了生命周期参数 `'a`，不过没有为参数 `y` 指定，因为 `y` 的生命周期与参数 `x` 和返回值的生命周期没有任何关系

## 结构体中的生命周期
* 必须在结构体名称后面的尖括号中声明泛型生命周期参数，以便在结构体定义中使用生命周期参数
* 结构体中的生命周期注解如下：

```
#[derive(Debug)]
struct ImportantExcept<'a> {
    part:&'a str,
}

fn test_struct_lifetime()  {
    let str1 = "hello world.";
    let st1 = ImportantExcept {
        part: &str1,
    };
    println!("{:?}", st1);
}
```

* 这个注解意味着 `ImportantExcerpt` 的示例不能比其中的 `part` 字段中的引用存在的更久。

## 生命周期注解的省略
* 在编写了很多 Rust 代码后，Rust 团队发现在特定情况下 Rust 程序员们总是重复地编写一模一样的生命周期注解。这些场景是可预测的并且遵循几个明确的模式。接着 Rust 团队就把这些模式编码进了 Rust 编译器中，如此借用检查器在这些情况下就能推断出生命周期而不再强制程序员显式的增加注解。
* 这些被编码进 Rust 引用分析的模式被称为“生命周期省略规则”（lifetime elision rules）
* 函数或方法的参数的生命周期被称为 输入生命周期（input lifetimes），而返回值的生命周期被称为 输出生命周期（output lifetimes）。

## 方法定义中的生命周期注解

```
#[derive(Debug)]
struct Stu<'a> {
    name: String,
    age:i32,
    desc:&'a str,
}

impl<'a> Stu<'a> {
    fn study_english(&self, teacher:&str) -> &str {
        println!("student {:?}'s english teacher is {}", self.name, teacher);
        &self.name
    }
}
```

## 静态生命周期
* 生命周期中有一个“静态生命周期”的概念，其生命周期存活于整个程序期间。所有的字符串字面值都拥有 'static 生命周期
* 如果要标注一个字符串字面值的生命周期，可以如下所示：`let s:&'static str = "this is static lifetime";`

## 结合泛型、trait bounds和生命周期
* 实例代码如下：

```
fn longest_with_an_announcement<'a,T>(x:&'a str,y:&'a str, ann:T) -> &'a str 
    where T:Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

## 其他
* 每一个引用都有一个生命周期
* 生命周期是一种泛型



*/