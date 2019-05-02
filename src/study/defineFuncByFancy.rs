fn main() {
    trans_into_func();
}   

// 将逻辑独立，写成函数
fn trans_into_func()  {
    // 元素为i32类型的数组
    let list1 = vec![10,9,76,52,108,76];
    let largest = largest_i32(&list1);
    println!("{:?}", largest);
    // char类型的数组
    let list2 = vec!['d','o','y','o','u'];
    let largest = largest_for_char(&list2);
    println!("{:?}", largest);
    // 使用泛型定义的比较函数
    let list1 = vec![10,9,76,52,108,76,87];
    let largest = largest_in_fancy(&list1);
    println!("{:?}", largest);
}

fn largest_i32(list: &[i32]) ->i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_for_char(list:&[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 这个函数先放一放，暂时还不能运行，因为类型`T`不一定是可以进行比较的
fn largest_in_fancy(list:&[T]) ->T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[derive(Debug)]
struct Point<T> {
    x:T,
    y:T,
}

// 只是为f32类型时的Point增加方法
impl Point<f32> {
    fn test_func1(&self) {
        println!("\nthis is test_func1,value x is {}", self.x);
    }
}


/*
## 通过泛型定义函数

### 非泛型函数方式定义
* 当代码逻辑中含有相同的逻辑时，我们一般会将相同逻辑独立出来，写成函数
* 例如上方的函数`largest_i32()`和函数`largest_for_char()`
* 这2个函数的主体逻辑很相似，只是参数类型不同，如果使用泛型的方式进行封装，可以达到代码重用的效果。


### 泛型的效率
* Rust 实现了泛型，使得使用泛型类型参数的代码相比使用具体类型并没有任何速度上的损失。
* Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率。也就是说，泛型只是可能影响编译时的效率
* 更加详细的描述可查看 https://rustlang-cn.org/office/rust/book/generics/ch10-01-syntax.html#%E6%B3%9B%E5%9E%8B%E4%BB%A3%E7%A0%81%E7%9A%84%E6%80%A7%E8%83%BD

*/