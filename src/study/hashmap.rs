use std::collections::HashMap;

fn main() {
    println!("\nhashmap的创建:");
    create_hashmap();
    // 带有所有权的变量值，放入hashmap
    println!("\nhashmap中值的所有权:");
    ownership_for_hashmap();
    ownership_for_hashmap_1();
    // 访问hashmap中的值
    println!("\n访问hashmap中的值:");
    get_value_of_hashmap();
    // hashmap的遍历
    println!("\nhashmap的遍历:");
    foreach_hashmap();
    // 更新hashmap
    println!("\n更新hashmap:");
    update_hashmap();
    update_hashmap_2();
    update_hashmap_3();
}

// 新建一个hashmap
fn create_hashmap()  {
    let mut h1 = HashMap::new();
    h1.insert(String::from("helloKey"), "test value");
    println!("{:#?}", h1);
}

// 带有所有权的变量值，放入hashmap
fn ownership_for_hashmap()  {
    let s1 = String::from("key1");
    let v1 = String::from("value1");
    let mut h1 = HashMap::new();
    h1.insert(s1, v1);
    println!("{:#?}", h1);
    // v1值的所有权已经转移到hashmap中，所有后续不能再使用v1
    // println!("s1 value is:{}", v1)
}
fn ownership_for_hashmap_1()  {
    let s1 = String::from("key1");
    let v1 = String::from("value1");
    let mut h1 = HashMap::new();
    h1.insert(s1, &v1);
    println!("{:#?}", h1);
    // `h1.insert(s1, &v1);`调用中，用的是v1的引用，后续可以使用v1
    println!("s1 value is:{}", v1)
}

// 访问hashmap中的值
fn get_value_of_hashmap()  {
    let s1 = String::from("key1");
    let v1 = String::from("value1");
    let mut h1 = HashMap::new();
    h1.insert(s1, &v1);
    // 通过get方法访问
    let k1 = String::from("key1");
    let res_v = h1.get(&k1);
    println!("get the value is {:#?}", res_v);
}

// 遍历hashmap
fn foreach_hashmap()  {
    let s1 = String::from("key1");
    let v1 = String::from("value1");
    let s2 = String::from("key2");
    let v2 = String::from("value2");
    let mut h1 = HashMap::new();
    h1.insert(s1, v1);
    h1.insert(s2, v2);
    for (key,value) in &h1 {
        println!("{}=>{}", key, value);
    }
}

// 更新hashmap
fn update_hashmap()  {
    let s1 = String::from("key1");
    let v1 = String::from("value1");
    let mut h1 = HashMap::new();
    h1.insert(&s1, &s1);
    h1.insert(&s1, &v1);
    println!("{:#?}", h1)
}

// 更新hashmap 2，检查值是否有存在，不存在则插入
fn update_hashmap_2()  {
    let k1 = String::from("key1");
    let k2 = String::from("key2");
    let k3 = String::from("key3");
    let v1 = String::from("value1");
    let v2 = String::from("value2");
    let mut h1 = HashMap::new();
    h1.insert(&k1, &k1);
    h1.insert(&k1, &v1);
    h1.insert(&k2, &v2);
    // 因为k1对应的键值对已存在，所以v2没有被插入进去。如果存在，会返回对应值的`可变引用`
    h1.entry(&k1).or_insert(&v2);
    // 因为k3对应的键值对不存在，所以`k3->v2`键值对被插入进去
    h1.entry(&k3).or_insert(&v2);
    println!("{:#?}", h1)
}

// 更新hashmap 3，根据已经存在的值进行更新
fn update_hashmap_3()  {
    let k1 = String::from("key1");
    let k2 = String::from("key2");
    let k3 = String::from("key3");
    let v1 = String::from("value1");
    let v2 = String::from("value2");
    let mut h1 = HashMap::new();
    h1.insert(&k1, 1);
    h1.insert(&k2, 1);
    // 因为打印下方使用了可变引用，而一个变量的可变引用和不可变引用是不能在同一个作用域中出现的，所以独立出一个作用域
    {
        // k1对应的键值对已存在，所以不会再插入
        let k1v = h1.entry(&k1).or_insert(1);
        *k1v = *k1v + 10;
    }
    println!("{:?}", h1);
}

/*
## hashmap
* hashmap可以用于需要任何类型作为键来寻找数据的场景。这种特性，使其比vector更加灵活。

### 新建一个hashmap
* 在之前，需要引入hashmap的包：`use std::collections::HashMap;`
* 使用new方法：

```
fn create_hashmap()  {
    let mut h1 = HashMap::new();
    h1.insert(String::from("helloKey"), "test value");
    println!("{:#?}", h1);
}
```

* hashmap将它们的数据存放在堆上

### 访问hashmap中的值
* 通过hanshmap的`get`方法获取其中的值 `h1.get(&k1)`
* 值得注意的是，`get`返回的值的类型是`Option<V>`，也就是说结果被装进 `Some`
* 如果访问的值不存在，则返回的是`None`

#### 遍历hashmap
* 类似遍历vector，使用`for`遍历hashmap，只是格式上有所差异：

```
for (key,value) in &h1 {
    println!("{}=>{}", key, value);
}
```

* 更新hashmap
* 任何时候，每个键只能关联一个值

##### 用相同的键插入一个不同的值，与这个键相关联的旧值将被替换

```
fn update_hashmap()  {
    let s1 = String::from("key1");
    let v1 = String::from("value1");
    let mut h1 = HashMap::new();
    h1.insert(&s1, &s1);
    h1.insert(&s1, &v1);
    println!("{:#?}", h1)
}
```

##### 只在没有这个键值对时插入。检查某个特定的键是否有值，如果没有就插入一个值
* 这个api倒是少见，在rust中有提供
* 使用`entry`方法，它返回的是一个枚举值
* or_insert 方法事实上会返回这个键的值的一个可变引用`&mut V`

##### 根据值，更新某个值

```
fn update_hashmap_3()  {
    let k1 = String::from("key1");
    let k2 = String::from("key2");
    let k3 = String::from("key3");
    let v1 = String::from("value1");
    let v2 = String::from("value2");
    let mut h1 = HashMap::new();
    h1.insert(&k1, 1);
    h1.insert(&k2, 1);
    // 因为打印下方使用了可变引用，而一个变量的可变引用和不可变引用是不能在同一个作用域中出现的，所以独立出一个作用域
    {
        // k1对应的键值对已存在，所以不会再插入
        let k1v = h1.entry(&k1).or_insert(1);
        *k1v = *k1v + 10;
    }
    println!("{:?}", h1);
}
```

*/