
fn main() {
    // 迭代器适配器
    a_iterator_of_map();

    // 消费迭代器并收集结果
    collect_for_map();

    // into_iter的使用
    let many_shoes = vec![
        Shoe{size:38, style:String::from("nike")},
        Shoe{size:40, style:String::from("adiddas")},
        Shoe{size:41, style:String::from("tebu")},
        Shoe{size:42, style:String::from("贵人鸟")},
        Shoe{size:42, style:String::from("Meters")},
        Shoe{size:40, style:String::from("burberry")},
    ];
    let my_size_shoes = shoes_in_my_size(many_shoes, 40);
    println!("{:?}", my_size_shoes);

    test_counter_for_iterator();

    // 测试其他的iterator适配器，可以通过文档查看标准库中实现的迭代器适配器
    test_other_iterator();
}

// 迭代器适配器
fn a_iterator_of_map(){
    let v1:Vec<i32> = vec![1,2,5,12,98]; 
    let m1 = v1.iter().map(|x|x+2);
    println!("{:?}", m1);
}

// 消费迭代器并收集结果
fn collect_for_map(){
    let v1:Vec<i32> = vec![1,2,5,12,98]; 
    let v1:Vec<_> = v1.iter().map(|x|x+2).collect();
    println!("{:?}", v1);
}

// test for into_iter
#[derive(Debug)]
struct Shoe {
    size:u32,
    style:String,
}

fn shoes_in_my_size(shoes:Vec<Shoe>, shoe_size:u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|shoe| shoe.size==shoe_size)
        .collect()
}

// 自定义迭代器
#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {count:0,}
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) ->Option<Self::Item>{
        self.count += 1;
        if self.count > 6 {
            None
        } else {
            Some(self.count)
        }
    }
}

fn test_counter_for_iterator() {
    let mut c1 = Counter::new();
    println!("{:?}", c1.next());
    println!("{:?}", c1.next());
    println!("{:?}", c1.next());
}

fn test_other_iterator()  {
    let c1:Vec<_> = Counter::new().zip(Counter::new().skip(1))
                .map(|(x,y)| x*y)
                .filter(|x| x%3==0)
                .collect();
    println!("{:?}", c1);
}


// trait Iterator {
//     type Item;

//     fn next(&mut self) ->Option<Slef::Item>;
//     // todo 
// }

/*
## 迭代器
* 迭代器模式允许你对一个项的序列进行某些处理。迭代器（iterator）负责遍历序列中的每一项和决定序列何时结束的逻辑。当使用迭代器时，我们无需重新实现这些逻辑。
* 在 Rust 中，迭代器是惰性的（lazy），这意味着在调用方法使用迭代器之前它都不会有效果。

### `Iterator` 和 `next` 方法
* 在Rust中，`Iterator`是一个`trait`，在其他语言，可能是不同的实现，例如在PHP中，`Iterator`是一个接口
* Rust中，`Iterator`的大致代码如下：

```
trait Iterator {
    type Item;

    fn next(&mut self) ->Option<Slef::Item>;
    // todo 
}
```

* next 是 Iterator 实现者被要求定义的唯一方法。next 一次返回迭代器中的一个项，封装在 Some 中，当迭代器结束时，它返回 None
* 如果我们需要一个获取 v1 所有权并返回拥有所有权的迭代器，则可以调用 into_iter 而不是 iter
* 如果我们希望迭代可变引用，则可以调用 iter_mut 而不是 iter

### 迭代器适配器
* “迭代器适配器”允许我们将当前迭代器变为不同类型的迭代器。
* 可以链式调用多个迭代器适配器，因为调用迭代器适配器后，返回的还是迭代器
* 如下，map是“迭代器适配器”的一种：

```
fn a_iterator_of_map(){
    let v1:Vec<i32> = vec![1,2,5,12,98]; 
    let m1 = v1.iter().map(|x|x+2);
    println!("{:?}", m1);
}
```

#### 消费迭代器收集结果
* 迭代器是惰性的，所以只有消费的时候，才能获取具体的值，可以使用`collect`进行消费

```
fn collect_for_map(){
    let v1:Vec<i32> = vec![1,2,5,12,98]; 
    let v1:Vec<_> = v1.iter().map(|x|x+2).collect();
    println!("{:?}", v1);
}
```

* 值得注意的是，collect需要进行变量绑定：` let v1:Vec<_> = v1.iter().map(|x|x+2).collect();`

#### 使用into_iter
* 示例如下：

```
#[derive(Debug)]
struct Shoe {
    size:u32,
    style:String,
}

fn shoes_in_my_size(shoes:Vec<Shoe>, shoe_size:u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|shoe| shoe.size==shoe_size)
        .collect()
}
```

### 创建自定义迭代器
* 可以实现 `Iterator` trait 来创建任何我们希望的迭代器。
* 如下方所示：

```
#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {count:0,}
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) ->Option<Self::Item>{
        self.count += 1;
        if self.count > 6 {
            None
        } else {
            Some(self.count)
        }
    }
}

fn test_counter_for_iterator() {
    let mut c1 = Counter::new();
    println!("{:?}", c1.next());
    println!("{:?}", c1.next());
    println!("{:?}", c1.next());
}
```

*/