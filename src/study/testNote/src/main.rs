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

// 将`Rersult<(), String>`用于测试
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
### 标记注解
* 在rust代码中，我们可以在函数前使用注解`#[test]`表示该函数是测试函数

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
* 

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

*/

