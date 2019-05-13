use std::ops::Deref;
use std::rc::Rc;

fn main() {
    // 使用Box存放数据
    get_a_box();
    // 递归类型
    test_recursive();
    // 使用 * 作为解引用 运算符
    use_deref_operator();
    // 自定义智能指针
    self_defined_smart_point();
    self_defined_smart_point_2();
    // 函数和方法的隐式解引用强制多态
    deref_auto_change_of_type();
    // Drop trait
    test_drop();
    // 主动释放资源
    println!("主动释放资源：");
    release_resource_automaticaly();
    // Rc的多所有权
    println!("Rc的多所有权:");
    mul_ownership();
    // 查看 Rc 的引用计数
    println!("查看 Rc 的引用计数");
    get_mul_ownership_ref_count();
}

// 定义一个 Box
fn get_a_box(){
    let a = Box::new(5);
    println!("{}", a);
}

// 递归类型
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
fn test_recursive() {
    let list = List::Cons(1, 
                        Box::new(List::Cons(2, 
                            Box::new(List::Cons(3, 
                                Box::new(List::Nil)
                            ))
                        ))
                );
    println!("{:?}", list);
}

// 使用 * 作为解引用 运算符
fn use_deref_operator()  {
    let num1 = 6;
    let num2 = Box::new(num1);
    println!("{}", *num2);
}

// 自定义智能指针
#[derive(Debug)]
struct MyBox<T> (T);
impl<T> MyBox<T> {
    fn new(x: T) ->MyBox<T> {
        MyBox(x)
    }
}
fn self_defined_smart_point()  {
    let num1 = 6;
    let num2 = MyBox::new(num1);
    // 此时 MyBox 类型的num2 没有实现解引用，因此编译错误
    // println!("{}", *num2);
}
impl<T> Deref for MyBox<T> {
    // 用于此 trait 的关联类型
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
fn self_defined_smart_point_2()  {
    let num1 = 6;
    let num2 = MyBox::new(num1);
    println!("{}", *num2);
}

// 函数和方法的隐式解引用强制多态
fn hello(s1: &str) {
    println!("hello {}!", s1)
}
fn deref_auto_change_of_type()  {
    let s1 = MyBox::new(String::from("Sam"));
    hello(&s1);
}

// drop 方法如何工作
#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}
fn test_drop() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer create.");
}

// 提前、主动释放资源
fn release_resource_automaticaly() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer create.");
    drop(c);
    println!("CustomSmartPointer droped before the end of main.");
}

// 多所有权数据
#[derive(Debug)]
enum List1 {
    Cons(i32, Rc<List1>),
    Nil,
}
fn mul_ownership() {
    let a = Rc::new(List1::Cons(2, 
                            Rc::new(List1::Cons(3, 
                                Rc::new(List1::Nil)
                            ))
                        ));
    let list2 = List1::Cons(10, Rc::clone(&a));
    let list3 = List1::Cons(11, Rc::clone(&a));
    println!("{:?}", list3);
}

// 查看 Rc 的引用计数
fn get_mul_ownership_ref_count() {
    let a = Rc::new(List1::Cons(2, 
                            Rc::new(List1::Cons(3, 
                                Rc::new(List1::Nil)
                            ))
                        ));
    println!("count after creating a = {}", Rc::strong_count(&a));                
    let list2 = List1::Cons(10, Rc::clone(&a));
    println!("place 1, a = {}", Rc::strong_count(&a));  
    {
        let list3 = List1::Cons(11, Rc::clone(&a));
        println!("place 1, a = {}", Rc::strong_count(&a)); 
        // 离开内部作用域，会对应的清理一次引用
    }  
    println!("place 1, a = {}", Rc::strong_count(&a));             
}

