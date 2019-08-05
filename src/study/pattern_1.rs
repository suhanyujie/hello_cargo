fn main() {
    // test_if_pattern();

    // demo_while_pattern();

    // demo_for_pattern();

    // demo_let_deconstruct_tuple();

    demo_fn_param_deconstruct_tuple();
}   

fn test_if_pattern() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("today is not Tuesday");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn demo_while_pattern() {
    let mut stack = vec!['a', 'b', 'c'];
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn demo_for_pattern() {
    let mut stack = vec!['a', 'b', 'c'];
    // 使用 enumerate 方法适配一个迭代器来产生一个值和其在迭代器中的索引
    for (index, value) in stack.iter().enumerate() {
        println!("{} -- {}", index, value);
    }
}

/// let 解构元组
fn demo_let_deconstruct_tuple() {
    let (x, y, z) = (1,2,3);
    println!("{} - {} - {}", x, y, z);
}

/// 函数参数解构
fn demo_fn_param_deconstruct_tuple() {
    let point = (1,90);
    demo_fn(&point);
}

fn demo_fn(&(x,y): &(u32, u32)) {
    println!("{} -- {}", x, y);
}

/*
## 模式匹配
* _模式_ 是 Rust 中的特殊语法。用于匹配类型中的结构。简单的讲，就是 `match` 的使用。
* match 有点类似于 PHP 中的 switch。只不过，match 的功能更强大、更灵活。“模式匹配”中的“模式”就好比 switch 中 case 的值，它可以由这些内容组成：
    * 字面值
    * 解构的数组、枚举、结构体或元组
    * 变量
    * 通配符
    * 占位符

* 模式后跟随的是处理的程序逻辑。 

### match 语法
* match 表达式由 match 关键字、匹配的值以及若干分支。形如下方：

```other
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

* Rust 中规定，match 表达式**必须**是穷尽的。因此，往往最后一个匹配分支需要涵盖剩下的所有情况。有一个特定的模式 `_` 可以匹配所有情况，不过它从不绑定任何变量。

### 灵活的 `if let`
* 相比于 match 的一次只匹配一个值，`if let` 语法可以更加灵活的在不同的分支中匹配不同的值，如下：

```rust
fn test_if_pattern() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("today is not Tuesday");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
```

* 注意一点，`if let` 表达式的缺点在于其穷尽性没有为编译器所检查，而 match 表达式则检查了
* `if let` 方式很灵活。但我个人不建议使用，在实际的业务中，我们往往是对某一个值进行匹配，在同一个 if 语句中，对不同的参数判断，个人感觉不利于代码的阅读，容易把业务逻辑写乱。

### while let 模式
* 除了 `if let` 的匹配之外，类似的还有 `while let`：

```rust
fn demo_while_pattern() {
    let mut stack = vec!['a', 'b', 'c'];
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
```

### for 循环的模式
* for 循环中，可以通过解构来获取一个模式

```rust
fn demo_for_pattern() {
    let mut stack = vec!['a', 'b', 'c'];
    // 使用 enumerate 方法适配一个迭代器来产生一个值和其在迭代器中的索引
    for (index, value) in stack.iter().enumerate() {
        println!("{} -- {}", index, value);
    }
}
```

### let 语句
* 其实， `let` 本身就是一种模式。它的一个原型是：

```other
let PATTERN = EXPRESSION;
```

* 这样的语句中变量名位于 PATTERN 位置，变量名不过是形式特别朴素的模式。所以例如 let x = 5; 的情况，x 是一个模式代表 “将匹配到的值绑定到变量 x”。同时因为名称 x 是整个模式，这个模式实际上等于 “将任何值绑定到变量 x，不管值是什么”。
* 除此之外，还有一个典型的例子就是 `let` 解构元组：

```rust
fn demo_let_deconstruct_tuple() {
    let (x, y, z) = (1,2,3);
    println!("{} - {} - {}", x, y, z);
}
```

### 函数参数的模式
* 关于上方提到的元组，在调用函数时，参数可以传递元素，函数的形参会自动解构：

```rust
fn demo_fn_param_deconstruct_tuple() {
    let point = (1,90);
    demo_fn(&point);
}

fn demo_fn(&(x,y): &(u32, u32)) {
    println!("{} -- {}", x, y);
}
```

* 注意一下形参的写法：`&(x,y): &(u32, u32)`


*/