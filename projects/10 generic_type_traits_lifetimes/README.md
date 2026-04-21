# Generic Types, Traits, and Lifetimes — Rust 的抽象三件套

## 本质

泛型、trait 和生命周期是 Rust 类型系统的三大支柱。泛型让你用一套代码处理多种类型；trait 定义共享行为，是 Rust 实现多态的方式；生命周期则是编译器用来验证引用合法性的静态分析工具。三者常一起出现，构成 Rust"零成本抽象"的核心。

## 关键知识点

### 泛型（Generics）

泛型让你编写适用于多种类型的代码，而不牺牲运行时性能——Rust 在编译期通过**单态化**为每个具体类型生成专属代码。

**函数泛型**：

```rust
fn largest<T>(list: &[T]) -> &T
where
    T: PartialOrd,
{
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

**结构体与枚举泛型**：

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

enum Option<T> {
    Some(T),
    None,
}
```

**在 `impl` 块中使用泛型**：

```rust
impl<T, U> Point<T, U> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<T, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
```

**泛型性能**：Rust 的泛型是零成本抽象——编译器会为每个用到的具体类型生成独立代码，运行时没有任何额外开销。

### Trait：定义共享行为

Trait 是 Rust 中定义接口的方式，类似于其他语言中的 interface。

**定义 trait**：

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    // 默认实现
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

**为类型实现 trait**：

```rust
impl Summary for SocialPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

**Trait bound**：限制泛型参数必须实现某些 trait：

```rust
// impl Trait 语法（语法糖）
pub fn notify(item: &impl Summary) { ... }

// Trait bound 语法（更灵活）
pub fn notify<T: Summary>(item: &T) { ... }

// 多个 trait bound
pub fn notify<T: Summary + Display>(item: &T) { ... }

// Where 子句（复杂约束更清晰）
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}
```

**返回 `impl Trait`**：

```rust
fn returns_summarizable() -> impl Summary {
    SocialPost { ... }
}
```

注意：`impl Trait` 返回类型只能返回一种具体类型，不能根据条件返回不同类型（此时需要用 trait object）。

**Trait bound 的条件实现**：

```rust
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) { ... }
}
```

只有实现了 `Display + PartialOrd` 的 `Pair<T>` 才会有 `cmp_display` 方法。

### 生命周期（Lifetimes）

生命周期是 Rust 引用验证系统的核心——它确保引用不会比它指向的数据活得更久。

**函数中的生命周期**：

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

`'a` 表示返回值的生命周期与 `x` 和 `y` 中较短的那个相同。

**结构体中的生命周期**：

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 { 3 }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        // 省略规则：返回的 &str 自动获得 &self 的生命周期
        self.part
    }
}
```

**生命周期省略规则**：编译器会自动应用三条规则来推断生命周期，省去显式标注：

1. 每个引用参数都有自己的生命周期参数
2. 如果只有一个输入生命周期，它被赋予所有输出生命周期
3. 如果有多个输入生命周期但其中有 `&self` 或 `&mut self`，`self` 的生命周期被赋予所有输出生命周期

**静态生命周期**：`'static` 表示整个程序运行期间都有效，如字符串字面量：

```rust
let s: &'static str = "I have a static lifetime.";
```

**泛型、trait bound 与生命周期的结合**：

```rust
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() { x } else { y }
}
```

## 这一章真正建立的直觉

- **泛型 = 写一次，到处用**：通过单态化实现零成本，没有运行时开销
- **Trait = Rust 的多态**：没有继承，只有组合和 trait——用 trait bound 表达"任何实现了某行为的类型"
- **生命周期 = 引用的有效范围**：不是运行时检查，而是编译期的借用检查器的验证标注
- **三者常一起出现**：真实的 Rust 函数签名往往是 `<'a, T: SomeTrait>` 这种形式——生命周期保证引用安全，trait bound 保证类型能力，泛型保证代码复用
