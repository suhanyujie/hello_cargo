
fn main() {
    println!("Hello, world!");
}

#[test]
fn test_for_test()  {
    println!("Hello, test!");
}

#[test]
fn test_for_bad()  {
    assert_eq!(2+2, 5);
}

#[test]
fn test_for_panic()  {
    if true {
        panic!("some error.");
    }
}

// 使用`assert!`宏
#[derive(Debug)]
struct Rectangle {
    width: i32,
    height:i32,
}

impl Rectangle {
    fn can_hold(&self, rec:&Rectangle) -> bool {
        self.width >= rec.width && self.height >= rec.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {width:10,height:8,};
        let one_rec = Rectangle {width:7,height:4,};
        assert!(larger.can_hold(&one_rec));
    }
}

// 使用assert_eq!宏进行判等
fn add_three(a:i32) -> i32 {
    a+3
}

#[test]
fn is_equal() {
    assert_eq!(4, add_three(1));
}

#[test]
#[ignore]
fn is_not_equal()  {
    assert_ne!(4, add_three(4));
}

// 用`assert!`自定义失败信息
#[test]
fn test_for_define_tips() {
    let res = add_three(4);
    assert!(res==6 , "error.the actual value is {}", res);
}

// 用should_panic注解
#[derive(Debug)]
struct Guess {
    value:i32,
}

impl Guess {
    fn new(value:i32) -> Guess {
        if value > 100 || value < 1 {
            panic!("the value must between 1 and 100! got value {}.", value);
        }
        Guess {
            value,
        }
    }
}

// 测试使用should_panic注解
#[test]
#[should_panic]
fn test_should_panic() {
    let gs = Guess::new(302);
    println!("{:#?}", gs);
}

#[test]
#[should_panic(expected="the value must between 1 and 100!")]
fn test_attr_of_should_panic() {
    let gs = Guess::new(302);
    println!("{:#?}", gs);
}

// 将`Rersult<(), String>`用于测试（此时不能再使用`Result<>`）
#[test]
fn test_use_result() -> Result<(), String> {
    if 2 + 2 == 5 {
        Ok(())
    } else {
        Err(String::from("add value is error."))
    }
}

