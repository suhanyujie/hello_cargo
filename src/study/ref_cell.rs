use std::rc::Rc;
use std::cell::RefCell;

/**
 * 因为单个rs文件，不支持单元测试，所以这里调整一下，直接在main中调用tests中的函数
 */
fn main() {
    tests::it_sends_an_over_75_percent_warning_message();

    // 结合 Rc 和 RefCell 的实例
    tests::combine_rc_and_refcell();
}

pub trait Messager {
    fn send(&self, msg:&str);
}

#[derive(Debug)]
pub struct LimitTracker<'a, T:'a + Messager> {
    messager:&'a T,
    value:usize,
    max:usize,
}

impl<'a, T> LimitTracker<'a, T> 
    where T:Messager 
{
    pub fn new(messager:&T, max:usize) -> LimitTracker<T> {
        LimitTracker {
            messager,
            value:0,
            max,
        }
    }

    pub fn set_value(&mut self, value:usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messager.send("Warning:You're used up over 75% of your quota!");
        } else if percentage_of_max > 0.9 && percentage_of_max < 1.0 {
            self.messager.send("Urgent warning:you are used up over 90% of your quota");
        } else if percentage_of_max > 1.0 {
            self.messager.send("Error:You are over your quota!");
        }
    }
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

// #[cfg(test)]
pub mod tests {
    use super::*;
    use std::cell::RefCell;
    use List::{Cons, Nil};
    use std::rc::Rc;

    #[derive(Debug)]
    struct MockMessager {
        // sent_messages:Vec<String>,
        sent_messages_sp: RefCell<Vec<String>>,
    }

    impl MockMessager {
        fn new() -> MockMessager {
            MockMessager {
                // sent_messages:vec![],
                sent_messages_sp: RefCell::new(vec![]),
            }
        }            
    }

    impl Messager for MockMessager {
        fn send(&self,message:&str) {
            println!("{}", message);
            // self.sent_messages.push(String::from(message));
            self.sent_messages_sp.borrow_mut().push(String::from(message));
        }
    }

    // #[test]
    pub fn it_sends_an_over_75_percent_warning_message() {
        let mock_messager = MockMessager::new();
        let mut limit_tracker = LimitTracker::new(&mock_messager, 100);
        limit_tracker.set_value(80);
        // assert_eq!(mock_messager.sent_messages.len(), 1);
        assert_eq!(mock_messager.sent_messages_sp.borrow().len(), 1);
        // println!("{:?}", mock_messager.sent_messages_sp.borrow().len());
    }

    // 结合 Rc 和 RefCell 的实例
    pub fn combine_rc_and_refcell() {
        let value = Rc::new(RefCell::new(5));
        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
        let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
        *value.borrow_mut() += 10;
        println!("{:?}", a);
        // println!("a after = {:?}", a);
    }
}



/*
## `RefCell<T>` 和内部可变性模式
### 回顾借用规则
* 在任意给定时间，只能拥有一个可变引用或任意数量的不可变引用 之一（而不是全部）。
* 引用必须总是有效的。

### `RefCell<T>` 的作用
* 内部可变性（Interior mutability）是 Rust 中的一个设计模式，它允许你即使在有不可变引用时改变数据，这通常是借用规则所不允许的。
* 为了改变数据，该模式在数据结构中使用 unsafe 代码来模糊 Rust 通常的可变性和借用规则。
* 不同于 `Rc<T>`，`RefCell<T>` 代表其数据的唯一的所有权。
* 类似于 `Rc<T>`，`RefCell<T>` 只能用于单线程场景。如果尝试在多线程上下文中使用 `RefCell<T>`，会得到一个编译错误。

### `Box<T>`，`Rc<T>` 和 `RefCell<T>` 的不同
* Rc<T> 允许相同数据有多个所有者；Box<T> 和 RefCell<T> 有单一所有者
* Box<T> 允许在编译时执行不可变或可变借用检查；Rc<T>仅允许在编译时执行不可变借用检查；RefCell<T> 允许在运行时执行不可变或可变借用检查。
* 因为 `RefCell<T>` 允许在运行时执行可变借用检查，所以我们可以在即便 RefCell<T> 自身是不可变的情况下修改其内部的值。

### `RefCell<T>` 在运行时记录借用
* 当创建不可变和可变引用时，我们分别使用 `&` 和 `&mut` 语法
* borrow 方法返回 `Ref` 类型的智能指针，borrow_mut 方法返回 `RefMut` 类型的智能指针。这两个类型都实现了 `Deref` 所以可以当作常规引用对待。

#### RefCell<T> 如何在运行时处理违反借用规则的情况
* 在运行时捕获借用错误而不是编译时意味着将会在开发过程的后期才会发现错误，甚至有可能发布到生产环境才发现。还会因为在运行时而不是编译时记录借用而导致少量的运行时性能惩罚。
* 然而，使用 RefCell 使得在只允许不可变值的上下文中编写修改自身以记录消息的 mock 对象成为可能。虽然有取舍，但是我们可以选择使用 RefCell<T> 来获得比常规引用所能提供的更多的功能。

### 结合 Rc<T> 和 RefCell<T> 来拥有多个可变数据所有者
* `Rc<T>` 允许对相同数据有多个所有者，不过只能提供数据的不可变访问
* 如果有一个储存了 RefCell<T> 的 Rc<T> 的话，就可以得到有多个所有者 并且 可以修改的值了！实例如下：

```rust
 // 结合 Rc 和 RefCell 的实例
pub fn combine_rc_and_refcell() {
    let value = Rc::new(RefCell::new(5));
    // clone 过后，a 和 value 都拥有 5 的所有权
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    *value.borrow_mut() += 10;
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}
```

* `RefCell<T>` 的运行时借用规则检查也确实保护我们免于出现数据竞争，而且我们也决定牺牲一些速度来换取数据结构的灵活性。


*/