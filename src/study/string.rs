use std::string::String;

fn main() {
    // 字符串的创建：
    // println!("\n字符串的创建：");
    // create_string();
    // create_string2();
    // 字符串的拼接
    // println!("\n字符串的拼接：");
    // string_concat();
    // println!("\n通过push_str附加字符串：");
    // add_str_by_push_str();
    // println!("\n通过push附加字符：");
    // add_str_by_push();
    // println!("\n通过format!拼接字符串：");
    // concat_str_by_format();
    // println!("\n字符串slice：");
    // str_slice();
    // println!("\n字符串的遍历：");
    // foreach_str();
    // Rust 中一个字符的大小
    about_char();
}

// 方式1 新建字符串 `#[warn(unused_assignments)]`的作用是让编译器忽略一些warning
#[warn(unused_assignments)]
fn create_string() {
    let mut s = String::new();
    println!("{}", s);
    s = "hello".to_string();
    println!("{}", s);
}

// 方式2 创建字符串
fn create_string2() {
    let s = String::from("hello world...");
    println!("{}", s);
}

// 使用`+`运算符拼接字符串
fn string_concat() {
    let mut s = String::from("hello ");
    let s1 = String::from("world...");
    s = s + &s1;
    println!("{}", s);
}

// 通过push附加字符串
fn add_str_by_push_str() {
    let mut s = String::from("hello");
    s.push_str(" world");
    println!("{}", s);
}

// 通过push附加字符
fn add_str_by_push() {
    let mut s = String::from("ye");
    s.push('s');
    println!("{}", s);
}

// 通过format!宏拼接字符串
fn concat_str_by_format()  {
    let mut s1 = String::from("hello");
    let s2 = String::from(" ");
    let s3 = String::from("world");
    s1 = format!("打招呼：{}{}{}!", s1,s2,s3);
    println!("{}", s1);
}

// 字符串slice
fn str_slice()  {
    let s = String::from("你是谁");
    let sl1 = &s[0..=2];
    println!("{}", sl1);
}

// 遍历字符串
fn foreach_str() {
    let s = String::from("i think 这个有问题");
    for str in s.chars() {
        println!("{}", str);
    }
    println!("// ----------------------------------------");
    for str in s.bytes() {
        println!("{}", str);
    }
}

// https://stackoverflow.com/questions/21747136/how-do-i-print-the-type-of-a-variable-in-rust
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

// 字符相关
fn about_char() {
    let c1 = '国';
    let s1 = "国";
    print_type_of(&c1);
    println!("{}", std::mem::size_of::<char>());
    println!("{}", std::mem::size_of::<String>());
}

/*
## 字符串
* 字符串是 `UTF-8` 编码的
* `String` 是一个 `Vec<u8>` 的封装
* Rust中的字符串，不支持字符索引。如`s`是一个非空字符串：`let c1 = s[0];`这种写法是不允许的。
* 因为索引字符串通常是一个坏点子，因为字符串索引应该返回的类型是不明确的：字节值、字符、字形簇或者字符串 slice

### 创建字符串
#### 以 new 函数创建字符串
* 新建字符串变量，而后向其中装载内容，例如代码中`create_string()`函数

```
fn create_string() {
    let mut s = String::new();
    println!("{}", s);
    s = "hello".to_string();
    println!("{}", s);
}
```

#### 用`String::from()`创建字符串
* 通过方法调用来新建字符串，例如代码中的`create_string2()`

```
fn create_string2() {
    let s = String::from("hello world...");
    println!("{}", s);
}
```

### 更新字符串
#### 字符串的拼接
##### 使用运算符`+`
* `String`的大小可以增长其内容也可以改变，也就是常说的扩容
* 在Rust中，使用`+`运算符来进行字符串的拼接

##### 使用宏:`format!`
* 

##### 使用push_str/push附加字符串
* `push_str`可以向字符串后附加其他字符串

```
fn add_str_by_push_str() {
    let mut s = String::from("hello");
    s.push_str(" world");
    println!("{}", s);
}
```

* `push`可以向字符串后，追加单个字符

```
fn add_str_by_push() {
    let mut s = String::from("ye");
    s.push('s');
    println!("{}", s);
}
```

### 字符索引
* 实际上在Rust中，不允许对字符串进行字符索引，因为对于不同的字符集，返回对应的字符元素是不一样的，这样可能会导致问题。
* 但是有其他的解决方案

#### 字符串slice
* 使用 `[]` 和一个 `range` 来创建含特定字节的字符串 `slice`

```
fn str_slice()  {
    let s = String::from("你是谁");
    let sl1 = &s[0..=2];
    println!("{}", sl1);
}
```

* 这种使用方式得小心，如果使用`&s[0..=3]`,就会报panic，因为这样访问属于“无效索引”

#### 遍历字符串

```
fn foreach_str() {
    let s = String::from("i think 这个有问题");
    for str in s.chars() {
        println!("{}", str);
    }
}
```

* 其中使用`.chars()`方法是为了单独的 `Unicode` 标量值，可以返回`char`类型的值
* 除此之外，还有`.bytes()`方法

```
for str in s.bytes() {
    println!("{}", str);
}
```

这样使用时，得注意：有效的 `Unicode` 标量值可能会由不止一个字节组成。
Rust 中 char 类型占用的是 4 个字节，也就是一个 `Unicode` 标量值的大小。




*/