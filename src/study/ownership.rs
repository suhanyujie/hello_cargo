
fn main() {
    let s = String::from("hello world....");
    println!("{}", calculate_length(&s));
}


fn calculate_length(s: &String) -> usize {
    s.len()    
}


/*
## 所有权
* 所有权的规则有以下几点：
    * Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
    * 值有且只有一个所有者。
    * 当所有者（变量）离开作用域，这个值将被丢弃。
* 变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。

## 一些概念
* 两次释放（相同）内存会导致内存污染，它可能会导致潜在的安全漏洞。
* 像整型这样的在编译时已知大小的类型被整个存储在栈上，所以拷贝其实际的值是快速的


*/