use std::thread;
use std::time::Duration;

fn main() {
    // 创建线程
    // create_thread();

    // 使用 join 等待线程结束
    // wait_endup_for_thread();

    // 不使用 move 闭包尝试
    // not_move();
    // 使用 move 闭包，使闭包中可以获取上下文环境信息
    using_move();
}

fn create_thread(){
    thread::spawn(||{
        for i in 1..10 {
            println!("hi number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn wait_endup_for_thread() {
    let handle = thread::spawn(||{
        for i in 1..10 {
            println!("hi number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

// 使用普通闭包尝试使用主线程中的变量
fn not_move(){
    let v = vec![1,2,3];
    let handle = thread::spawn(||{
        // 当前函数是一个错误的范例，下面如果不注释，则无法编译
        // println!("v's value is: {:?}", v);
    });
    handle.join().unwrap();
}

fn using_move(){
    let v = vec![1,2,3];
    let handle = thread::spawn(move ||{
        println!("v's value is: {:?}", v);
    });
    handle.join().unwrap();
}

/*
## 无畏并发
### 创建线程
* 为了创建一个新线程，需要调用 thread::spawn 函数并传递一个闭包，其包含希望在新线程运行的代码
* 当主线程结束时，新线程也会结束，而不管其是否执行完毕。这有点类似 go 中的主协程关闭后，其他的工作协程也会退出。
* `thread::sleep` 调用强制线程停止执行一小段时间，这会允许其他不同的线程运行
* 这些线程可能会轮流运行，不过并不保证如此：这依赖操作系统如何调度线程。

```rust
fn create_thread(){
    thread::spawn(||{
        for i in 1..10 {
            println!("hi number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

### 使用 join 等待线程结束
* 从 thread::spawn 保存一个 JoinHandle 以确保该线程能够运行至结束

```rust
fn wait_endup_for_thread() {
    let handle = thread::spawn(||{
        for i in 1..10 {
            println!("hi number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

* 主线程会由于 handle.join() 调用会等待直到新建线程执行完毕。

### 线程与 move 闭包
* move 闭包经常与 thread::spawn 一起使用，它允许我们在一个线程中使用另一个线程的数据。
* 在参数列表前使用 move 关键字强制闭包获取其使用的环境值的所有权。这个技巧在将闭包传递给新线程以便将数据移动到新线程中时最为实用。
* 和 PHP 中创建进程类似，在 fork 一个进程时，新的进程会拷贝父进程上下文的副本，其中就包含了运行环境中的变量 堆栈等信息
* Rust 中，为了在新建线程中使用来自于主线程的数据，需要新建线程的闭包获取它需要的值。
* 想象一下，如果只是使用普通闭包来让新建线程使用主线程中的变量，会发生什么？

```rust
fn not_move(){
    let v = vec![1,2,3];
    let handle = thread::spawn(||{
        println!("v's value is: {:?}", v);
    });
}
```

* 运行这段代码，会报错如下：

```
closure may outlive the current function, but it borrows `v`, which is owned by the current function
```

* 接下来，对比一下使用 move 的情况

```rust
fn using_move(){
    let v = vec![1,2,3];
    let handle = thread::spawn(move ||{
        println!("v's value is: {:?}", v);
    });
    handle.join().unwrap();
}
```

* 通过告诉 Rust 将 v 的所有权移动到新建线程，我们向 Rust 保证主线程不会再使用 v，如果对 v 修改，那么当在主线程中使用 v 时就会违反所有权规则

*/