use std::thread;
use std::time::Duration;


fn main() {
    // 省略大括号的写法
    let add_one_v1 = |num:u32| num+1;

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
    
    // 使用闭包
    generate_workout_use_cacher(simulated_user_specified_value, simulated_random_number);

}

fn generate_workout(intensity:u32, random_number:u32)  {
    if intensity <25 {
        println!("Today,do {} pushups", simulated_expensive_cal(intensity));
        println!("Next,do {} situp", simulated_expensive_cal(intensity));
    } else {
        if random_number == 3 {
            println!("take a break today, Remember to stay hydrated.");
        } else {
            println!("Today, run for {} minutes", simulated_expensive_cal(intensity))
        }
    }
}

fn simulated_expensive_cal(intensity:u32) ->u32 {
    println!("cal slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}



// 使用闭包重构 generate_workout
fn generate_workout_2(intensity:u32, random_number:u32)  {
    let expensive_closure = |num:u32| ->u32{
        println!("calculate ...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity <25 {
        println!("Today,do {} pushups", expensive_closure(intensity));
        println!("Next,do {} situp", expensive_closure(intensity))
    } else {
        if random_number == 3 {
            println!("take a break today, Remember to stay hydrated.");
        } else {
            println!("Today, run for {} minutes", expensive_closure(intensity))
        }
    }
}

#[derive(Debug)]
struct Cacher<T> 
    where T:Fn(u32)->u32
{
    calculation:T,
    value:Option<u32>,
}

impl<T> Cacher<T> 
    where T:Fn(u32)->u32
{
    fn new(calculation:T) ->Cacher<T> {
        Cacher {
            calculation,
            value:None,
        }
    }

    fn value(&mut self, arg:u32) ->u32  {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

// 使用Cacher重构
fn generate_workout_use_cacher(intensity:u32, random_number:u32)  {
    let mut calcu_result = Cacher::new(|num| {
        println!("calculating slowly");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity <25 {
        println!("Today,do {} pushups", calcu_result.value(intensity));
        println!("Next,do {} situp", calcu_result.value(intensity))
    } else {
        if random_number == 3 {
            println!("take a break today, Remember to stay hydrated.");
        } else {
            println!("Today, run for {} minutes", calcu_result.value(intensity))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cacher() {
        let c1 = Cacher::new(|a| a);
        let v1 = c1.value(1);
        let v2 = c1.value(2);
        assert_eq!(v1, 2);
    }
}



/*
## 闭包
* Rust 的 闭包（closures）是可以保存进变量或作为参数传递给其他函数的匿名函数。
* 闭包可以捕获当前作用域中的变量，而Rust中的函数则做不到这一点。而JavaScript中，函数是可以向上一层作用域中寻找变量。

### 闭包的定义
* 闭包的定义以一对竖线（|）开始，在竖线中指定闭包的参数；之所以选择这个语法是因为它与 Smalltalk 和 Ruby 的闭包定义类似。这个闭包有一个参数 num；如果有多于一个参数，可以使用逗号分隔，比如 |param1, param2|
* 参数之后是存放闭包体的大括号 —— 如果闭包体只有一行则大括号是可以省略的。大括号之后闭包的结尾，需要用于 let 语句的分号。
* 当然，还可以为闭包增加必要的类型注解：

```
let expensive_closure = |num:u32| ->u32 {
    println!("calculate ...")    
    thread::sleep(Duration::from_secs(2);
    num
};
```
* 省略大括号的闭包写法：

```
let add_one_v1 = |num| num+1;
```

### 带有泛型的闭包、结构体闭包
* 每一个闭包实例有其自己独有的匿名类型：也就是说，即便两个闭包有着相同的签名，他们的类型仍然可以被认为是不同。
* 所有的闭包都实现了 trait `Fn`、`FnMut` 或 `FnOnce` 中的一个。
* 一个结构体含有闭包如下所示：

```
#[derive(Debug)]
struct Cacher<T> 
    where T:Fn(u32)->u32
{
    calculation:T,
    value:Option<u32>,
}
```

### 闭包环境
* 上面提到“闭包”可以捕获环境中的变量，用于消费。这会使用内存并产生额外的开销，在更一般的场景中，当我们不需要闭包来捕获环境时，我们不希望产生这些开销。因为函数从未允许捕获环境，定义和使用函数也就从不会有这些额外开销。
* 三种捕获值的方式被编码为如下三个 `Fn` trait
    * `FnOnce` 消费从周围作用域捕获的变量，闭包周围的作用域被称为其 环境，environment。为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包。其名称的 Once 部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。
    * `FnMut` 获取可变的借用值所以可以改变其环境
    * `Fn` 从其环境获取不可变的借用值
* 如果你希望强制闭包获取其使用的环境值的所有权，可以在参数列表前使用 move 关键字。这个技巧在将闭包传递给新线程以便将数据移动到新线程中时最为实用。例如：

```

```







*/