/*
## 前言
* 通过这一章的学习，需要知道：
    * 什么是内部可变性（interior mutability）模式？
    * 怎么解决引用循环问题？

## 智能指针
* 指针 （pointer）是一个包含内存地址的变量的通用概念。这个地址引用，或 “指向”（points at）一些其他数据。Rust 中最常见的指针是第四章介绍的 引用（reference）。引用以 & 符号为标志并借用了他们所指向的值。除了引用数据没有任何其他特殊功能。它们也没有任何额外开销，所以应用的最多。
* Rust 中，智能指针（smart pointers）是一类数据结构，他们的表现类似指针
* 起源于 C++ 并存在于其他语言中。
* Rust 标准库中不同的智能指针提供了多于引用的额外功能。本章将会探索的一个例子便是 引用计数 （reference counting）智能指针类型，其允许数据有多个所有者。引用计数智能指针记录总共有多少个所有者，并当没有任何所有者时负责清理数据。
* 智能指针是一个在 Rust 经常被使用的通用设计模式。很多库都有自己的智能指针而你也可以编写属于你自己的智能指针。

### 普通引用和智能指针的区别
* 引用是一类只借用数据的指针
* 智能指针 拥有 他们指向的数据。

### 实现
* 智能指针区别于常规结构体的显著特性在于其实现了 `Deref` 和 `Drop` trait

### 标准库中常用的智能指针
* Box<T>，用于在堆上分配值
* Rc<T>，一个引用计数类型，其数据可以有多个所有者
* Ref<T> 和 RefMut<T>，通过 RefCell<T> 访问，一个在运行时而不是在编译时执行借用规则的类型。

### 自定义智能指针
#### 解引用运算符 `*`
* 注意，每次当我们在代码中使用 `*` 时， `*` 运算符都被替换成了先调用 `deref` 方法再接着使用 * 解引用的操作，且只会发生一次，不会对 * 操作符无限递归替换，解引用出上面 i32 类型的值就停止了，这个值与示例 15-9 中 assert_eq! 的 5 相匹配。

### 函数和方法的隐式解引用强制多态
* 解引用强制多态使得 Rust 自动的帮我们处理参数的类型转换。
* 解引用强制多态的加入使得 Rust 程序员编写函数和方法调用时无需增加过多显式使用 & 和 * 的引用和解引用。这个功能也使得我们可以编写更多同时作用于引用或智能指针的代码。
* 当所涉及到的类型定义了 Deref trait，Rust 会分析这些类型并使用任意多次 Deref::deref 调用以获得匹配参数的类型。这些解析都发生在编译时，所以利用解引用强制多态并没有运行时代价！
* 不可变引用永远也不能强转为可变引用。

## Drop trait清理代码
* 对于智能指针模式来说第二个重要的 trait 是 Drop，其允许我们在值要离开作用域时执行一些代码。可以为任何类型提供 Drop trait 的实现，同时所指定的代码被用于释放类似于文件或网络连接的资源。
* 指定在值离开作用域时应该执行的代码的方式是实现 Drop trait。Drop trait 要求实现一个叫做 drop 的方法，它获取一个 self 的可变引用。

```rust
#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}
fn test_drop() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer create.");
}
```

* Rust 并不允许我们主动调用 Drop trait 的 drop 方法；
* 当我们希望在作用域结束之前就强制释放变量的话，我们应该使用的是由标准库提供的 std::mem::drop
* Rust 中的 drop 函数就是这么一个析构函数。

## `std::mem::drop` 的使用
* 因为 `Drop` trait 中的 drop 方法不能被显示的调用，如果遇到需要手动释放资源的一些场景下，该如何处理呢？
* 标准库中提供了 `std::mem::drop` ，可以记性主动或者提前释放资源：

```rust
fn release_resource_automaticaly() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer create.");
    drop(c);
    println!("CustomSmartPointer droped before the end of main.");
}
```

## Rc<T>
* 为了启用多所有权，Rust 有一个叫做 Rc<T> 的类型。其名称为 引用计数（reference counting）的缩写。引用计数意味着记录一个值引用的数量来知晓这个值是否仍在被使用。如果某个值有零个引用，就代表没有任何有效引用并可以被清理。
* 它用于当我们希望在堆上分配一些内存供程序的多个部分读取，而且无法在编译时确定程序的那一部分会最后结束使用它的时候。
* `Rc<T>` 只能用于单线程场景
* 不同于引用，当多所有权的数据被在多个地方使用时，其生命周期取决于使用该数据最久的那一处。
* 而如果是引用类型的话，其生命周期取决于有所有权数据所在的变量何时被清理。
* Rc 通过对引用计数的增加从而实现多所有权：

```rust
#[derive(Debug)]
enum List1 {
    Cons(i32, Rc<List1>),
    Nil,
}
fn mul_ownership() {
    let a = Rc::new(List1::Cons(2, 
                            Rc::new(List1::Cons(3, 
                                Rc::new(List1::Nil)
                            ))
                        ));
    let list2 = List1::Cons(10, Rc::clone(&a));
    let list3 = List1::Cons(11, Rc::clone(&a));
    println!("{:?}", list3);
}
```

* 这里的拷贝就是增加计数的过程，出了使用 `Rc::clone(&a)` ，还可以使用 `a.clone()`

### 查看 Rc 数据的引用计数
* 可以通过 `Rc::strong_count` 进行查看

```rust
fn get_mul_ownership_ref_count() {
    let a = Rc::new(List1::Cons(2, 
                            Rc::new(List1::Cons(3, 
                                Rc::new(List1::Nil)
                            ))
                        ));
    println!("count after creating a = {}", Rc::strong_count(&a));                
    let list2 = List1::Cons(10, Rc::clone(&a));
    println!("place 1, a = {}", Rc::strong_count(&a));  
    {
        let list3 = List1::Cons(11, Rc::clone(&a));
        println!("place 1, a = {}", Rc::strong_count(&a)); 
        // 离开内部作用域，会对应的清理一次引用
    }  
    println!("place 1, a = {}", Rc::strong_count(&a));             
}
```

* `Rc<T>` 允许通过**不可变引用**来只读的在程序的多个部分共享数据。
* 因为允许多个可变引用，则会违反第四章讨论的借用规则之一：相同位置的多个可变借用可能造成数据竞争和不一致。

*/
