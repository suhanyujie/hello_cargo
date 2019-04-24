use crate::testCrate;

fn main() {
    testCrate.second_fn();
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

*/