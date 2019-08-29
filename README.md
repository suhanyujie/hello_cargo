# hello Rust/Cargo

## index
* [字符串](./src/study/string.rs)
* [Option 枚举](./src/study/option.rs)
* [hashmap](./src/study/hashmap.rs)
* [match](./src/study/match.rs)
* [所有权](./src/study/ownership.rs)
* [迭代器](./src/study/iterator.rs)
* [闭包](./src/study/closures.rs)
* [生命周期](./src/study/lifetime.rs)
* [trait](./src/study/trait.rs)
* [线程](./src/study/thread.rs)

## build
* `rustc src/study/ownership.rs -o a.out`

## 知识点

### run
* `./a.out`

### 所有权
* 在堆上创建字符串：`let str1 = String::from("hello world")`

## example app
### try my first app in rust
* 根据博文[building-a-command-line-todo-app-in-rust](https://medium.com/@devashishdxt/building-a-command-line-todo-app-in-rust-a89bb7af91c3)进行代码编写

#### 流程
* 每个事项有一个id和一个名称
* 当我们新增一个事项，会将其写入`todos/pending.todo`文件中，这个文件包含了所有“进行中”事项的数组，这个数组使用`bincode`编码。
* `todos/counter.todo`这个文件包含了最新使用的主键id，它是为了给新创建的事项分配一个唯一的id，当新增一个事项时，会将这个数组递增并存入这个文件中。
* 无论何时，用户标记事项已经完成是，将会将事项从`todos/pending.todo`中移除，并将其写入到`todos/completed.todo`文件中，`todos/completed.todo`中包含了所有的已完成事项的数组。

## 百坑全书
* 代码块中的调用语句，“加分号”和“不加分号”返回的结果截然不同
* 一个`rs`文件中，如果存在不会调用的函数，可以通过在函数上方添加`#[allow(dead_code)]`注解或者`#[test]`，让编译器不在提示warning。

## 参考资料
* 社区电子书 https://kaisery.github.io/trpl-zh-cn/
* https://medium.com/@devashishdxt/building-a-command-line-todo-app-in-rust-a89bb7af91c3
