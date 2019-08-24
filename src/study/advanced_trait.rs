use std::ops::Add;

fn main() {
    // test_overload_op();

    test_fully_qualified_syntax();
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type  Output = Point;

    fn add(self, other: Point) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn test_overload_op() {
    let p1 = Point {
        x: 1,
        y: 2,
    };
    let p2 = Point {
        x: 1,
        y: 2,
    };

    let p1 = p1 + p2;
    println!("{:#?}", p1);
}

struct Stu {
    name: String,
}

trait Human {
    fn say();
}

impl Stu {
    fn say() {
        println!("hi I'm a student");
    }
}

impl Human for Stu {
    fn say() {
        println!("hi I'm a human");
    }
}

fn test_fully_qualified_syntax() {
    Stu::say();
    // 完全限定语法调用方式
    <Stu as Human>::say();
}




/*
## 高级 trait
### 默认泛型类型
* 我们知道在很多编程语言中，函数声明时，可以对函数参数指定默认值。对于 Rust，在用泛型表示一种类型时，我们可以指定该泛型的默认类型：`<PlaceholderType=ConcreteType>`

### 运算符重载
* Rust 中不允许对任意的运算符重载，但是可以针对 `std::ops` 中指定的 trait 和“运算符”进行重载

### 完全限定语法
* 完全限定语法定义为：

```rust
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

* 示例代码如下：

```rust
struct Stu {
    name: String,
}

trait Human {
    fn say();
}

impl Stu {
    fn say() {
        println!("hi I'm a student");
    }
}

impl Human for Stu {
    fn say() {
        println!("hi I'm a human");
    }
}

fn test_fully_qualified_syntax() {
    Stu::say();
    // 完全限定语法调用方式
    <Stu as Human>::say();
}
```

### 父 trait 用于在另一个 trait/trait 依赖
* 在实际使用 trait 时， 



*/
