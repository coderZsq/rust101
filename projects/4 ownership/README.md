# Ownership — Rust 最核心的灵魂

## 本质

**Ownership 是 Rust 区别于所有其他语言的根本。** 它让 Rust 不需要垃圾回收器（GC），却能在编译期保证内存安全。理解 ownership，就理解了 Rust 为什么"难"以及为什么值得学。

三条核心规则：
1. 每个值有且只有一个**所有者**（owner）
2. 同一时刻，值只能被一个变量拥有（move 语义）
3. 所有者离开作用域，值被自动释放（drop）

## 关键知识点

### Move vs Copy — 不是所有赋值都一样

- `let s2 = s1;`（String）→ **s1 失效**，所有权转移到 s2。这叫 **move**
- `let x2 = x1;`（i32）→ 两个都有效。栈上的简单类型实现 `Copy` trait，按位复制
- 需要 deep copy？显式调用 `.clone()`
- **本质**：Rust 永远不会自动做"廉价"的深拷贝来骗你。要么 move（零成本），要么你自己 clone（显式成本）

### 借用（Borrowing）与引用（References）

- `&s` 创建不可变引用，**不获取所有权**，用完原变量还在
- `&mut s` 创建可变引用，允许修改
- **铁律**：在同一个作用域内，要么只有一个 `&mut`，要么有多个 `&`，**不能同时存在**。这是数据竞争（data race）的编译期解决方案
- **Non-lexical lifetimes**：引用的实际有效期到"最后一次使用"而非"作用域结束"，这放宽了借用规则

### 切片（Slices）— 引用的"子集"

- 字符串切片 `&s[0..2]` 是对 `String` 一部分的引用，类型是 `&str`
- `first_word(s: &str) -> &str` — 用 `&str` 而非 `&String` 作参数，函数既能接受 `&String` 也能接受 `&str` 字面量
- 数组切片 `&a[1..3]` 类型是 `&[i32]`
- **本质**：切片让函数可以操作"一段数据"而不关心数据来源，解耦了视图和所有权

### 函数与所有权

- 传参和返回值都会转移所有权
- 这意味着"把 String 传进函数，还想在外面用"需要借用或返回回来
- 借用才是惯用方式：`fn calculate_length(s: &String) -> usize`

### The Big Picture

| 场景 | 发生了什么 |
|------|-----------|
| `let s2 = s1`（String） | Move，s1 失效 |
| `let x2 = x1`（i32） | Copy，两个都有效 |
| `fn f(s: String)` | Move 进函数，外部失效 |
| `fn f(s: &String)` | 借用，外部不变 |
| `fn f(s: &mut String)` | 可变借用，外部能看到修改 |
| `fn f() -> String` | 所有权返回给调用者 |
