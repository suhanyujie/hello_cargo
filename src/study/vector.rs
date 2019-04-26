

fn main() {
    let mut v = vec![12,21,43];
    // 遍历数组
    println!("\n遍历数组:");
    for i in &v {
        println!("{}", i)
    }
    // 访问数组单元
    println!("the val of 1 is:{}", &v[1]);
    // 通过get方法 访问数组单元
    match v.get(1) {
        Some(val) => println!("This is val:{}", val),
        None => println!("This is None"),
    }
    println!("{:?}", v.get(1));
    // 修改数组单元
    println!("\n修改数组单元后，打印结果:");
    for val in &mut v {
        *val = *val + 1;
    }
    println!("{:?}", v);

    // 通过枚举向数组中存放不同类型的值
    println!("\n通过枚举向数组中存放不同类型的值:");
    #[derive(Debug)]
    enum Enum1 {
        Int1(i32),
        Float1(f64),
        Text2(String),
    }
    let mut v = vec![
        Enum1::Int1(12)
    ];
    v.push(Enum1::Text2(String::from("hello i'm String")));
    for i in &v {
        println!("{:#?}", i)
    }
}

/*
## 数组
* 它的类型描述是`Vec<T>`，也被称为`vector`
* Vec 是一个由标准库提供的类型，它可以存放任何类型，而当 Vec 存放某个特定类型时，那个类型位于尖括号中
* 它允许在一个单独的数据结构中储存多于一个的值，它在内存中彼此相邻地排列所有的值。

### 新建一个Vec
* 新建方式如下：`let v:Vec<i32> = Vec::new();`
* 如果在声明时就初始化，则无需指定`<>`中的内容，而直接让编译器来推断即可。

#### 使用宏创建Vec
* Rust 提供了 `vec!` 宏
* 宏的写法如下：`let v = vec![1,24,12];`

## 数组的使用
### 增加内容
* 向数组中增加元素，需要数组变量是可变的，因此需要使用`mut`修饰
* 使用`push`方法，可以向数组中追加单元，如下：

```
let mut v = vec![12,21,43];
v.push(17);
v.push(19);
```

* 当 vector 被丢弃时，所有其内容也会被丢弃，这意味着这里它包含的整数将被清理。

### 数组的访问
* 法1：使用 `get` 方法以索引作为参数来返回一个 `Option<&T>`，例如:`v.get(2)`
* 法2：使用 `&` 和 `[]` 返回一个引用，例如：`&v[2]`

#### 注意事项
* 使用方法2时，即引用一个不存在的元素时 Rust 会造成 `panic`
* 使用方法1，即当 `get` 方法被传递了一个数组外的索引时，它不会 `panic` 而是返回 `None`

### 遍历数组
* 使用`for`的语法，可以遍历一个数组：

```
let mut v = vec![12,21,43];
for i in &v {
    println("{}", i);
}
```

### 通过枚举向数组中存放不同类型的值
* 定义一个枚举，其成员会存放这些不同类型的值，同时所有这些枚举成员都会被当作相同类型，那个枚举的类型。如下：

```

```





*/