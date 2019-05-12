fn main() {
    // 使用Box存放数据
    get_a_box();
}

// 定义一个 Box
fn get_a_box(){
    let a = Box::new(5);
    println!("{}", a);
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




*/
