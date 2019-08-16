fn main() {
    // test_if_pattern();

    // demo_while_pattern();

    // demo_for_pattern();

    // demo_let_deconstruct_tuple();

    // demo_fn_param_deconstruct_tuple();

    // pattern_for_literally();

    // pattern_for_var();
    // pattern_for_or();
    // pattern_for_3_dot();
    // pattern_for_3_dot_ascii();
    // pattern_for_deconstruct();
    // pattern_for_enum();
    // patterm_for_nest_enum();
    // pattern_for_deconstruct_ref();
    // pattern_for_deconstruct_struct();
    // pattern_for_ignore_value();
    // pattern_for_ignore_part_val_in_struct_data();
    // pattern_for_ignore_the_rest();
    // pattern_for_guard();
    // pattern_for_at();
    pattern_for_ref();
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

fn pattern_for_literally() {
    let x = 1;
    match x {
        0 => {
            println!("zero");
        },
        1 => {
            println!("one");
        },
        2 => {
            println!("two");
        },
        _ => {
            println!("other number");
        },
    }
}

fn pattern_for_var() {
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => {
            println!("has matched the value:Some(50)");
        },
        Some(y) => {
            println!("the value is {}", y);
        },
        _ => {
            println!("default case...");
        }
    }
}

fn  pattern_for_or () {
    let x = 2;
    match x {
        1 | 2 => {
            println!("the value is 1 or 2");
        },
        3 => {
            println!("the value is 3");
        },
        _ => {
            println!("the value is other");
        },
    }
}

fn pattern_for_3_dot () {
    let x = 12;

    match x {
        1...12 => {
            println!("the value range is 1 ~ 12");
        },
        _ => println!("the value range is other"),
    }
}

fn pattern_for_3_dot_ascii () {
    let x = 'c';

    match x {
        'a'...'c' => {
            println!("the value range is a ~ c");
        },
        _ => println!("the value range is other"),
    }
}

fn pattern_for_deconstruct() {
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point {x:0, y: 7};
    // 可以简写为 let Point{a, b} = p;
    let Point{x: a, y: b} = p;
    println!("{}-{}", a, b);
    match p {
        Point {x, y:0} => {
            println!("x:{}", x);
        },
        Point {x: 0, y} => {
            println!("y:{}", y);// y:7
        },
        Point {x, y} => {
            println!("{} - {}", x, y);
        },
    }
}

fn pattern_for_enum() {
    enum Message {
        Quit,
        Move{x: i32, y:i32},
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("the value type is Quit");
        },
        Message::Move{x, y} => {
            println!("the value type is Move {}-{}", x, y);
        },
        Message::ChangeColor(r, g, b) => {
            println!("the type is Message {}-{}-{}", r, g, b);
        },
    }
}

fn patterm_for_nest_enum() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    enum Message {
        Quit,
        Move{x: i32, y:i32},
        ChangeColor(Color),
        ChangeColor2(i32, i32, i32),
    }

    let msg = Message::ChangeColor(Color::Rgb(0, 160, 255));
    match msg {
        Message::Quit => {
            println!("the value type is Quit");
        },
        Message::Move{x, y} => {
            println!("the value type is Move {}-{}", x, y);
        },
        Message::ChangeColor2(r, g, b) => {
            println!("the type is ChangeColor-one {}-{}-{}", r, g, b);
        },
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("the type is ChangeColor-Color {}-{}-{}", r, g, b);
        },
        _ => {
            println!("other type");
        },
    }
}

fn pattern_for_deconstruct_ref() {
    struct Point {
        x: i32,
        y: i32,
    }

    let points = vec![
        Point{x: 0, y: 1},
        Point{x: 1, y: 1},
        Point{x: 2, y: -1},
        Point{x: -3, y: -1},
    ];
    // 闭包中的参数加上 `&`，使得调用时，使用的是 `iter()` 迭代的引用，而不是值本身
    let some_of_squares: i32 = points
            .iter()
            .map(|&Point{x, y}|x * x + y * y)
            .sum();
    println!("{}", some_of_squares);// 18
}

fn pattern_for_deconstruct_struct() {
    struct Point {
        x: i32,
        y: i32,
    }
    let ((feet, inches), Point{x, y}) = ((12, 15), Point {x: 3, y: -10});
    println!("{}-{}", feet, inches);
    println!("{}-{}", x, y);
}

fn pattern_for_ignore_value() {
    let func1 = |_: i32, y: i32| {
        println!("{}", y);
    };

    func1(1, 23);
}

fn pattern_for_ignore_part_val_in_struct_data() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("ignore some value");
        },
        _ => {
            setting_value = new_setting_value;
        },
    }
    println!("{:?}", setting_value);
}

fn pattern_for_ignore_the_rest() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin_point = Point {x: 0, y: 0, z: 0};
    match origin_point {
        Point {x, ..} => {
            println!("x value is {}", x);
        },
        _ => println!("other branch"),
    }
}

