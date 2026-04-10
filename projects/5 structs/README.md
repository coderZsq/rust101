# Structs — 用类型表达数据

## 本质

Struct 是 Rust 中组织数据的基本方式。它不像 class 那样把数据和行为绑在一起，而是**数据和方法的分离**：struct 定义数据形状，`impl` 块定义行为。这种分离让组合优于继承成为自然选择。

## 关键知识点

### 三种 Struct

| 类型 | 示例 | 用途 |
|------|------|------|
| 命名字段 struct | `struct User { name: String, age: u32 }` | 最常用，字段有名字 |
| 元组 struct | `struct Color(i32, i32, i32)` | 有名字的元组，区分类型 |
| 单元 struct | `struct AlwaysEqual` | 无字段，用于标记 trait |

### Struct Update Syntax

```rust
let user2 = User {
    email: String::from("another@example.com"),
    ..user1  // 其余字段从 user1 复制（注意：如果 user1 的字段是 String 等，会被 move）
};
```

**注意**：`..user1` 会 move `user1` 中被使用到的字段，之后 `user1` 整体不可用。

### `impl` — 方法和关联函数

- **方法**第一个参数是 `&self`（借用）或 `&mut self`（可变借用）或 `self`（获取所有权）
- **关联函数**（类似其他语言的 static method）没有 `self`，用 `::` 调用：`String::from()`、`Rectangle::square(3)`
- `Self` 是类型别名，指代当前 struct 类型
- 可以有多个 `impl` 块（这是合法的，有时按功能分组）

### `#[derive(Debug)]` — 自动打印

- struct 默认不能用 `println!("{}`" 打印
- `#[derive(Debug)]` 让编译器自动生成调试格式
- `{:?}` 单行输出，`{:#?}` 美化输出
- `dbg!(&rect1)` 打印到 stderr 并返回值，调试利器

### 从函数到方法的演进

代码中展示了清晰的演进路径：
1. `fn area(width: u32, height: u32)` — 参数太多
2. `fn area(rectangle: &Rectangle)` — 用 struct 聚合参数
3. `impl Rectangle { fn area(&self) }` — 变成方法，调用 `rect1.area()`

**本质**：方法是"第一个参数是 self 的函数"，它让代码读起来像 `rect1.can_hold(&rect2)` 这样的自然语言。

### 悬念：生命周期

代码中注释掉了含 `&str` 字段的 struct：
```rust
// struct User {
//     username: &str,  // 这需要生命周期标注
// }
```

这指向下一个关键概念：**结构体中存储引用需要生命周期标注**，这是 ownership 规则在自定义类型中的延伸。