/*
## 测试
### 测试命令
* 一般使用 `cargo test`命令来进行测试
* 可以运行`cargo test --help`和`cargo test -- --help`来获取相关命令的帮助信息。 
* 运行单个测试：`cargo test {funcName}`，例如`cargo test test_use_result`
* 运行部分测试：`cargo test {partFuncName}`(函数名的一部分)，此时程序会筛选出含有关键词`partFuncName`的函数(测试)进行运行，例如`cargo test is_`

### 标记注解
* 在rust代码中，我们可以在函数前使用注解`#[test]`表示该函数是测试函数
* 测试通过时，所有输出到标准输出的内容不会打印出来，只会显示测试通过
* 测试未通过时，所有输出到标准输出的内容会输出到标准输出，并显示对应的错误信息

### 测试函数的作用
* 大致有3种作用：
    * 设置任何所需的数据或状态
    * 运行需要测试的代码
    * 断言其结果是我们所期望的
* 可以通过宏`assert_eq!`宏来进行断言

```
#[test]
fn test_for_bad()  {
    assert_eq!(2+2, 3);
}
```

### 使用panic!宏
* 断言不正确时，可以触发panic：

```
#[test]
fn test_for_panic()  {
    if true {
        panic!("some error.");
    }
}
```

### 使用`assert!`宏
* `assert!()`宏的第一个参数，类型是`bool`，如果为`false`，则会出发`panic`，如果为`true`，则断言通过
* 一个例子：

```
#[derive(Debug)]
struct Rectangle {
    width: i32,
    height:i32,
}

impl Rectangle {
    fn can_hold(&self, rec:&Rectangle) -> bool {
        self.width >= rec.width && self.height >= rec.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {width:10,height:8,};
        let one_rec = Rectangle {width:7,height:4,};
        assert!(larger.can_hold(&one_rec));
    }
}
```

* `larger_can_hold_smaller`中用到了`Rectangle`结构体，但是涉及到访问权限问题，所以需要使用`use super::*;`

### 使用`assert_eq!`和`assert_ne!`宏进行判等操作

```
fn add_three(a:i32) -> i32 {
    a+3
}

#[test]
fn is_equal() {
    assert_eq!(4, add_three(1));
}

#[test]
fn is_not_equal()  {
    assert_ne!(4, add_three(4));
}
```

* `assert_ne!` 宏在传递给它的两个值不相等时通过，而在相等时失败
* `assert_eq!` 和 `assert_ne!` 宏在底层分别使用了 `==` 和 `!=`。当断言失败时，这些宏会使用调试格式打印出其参数，这意味着被比较的值必需实现了 `PartialEq` 和 `Debug` trait。
* 对于自定义的结构体和枚举，需要实现 `PartialEq` 才能断言他们的值是否相等。

### 自定义测试时异常时的提示信息
* `assert!`宏还可以多接受几个参数，用于自定义失败信息：

```
#[test]
fn test_for_define_tips() {
    let res = add_three(4);
    assert!(res==6 , "error.the actual value is {}", res);
}
```

### 使用`should_panic`检查panic
* 它是作为注解来使用的：

```
#[test]
#[should_panic]
fn test_should_panic() {
    let gs = Guess::new(302);
    println!("{:#?}", gs);
}
```

#### 给should_panic注解增加属性
* 为了确定测试时`panic`是我们所预期的`panic`(原因)，可以给should_panic注解加上`expected`属性

```
#[test]
#[should_panic(expected="the value must between 1 and 100!")]
fn test_attr_of_should_panic() {
    let gs = Guess::new(302);
    println!("{:#?}", gs);
}
```

### 将`Rersult<(), String>`用于测试
* 需要注意一点，在使用`Rersult<(), String>`时，就不能再使用`should_panic`注解了。

```
#[test]
fn test_use_result() -> Result<(), String> {
    if 2 + 2 == 5 {
        Ok(())
    } else {
        Err(String::from("add value is error."))
    }
}
```

### 忽略一些测试
* 使用 `ignore` 属性来标记耗时的测试并排除他们
* 在运行`cargo test`前，通过对测试函数添加注解来达到ignore的效果

```
#[test]
#[ignore]
fn is_not_equal()  {
    assert_ne!(4, add_three(4));
}
```

* 如果我们只希望运行被忽略的测试，可以使用 `cargo test -- --ignored`

## 单元测试
* 测试模块的 `#[cfg(test)]` 注解告诉 Rust 只在执行 `cargo test` 时才编译和运行测试代码, 而在运行 `cargo build` 时不这么做
* 单元测试位于与源码相同的文件中，所以你需要使用 `#[cfg(test)]` 来指定他们不应该被包含进编译结果中
* `cfg` 属性代表 `configuration` 

## 集成测试
* 不同于单元测试，集成测试对于你需要测试的库来说完全是外部的。也就是它是单独的文件
* 集成测试的目的是测试库的多个部分能否一起正常工作
* 集成测试，一般需要在src的同级目录下新建目录`tests`，可以随意在这个目录中创建任意多的测试文件
* 可以针对某个`tests`目录下的测试文件运行测试：`cargo test --test integration_test`，`integration_test`是文件名部分
* `tests` 目录中的子目录不会被作为单独的 `crate` 编译或作为一个测试结果部分出现在测试输出中。
* 因此，如果要编写测试时的工具函数，可以新建目录，然后将代码文件放于其中，例如`tests/common/mod.rs`，不过文件名必须为`mod.rs`
* 使用时，引入：`mod common;`，然后调用即可：`common::test_setup();`

### 二进制 crate 的集成测试
* 在一个crate中，如果存在`src/main.rs`，书写逻辑时，最后写到一个独立的crate中，而非`main.rs`文件中，例如写到`src/lib.rs`
* 因为这样，可以为`src/lib.rs`写集成测试，（基于`src/lib.rs`的crate可以导入集成测试文件中，而`main.rs`不能）,这样`src/main.rs`文件中只需要写很少的代码就能让项目运行起来！

## 测试私有函数
* 对于私有的方法，也是可以调用测试的：

```
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {width:10,height:8,};
        let one_rec = Rectangle {width:7,height:4,};
        assert!(larger.can_hold(&one_rec));
    }
}
```

* 只需要引入对应的库，如上方示例：`use super::*;`












*/