fn pattern_for_guard() {
    let v1 = Some(4);
    match v1 {
        Some(x) if x < 2 => {
            println!("it is less than 2");
        },
        Some(x) if x >= 2 => {
            println!("it is larger than 2");
        },
        _ => {
            println!("other state");
        },
    }
}

fn pattern_for_at() {
    enum Message {
        Hello {id: i32},
    }
    let msg = Message::Hello{id: 5};
    match msg {
        Message::Hello{id: id_var @ 1...5} => {
            println!("the range is 1~5 and val is {}", id_var);
        },
        Message::Hello{id: 6...7} => {
            println!("the range is 6~7");
        },
        Message::Hello{id} => {
            println!("the id val is {}", id);
        },
    }
}

fn pattern_for_ref() {
    let robot_name = &Some(String::from("Wali"));
    match robot_name {
        &Some(ref name) => println!("{}", name),
        _ => println!("other option"),
    }
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

## 模式的可反驳性和不可反驳性
* 模式有两种形式：refutable（可反驳的）和 irrefutable（不可反驳的）。能匹配任何传递的可能值的模式被称为是 不可反驳的（irrefutable）。一个例子就是 let x = 5; 语句中的 x，因为 x 可以匹配任何值所以不可能会失败。对某些可能的值进行匹配会失败的模式被称为是 可反驳的（refutable）。一个这样的例子便是 if let Some(x) = a_value 表达式中的 Some(x)；如果变量 a_value 中的值是 None 而不是 Some，那么 Some(x) 模式不能匹配。

## 使用 `if let` 解决可反驳情况
* 对于一些可反驳模式的场景，可使用 `if let` 防止错误：

```rust
if let Some(x) = some_option_value {

}
```

* 注意一点，我们可以将反驳模式用在 `if let` 上，但是不能将不可反驳模式用在 `if let` 上，否则，编译会给出错误提示。因为不可反驳模式用于 `if let` 是没有意义的。

## 模式语法的种类
### 匹配字面值
* 如大多数语言中的 switch 一样，Rust 中的 match 可以匹配字面值：

```rust
fn pattern_for_literally() {
    let x = 1;
    match x {
        0 => {
            println!("zero");
        },
        1 => {
            println!("one");
        },
        2 => {
            println!("two");
        },
        _ => {
            println!("other number");
        },
    }
}
```

### 匹配命名变量
* 示例如下：

```rust
fn pattern_for_var() {
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => {
            println!("has matched the value:Some(50)");
        },
        Some(y) => {
            println!("the value is {}", y);
        },
        _ => {
            println!("default case...");
        }
    }
}
```

* 上方示例输出的是 `the value is 5`。通过示例，我们可以知道，`Some(y)` 中的 `y` 将匹配 x 的 `Some` 中的值。这里需要注意的是，从 match 开始，就是一个新的作用域。match 内部的 y 和 match 上方的 y 是不同的。

### 多个模式
* 通过在 match 的 case 中使用 `|` 语法，可以进行形如逻辑或的运算，如：

```rust
fn  pattern_for_or () {
    let x = 2;
    match x {
        1 | 2 => {
            println!("the value is 1 or 2");
        },
        3 => {
            println!("the value is 3");
        },
        _ => {
            println!("the value is other");
        },
    }
}
```

### 通过 `...` 匹配值的范围
* Rust 的 match 的 case 中，可以使用 `...` 的语法：

```rust
fn pattern_for_3_dot () {
    let x = 12;

    match x {
        1...12 => {
            println!("the value range is 1 ~ 12");
        },
        _ => println!("the value range is other"),
    }
}
```

* 并且匹配的是 `1<=x<=12`
* 除此之外，还可以使用 ASCII 字符来指定范围：

### 解构结构体
* 模式匹配可以解构结构体：

```rust
fn pattern_for_deconstruct() {
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point {x:0, y: 7};
    // 可以简写为 let Point{a, b} = p;
    let Point{x: a, y: b} = p;
    println!("{}-{}", a, b);
}
```

* 而在 match 中，有更加有趣的特性，如下：

```rust
match p {
    Point {x, y:0} => {
        println!("x:{}", x);
    },
    Point {x: 0, y} => {
        println!("y:{}", y);// y:7
    },
    Point {x, y} => {
        println!("{} - {}", x, y);
    },
}
```

### 解构枚举
* 不仅可以对结构体解构，还能对枚举类型的值解构：

```rust
fn pattern_for_enum() {
    enum Message {
        Quit,
        Move{x: i32, y:i32},
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("the value type is Quit");
        },
        Message::Move{x, y} => {
            println!("the value type is Move {}-{}", x, y);
        },
        Message::ChangeColor(r, g, b) => {
            println!("the type is Message {}-{}-{}", r, g, b);
        },
    }
}
```

### 解构嵌套的结构体或枚举
* 示例代码如下：

```rust
fn patterm_for_nest_enum() {
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    enum Message {
        Quit,
        Move{x: i32, y:i32},
        ChangeColor(Color),
        ChangeColor2(i32, i32, i32),
    }

    let msg = Message::ChangeColor(Color::Rgb(0, 160, 255));
    match msg {
        Message::Quit => {
            println!("the value type is Quit");
        },
        Message::Move{x, y} => {
            println!("the value type is Move {}-{}", x, y);
        },
        Message::ChangeColor2(r, g, b) => {
            println!("the type is ChangeColor-one {}-{}-{}", r, g, b);
        },
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("the type is ChangeColor-Color {}-{}-{}", r, g, b);
        },
        _ => {
            println!("other type");
        },
    }
}
```

### 解构引用
* 当模式所匹配的值中包含引用时，需要解构引用之中的值，可以通过在模式中指定 `&` 来实现：

```rust
fn pattern_for_deconstruct_ref() {
    struct Point {
        x: i32,
        y: i32,
    }

    let points = vec![
        Point{x: 0, y: 1},
        Point{x: 1, y: 1},
        Point{x: 2, y: -1},
        Point{x: -3, y: -1},
    ];
    // 闭包中的参数加上 `&`，使得调用时，使用的是 `iter()` 迭代的引用，而不是值本身
    let some_of_squares: i32 = points
            .iter()
            .map(|&Point{x, y}|x * x + y * y)
            .sum();
    println!("{}", some_of_squares);// 18
}
```

### 解构结构体和元组
* 示例代码如下：

```rust
fn pattern_for_deconstruct_struct() {
    struct Point {
        x: i32,
        y: i32,
    }
    let ((feet, inches), Point{x, y}) = ((12, 15), Point {x: 3, y: -10});
    println!("{}-{}", feet, inches);
    println!("{}-{}", x, y);
}
```

### 忽略模式中的值
#### 忽略整个值
* Rust 中的 `_` 标识符具有很大的作用，作用之一是忽略“模式匹配中的值”

```rust
fn pattern_for_ignore_value() {
    let func1 = |_: i32, y: i32| {
        println!("{}", y);
    };

    func1(1, 23);
}
```

#### 在嵌套的数据中忽略部分值
* 示例代码如下：

```rust
fn pattern_for_ignore_part_val_in_struct_data() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("ignore some value");
        },
        _ => {
            setting_value = new_setting_value;
        },
    }
    println!("{:?}", setting_value);
}
```

### 通过 `_` 忽略未使用的变量
* 例如 `let _x = 5;`
* 相比于只是用 `_` 的情况： `let _ = 5;`，`_x` 仍会将值绑定到变量；`_` 则完全不会绑定。

### 用 `..` 忽略剩余值
* 在模式匹配的语法结构中，可以用 `..` 来忽略部分值甚至所有值：

```rust
fn pattern_for_ignore_the_rest() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin_point = Point {x: 0, y: 0, z: 0};
    match origin_point {
        // 这里匹配的必须用 x，即必须是“字段名”
        Point {x, ..} => {
            println!("x value is {}", x);
        },
        _ => println!("other branch"),
    }
}
```

* 需要注意的是使用 `..` 必须是无歧义的

### 匹配守卫
* 通俗点理解，就是在 match 的模式分支上加上 if 语句，以此作为“匹配成功”的守卫

```rust
fn pattern_for_guard() {
    let v1 = Some(4);
    match v1 {
        Some(x) if x < 2 => {
            println!("it is less than 2");
        },
        Some(x) if x >= 2 => {
            println!("it is larger than 2");
        },
        _ => {
            println!("other state");
        },
    }
}
```

* 除此之外，还可以对形如 `1 | 5 | 6` 的模式进行守卫。

### `@` 绑定
* at 运算符（`@`）允许我们在创建一个存放值的变量的同时测试其值是否匹配模式。

```rust
fn pattern_for_at() {
    enum Message {
        Hello {id: i32},
    }
    let msg = Message::Hello{id: 5};
    match msg {
        Message::Hello{id: id_var @ 1...5} => {
            println!("the range is 1~5 and val is {}", id_var);
        },
        Message::Hello{id: 6...7} => {
            println!("the range is 6~7");
        },
        Message::Hello{id} => {
            println!("the id val is {}", id);
        },
    }
}
```

* 通过上面的实例，可以了解，使用 `@` 可以在一个模式中同时检测值以及保存值到变量中。

### 遗留模式
* 该模式的场景是：当匹配的模式是引用时，并且模式中的数据尝试进行绑定时

```rust
fn pattern_for_ref() {
    let robot_name = &Some(String::from("Wali"));
    match robot_name {
        &Some(ref name) => println!("{}", name),
        _ => println!("other option"),
    }
}
```


*/