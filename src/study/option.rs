#[derive(Debug)]
enum NameEnum {
    Samuel,
    Mark,
    Stephen
}

fn main() {
    println!("{:#?}", NameEnum::Samuel);
}

/*
## 枚举
* 在Rust中存在`enum`的数据类型，也就是枚举。
* 它表示一个常量(值)要么是某个值，要么什么都不是
* 它的定义方式如下：`enum typeName {value1,value2...}`
* 值的调用方式类似于关联函数：`typeName::value1`







*/
