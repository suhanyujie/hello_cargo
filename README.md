## 知识点
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

## 参考资料
* 社区电子书 https://kaisery.github.io/trpl-zh-cn/
* https://medium.com/@devashishdxt/building-a-command-line-todo-app-in-rust-a89bb7af91c3
