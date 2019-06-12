use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // 创建一个通道
    create_channel();
    // 向通道发送简单的信息
    simple_send_msg_to_channel();
    // 从通道中接收数据
    // receive_msg_from_channel();
    // 向通道中多次发送数据
    // send_many_msg_to_channel();
    // 多生产者发送数据到通道中
    multi_producer_send_msg();
}

fn create_channel() {
    // 使用了一个 let 语句和模式来解构了此元组
    // let (tx, rx) = mpsc::channel();
}

// 向通道发送简单信息
fn simple_send_msg_to_channel(){
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
    });
    handle.join().unwrap();
}

// 从通道中接收数据
fn receive_msg_from_channel() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("received's value is: {:?}", received);
    handle.join().unwrap();
}

// 向通道中多次发送数据
fn send_many_msg_to_channel() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let v = vec![
            String::from("Caral"),
            String::from("Eason"),
            String::from("XinYao"),
            String::from("Mark"),
        ];
        for msg in v {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    //let received = rx.recv().unwrap();
    for received in rx {
        println!("get msg:{}", received);
    }
    handle.join().unwrap();
}

// 多发送端向通道发送消息
fn multi_producer_send_msg()  {
    let (tx, rx) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("t1 Caral"),
            String::from("t1 Eason"),
            String::from("t1 XinYao"),
            String::from("t1 Mark"),
        ];
        for msg in vals {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move||{
        let vals = vec![
            String::from("t2 Six"),
            String::from("t2 Eight"),
            String::from("t2 Tank"),
            String::from("t2 Nike"),
        ];
        for msg in vals {
            tx2.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx  {
        println!("get msg : {}", received);
    }
}

/*
## 通道 —— 通过通讯来共享内存

### 通道的创建
* 使用 mpsc::channel 函数创建一个新的通道

```rust
fn create_channel() {
    // 使用了一个 let 语句和模式来解构了此元组
    let (tx, rx) = mpsc::channel();
}
```

* mpsc 是 多个生产者，单个消费者（multiple producer, single consumer）的缩写。

### 像通道发送信息
* 通道的发送端有一个 send 方法用来获取需要放入通道的值。send 方法返回一个 Result<T, E> 类型，所以如果接收端已经被丢弃了，将没有发送值的目标，所以发送操作会返回错误。

```rust
fn simple_send_msg_to_channel(){
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
    });
    handle.join().unwrap();
}
```

### 接收通道中的信息
* 通道的接收端有两个有用的方法：recv 和 try_recv
* 先试试使用 recv，这个方法会阻塞主线程执行直到从通道中接收一个值。

```rust
fn receive_msg_from_channel() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("received's value is: {:?}", received);
    handle.join().unwrap();
}
```

* 当通道发送端关闭，recv 会返回一个错误表明不会再有新的值到来了。
* try_recv 不会阻塞，相反它立刻返回一个 Result<T, E>：Ok 值包含可用的信息，而 Err 值代表此时没有任何消息。

### 发送数据和所有权
* send 函数获取其参数的所有权并移动这个值归接收者所有。这个意味着不可能意外的在发送后再次使用这个值；所有权系统检查一切是否合乎规则。

### 向通道中多次发送数据
* 示例代码如下：

```rust
fn send_many_msg_to_channel() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let v = vec![
            String::from("Caral"),
            String::from("Eason"),
            String::from("XinYao"),
            String::from("Mark"),
        ];
        for msg in v {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    //let received = rx.recv().unwrap();
    for received in rx {
        println!("get msg:{}", received);
    }
    handle.join().unwrap();
}
```

* 在主线程中，不再显式调用 recv 函数：而是将 rx 当作一个迭代器。

### 克隆发送者来创建多个生产者
* 之前我们提到了mpsc是 multiple producer, single consumer 的缩写
* 可以通过克隆通道的发送端来创建爱你多个发送者，克隆代码诸如：`mpsc::Sender::clone(&tx);`

```rust
fn multi_producer_send_msg()  {
    let (tx, rx) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("t1 Caral"),
            String::from("t1 Eason"),
            String::from("t1 XinYao"),
            String::from("t1 Mark"),
        ];
        for msg in vals {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move||{
        let vals = vec![
            String::from("t2 Six"),
            String::from("t2 Eight"),
            String::from("t2 Tank"),
            String::from("t2 Nike"),
        ];
        for msg in vals {
            tx2.send(msg).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx  {
        println!("get msg : {}", received);
    }
}
```

*/