fn main() {
    test_match();
    value_in_cents(En2::Quarter(En1::Alabama));
    if_let();
}

fn if_let(){
    let some_value = Some(2);
    if let Some(2) = some_value {
        println!("it is equal...");
    } else {
        println!("it is not equal...");
    }
}

// 测试 match运算符
#[allow(dead_code)]
fn test_match(){
    let s = String::from("hello");
    match s.len() {
        1 => {
            println!("the len is 1");
        },
        5 => {
            println!("the len is {}", 5);
        },
        _ => {
            println!("the len is unknow");
        },
    }
}

#[derive(Debug)]
enum En1 {
   Alaska,
   Alabama,
}

#[derive(Debug)]
enum En2 {
    Penny,
    Nickel,
    Dime,
    Quarter(En1),
}

fn value_in_cents(coin: En2) -> u32 {
    match coin {
        En2::Penny => 1,
        En2::Nickel => 5,
        En2::Dime => 10,
        En2::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

/*
## match匹配
* 它是一个极为强大的控制流运算符
* 当 match 表达式执行时，它将结果值按顺序与每一个分支的模式相比较。如果模式匹配了这个值，这个模式相关联的代码将被执行。如果模式并不匹配这个值，将继续执行下一个分支
* 每个分支相关联的代码是一个表达式，而表达式的结果值将作为整个 match 表达式的返回值。
* 每一个分支之间使用逗号分隔，如果分支代码较短的话通常不使用大括号
* 和`if`判断相比，有一个非常大的区别：对于`if`，表达式必须返回一个布尔值，而这里它可以是任何类型

### 通配符
* 使用`_`表示
* 通过将其放置于其他分支之后，`_`将会匹配所有之前没有指定的可能的值。

## 简单匹配
* match的匹配主要用于多个分支的匹配，当我们只需要进行一些简单的判断时，可以使用`if let`
* 可以在 `if let` 中包含一个 `else`。`else` 块中的代码与 `match` 表达式中的 `_` 分支块中的代码相同，这样的 `match` 表达式就等同于 `if let` 和 `else`

*/