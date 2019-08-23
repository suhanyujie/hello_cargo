
fn main() {
    simple_dereference_pointer();
    unsafe {
        simple_unsafe_func();
    }
}

fn simple_dereference_pointer() {
    let mut num = 5;
    let p1 = &num as *const i32;
    unsafe {
        println!("p1 value is: {}", *p1);
    }
}

unsafe fn simple_unsafe_func() {
    println!("this is a unsafe function...");
}


/*
# 不安全 Rust
## unsafe 的超级力量
* 其作用主要有以下几点：
    * 解引用裸指针
    * 调用不安全的函数或方法
    * 访问或修改可变静态变量
    * 实现不安全 trait
* unsafe 并不会关闭借用检查器或禁用任何其他 Rust 安全检查。如果在不安全代码中使用引用，其仍会被检查。
* 标准库的一部分被实现为在被评审过的不安全代码之上的安全抽象。

### 解引用裸指针
* 和引用一样，裸指针是可变或不可变的，分别写作 *mut T 和 *const T。这里的星号不是解引用运算符；它是类型名称的一部分。
* 裸指针的主要特性有：
    * 允许忽略借用规则，可以同时拥有不可变和可变的指针，或多个指向相同位置的可变指针
    * 不保证指向有效的内存
    * 允许为空
    * 不能实现任何自动清理功能
* 需要注意的是，我们可以在安全代码中 创建 裸指针，只是不能在不安全块之外 解引用 裸指针
* 创建一个指针不会造成任何危险；只有当访问其指向的值时才有可能遇到无效的值。
* 通过裸指针，就能够同时创建同一地址的可变指针和不可变指针。所以，若通过可变指针修改数据，则可能潜在造成数据竞争。

### 调用不安全的函数或方法
* 不安全函数和方法与常规函数方法十分类似，除了其开头有一个额外的 unsafe。

```rust
unsafe fn simple_unsafe_func() {
    println!("this is a unsafe function...");
}
```

* 必须在 unsafe 代码块内才能调用 unsafe 函数

```rust
fn main() {
    simple_dereference_pointer();
    unsafe {
        simple_unsafe_func();
    }
}
```

*/
