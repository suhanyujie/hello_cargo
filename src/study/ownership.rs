fn main() {
    let s = String::from("hello world....");
    println!("{}", calculate_length(&s));
    
    // let res = get_first_word(&s);
    // println!("{}", res);

fn calculate_length(s: &String) -> usize {
    s.len()    
}

// 返回字符串中的第一个单词的位置
#[test]
fn get_first_word_size(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// 可变引用
fn ref_mutable(){
    let mut s1 = String::from("hello1");
    calculate_length(&s1);
    {
        let s2 = &s1;
        s2 = s2 + "123";
        println!("{}",s2);
    }
    println!("{}",s1);
}

// 字符串slice
#[allow(dead_code)]
fn string_slice() {
    let s = String::from("hello world sam");
    // 包含end
    let s1 = &s[0..=1];
    // 不包含end   如果想要从第一个索引（0）开始，可以不写两个点号之前的值
    let s2 = &s[0..1];
    let s3 = &s[..1];
    println!("string slice s1 is:{}\ns2 is :{}\n", s1, s2);
    println!("s3 is:{}\n", s3);
}

// 返回字符串中的第一个单词
fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..=i];
        }
    }

    &s[..]
}

/*
## 所有权
* 所有权的规则有以下几点：
    * Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
    * 值有且只有一个所有者。
    * 当所有者（变量）离开作用域，这个值将被丢弃。
* 变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。
* 不能在拥有不可变引用的同时拥有可变引用。

## 引用和借用
* 可以使用大括号来创建一个新的作用域，以允许拥有多个可变引用，只是不能 同时 拥有
* 不能在拥有不可变引用的同时拥有可变引用
* “字符串 slice” 的类型声明写作 `&str`，它是一个`不可变引用`

### 总结
* 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
* 引用必须总是有效。

### 悬垂指针
* 所谓悬垂指针是其指向的内存可能已经被分配给其它持有者

## 一些概念
* 两次释放（相同）内存会导致内存污染，它可能会导致潜在的安全漏洞。
* 像整型这样的在编译时已知大小的类型被整个存储在栈上，所以拷贝其实际的值是快速的

*/