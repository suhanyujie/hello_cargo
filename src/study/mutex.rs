use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;
use std::rc::Rc;

fn main() {
    // Mutex 的简单使用
    // simle_mutex();
    // 在多个线程中使用同一个变量
    // use_var_in_multi_thread();
    // 通过智能指针Rc<T> 在多个线程中使用同一个变量
    // use_var_in_multi_thread_by_rc();
    // 通过 Arc<T> 在多个线程中使用同一个变量
    use_var_in_multi_thread_by_arc();
}

fn simle_mutex()  {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num  += 1;
    }

    println!("m={:?}", m);
}

// 尝试在多个线程中使用上下文环境中的同一个变量
// fn use_var_in_multi_thread()  {
//     let m1 = Mutex::new(5);
//     let handle = thread::spawn(move||{
//         let mut num = m1.lock().unwrap();
//         *num += 1;
//     });
//     let handle2 = thread::spawn(move || {
//         let mut num = m1.lock().unwrap();
//         *num += 1;
//     });
//     // println!("result is:{:?}", m1.lock().unwrap());
//     handle.join().unwrap();
// }

// fn use_var_in_multi_thread_by_rc()  {
//     let m1 = Rc::new(Mutex::new(5));
//     let handle = thread::spawn(move||{
//         let m1_1 = Rc::clone(&m1);
//         let mut num = m1_1.lock().unwrap();
//         *num += 1;
//     });
//     let handle2 = thread::spawn(move || {
//         let m1_2 = Rc::clone(&m1);
//         let mut num = m1_2.lock().unwrap();
//         *num += 1;
//     });
//     // println!("result is:{:?}", m1.lock().unwrap());
//     handle.join().unwrap();
// }

fn use_var_in_multi_thread_by_arc()  {
    let m1 = Arc::new(Mutex::new(5));
    let m1_1 = Arc::clone(&m1);
    let handle = thread::spawn(move||{
        let mut num = m1_1.lock().unwrap();
        *num += 1;
    });
    let m1_2 = Arc::clone(&m1);
    let handle2 = thread::spawn(move || {
        let mut num = m1_2.lock().unwrap();
        *num += 1;
    });
    handle.join().unwrap();
    handle2.join().unwrap();
    println!("result is:{:?}", m1.lock().unwrap());
}


/*
## 互斥器
* 互斥器（mutex）是 “mutual exclusion” 的缩写，也就是说，任意时刻，其只允许一个线程访问某些数据。
* 为了访问互斥器中的数据，线程首先需要通过获取互斥器的 锁（lock）来表明其希望访问数据。锁是一个作为互斥器一部分的数据结构，它记录谁有数据的排他访问权。
* 正确的管理互斥器异常复杂，这也是许多人之所以热衷于通道的原因。
* 然而，在 Rust 中，得益于类型系统和所有权，我们不会在锁和解锁上出错

### Mutex<T>的 API
* 先从在单线程上下文使用互斥器开始

```rust
fn simle_mutex()  {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num  += 1;
    }

    println!("m={:?}", m);
}
```

* Mutex<T> 是一个智能指针
* 更准确的说，lock 调用 返回 一个叫做 MutexGuard 的智能指针。这个智能指针实现了 Deref 来指向其内部数据；
* 这个智能指针也提供了一个 Drop 实现，当 MutexGuard 离开作用域时自动释放锁
* 简单的说，这里锁的释放是自动发生的。

### 在线程间共享 Mutex<T>
#### 一个错误的实例
* 试一下在多个线程中使用同一个变量

```rust
fn use_var_in_multi_thread()  {
    let m1 = Mutex::new(5);
    let handle = thread::spawn(move||{
        let mut num = m1.lock().unwrap();
        *num += 1;
    });
    let handle2 = thread::spawn(move || {
        let mut num = m1.lock().unwrap();
        *num += 1;
    });
    // println!("result is:{:?}", m1.lock().unwrap());
    handle.join().unwrap();
}
```

* 变量被移动进了 handle 所代表线程的闭包中。因此我们无法在第二个线程中对其调用 lock
* 要解决这个问题，我们尝试使用一下智能指针 Rc<T>

```rust
fn use_var_in_multi_thread_by_rc()  {
    let m1 = Rc::new(Mutex::new(5));
    let handle = thread::spawn(move||{
        let m1_1 = Rc::clone(&m1);
        let mut num = m1_1.lock().unwrap();
        *num += 1;
    });
    let handle2 = thread::spawn(move || {
        let m1_2 = Rc::clone(&m1);
        let mut num = m1_2.lock().unwrap();
        *num += 1;
    });
    // println!("result is:{:?}", m1.lock().unwrap());
    handle.join().unwrap();
}
```

* 遗憾的是，还是会报错，Rc<T> 不是线程安全的
* 这时候，我们引入原子引用计数

### 原子引用计数 Arc<T>
* 其中的 A 就是原子的意思 atomic
* Arc<T> 正是 这么一个类似 Rc<T> 并可以安全的用于并发环境的类型。
* 如果只是在单线程中对值进行操作，原子性提供的保证并无必要，代码可以因此运行的更快。
* 因为线程安全带有性能惩罚，我们希望只在必要时才为此买单。因此只在必要的时候才用 `Arc<T>`

```rust
fn use_var_in_multi_thread_by_arc()  {
    let m1 = Arc::new(Mutex::new(5));
    let m1_1 = Arc::clone(&m1);
    let handle = thread::spawn(move||{
        let mut num = m1_1.lock().unwrap();
        *num += 1;
    });
    let m1_2 = Arc::clone(&m1);
    let handle2 = thread::spawn(move || {
        let mut num = m1_2.lock().unwrap();
        *num += 1;
    });
    handle.join().unwrap();
    handle2.join().unwrap();
    println!("result is:{:?}", m1.lock().unwrap());
}
```

* 类似于 两个 Rc<T> 值相互引用，造成内存泄露一样，Mutex<T> 也有造成 死锁（deadlock） 的风险

### Send 和 Sync
* Send 标记 trait 表明类型的所有权可以在线程间传递。例如 Arc<T>
* Rust 类型系统和 trait bound 确保永远也不会意外的将不安全的 Rc<T> 在线程间发送。
* 任何完全由 Send 的类型组成的类型也会自动被标记为 Send。几乎所有基本类型都是 Send 的

#### Sync
* Sync 允许多线程访问
* Sync 标记 trait 表明一个实现了 Sync 的类型可以安全的在多个线程中拥有其值的引用。
* 换一种方式来说，对于任意类型 T，如果 &T（T 的引用）是 Send 的话 T 就是 Sync 的
* 基本类型是 Sync 的，完全由 Sync 的类型组成的类型也是 Sync 的。
* 智能指针 Rc<T> 也不是 Sync 的




*/