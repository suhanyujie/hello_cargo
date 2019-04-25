// use self::testCrate::testCrate;

pub mod instru {
    pub fn i_fun1() {
        println!("this is instru-{:#?}", "s_fun1");
    }
}

mod sound {
    pub use instru;
    pub fn s_fun1() {
        instru::i_fun1();
        println!("this is sound-{:#?}", "s_fun1");
    }
}

// 使用路径嵌套，来消除大量的 use 行
use crate::{instru as instru1,sound as sound1};

fn main() {
    // testCrate.second_fn();
    sound::s_fun1();
    // 这样使用，需要`重导出名称`
    sound::instru::i_fun1();
}

/*
## 包与模块
### 模块系统

#### 路径用来引用模块树中的项
* 种路径形式：
    * 绝对路径：从 `crate` 根开始，以 `crate` 名或者字面值 `crate` 开头
    * 相对路径：从当前模块开始，以 `self`、`super` 或当前模块的标识符开头。

### 访问权限
* 私有性规则
    * 模块是 Rust 中的 私有性边界。所有项（函数、方法、结构体、枚举、模块和常量）默认是私有的
    * 可以使用 `pub` 关键字使项变为公有。
    * 不允许使用定义于当前模块的子模块中的私有代码
    * 允许使用任何定义于父模块或当前模块中的代码。
* 增加 pub 关键字使得模块变为公有
* 对于没有 pub 关键字的项，当你从当前模块向 “下” 看时是私有的
* 使得模块公有并不使其内容也是公有的。模块上的 pub 关键字允许其父模块引用它。
* 在结构体定义中使用 pub，可以使结构体公有。然而结构体的字段仍是私有的。
* 而枚举则有所不同，它被设计为公有，则会使其所有成员公有

### 同名类型
* 将两个同名类型引入同一作用域时，可以使用`as`来讲其重命名，以避免冲突，例如：

```
use std::fmt::Result;
use std::io::Result as IoResult;
```




*/