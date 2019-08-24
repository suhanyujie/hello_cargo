fn main() {
    
}

/// 生命周期子类型 start
struct Context<'a>(&'a str);

struct Parser<'a, 's: 'a> {
    context: &'a Context<'s>,
}

impl<'a, 's> Parser<'a, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

/// 生命周期子类型 end



/*
# 高级生命周期
## 生命周期子类型
* “生命周期子类型”是为了确保某个生命周期长于另一个生命周期
* 有些情况下，我们需要多个生命周期参数，而且需要声明参数 `'a` 的生命周期不短于 `'b`，我们可以这样声明 `'a: 'b`：

```rust
struct Parser<'a, 'b: 'a> {
    context: &'a Context<'b>,
}
```

## 生命周期 bound
* 通过前面章节学习，我们应该已经知道在泛型类型上使用 trait bound 了，我们也可以为生命周期参数添加 bound，也就是“生命周期 bound”，例如：

```rust
struct Ref<'a, T: 'a>(&'a T);
```

* 其中 `T: 'a`，该结构体表示 T 中的引用的生命周期必须长于  `'a`

## trait 对象生命周期推断

## 匿名生命周期
* 匿名生命周期的写法：`'_`。它可以帮助我们编写简便的语句。

```rust
struct StrWrap<'a>(&'a str);

fn foo<'a>(string: &'a str) -> StrWrap<'a> {
    StrWrap(string)
}
```

* 如上方代码所示，函数的定义、参数、返回值都含有生命周期参数 `'a`，由于返回值中含有跟输入相关的引用，所以需要使用 `'a` 来表示。但这种写法有些冗余，此时使用“匿名生命周期”表示如下：

```rust
fn foo(string: &str) -> StrWrap<'_> {
    StrWrap(string)
}
```

* 此处使用 `'_`，意味着我们仍然知道 StrWrap 包含一个引用，不过无需所有的生命周期注解来知道。

*/