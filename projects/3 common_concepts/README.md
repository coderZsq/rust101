# Common Concepts — Rust 基础语法全景

## 本质

这一章是 Rust 语法的"速成班"，涵盖了变量、类型、函数、控制流四大基础。这些概念大部分语言都有，但 Rust 在每个细节上都有自己的设计选择。

## 关键知识点

### 变量：不可变是默认，可变是显式选择

- `let x = 5;` — 不可变，重新赋值会编译报错
- `let mut x = 5;` — 可变
- **Shadowing（遮蔽）** — `let x = x + 1` 创建新变量，可以改变类型（如 `let spaces = "   "` → `let spaces = spaces.len()`）。这和 `mut` 完全不同：mut 不能改类型，shadowing 可以
- `const MAX: u32 = 100;` — 常量必须标注类型，编译期求值，全大写命名

### 类型：Rust 是静态强类型，但能推断

- **标量类型**：整数（`i32`/`u32`等）、浮点（`f64`默认）、布尔（`bool`）、字符（`char`，Unicode）
- **复合类型**：元组（`(i32, f64, u8)`，固定长度异构）、数组（`[i32; 5]`，固定长度同构）
- 数组 `[3; 5]` 表示 5 个 3 的初始化语法

### 函数：一切皆表达式

- **语句（statement）** 不返回值，**表达式（expression）** 返回值
- `{}` 块是表达式：`let y = { let x = 3; x + 1 };` — 注意 `x + 1` 没有 `;`，有分号就变成语句
- 函数返回值用 `->` 声明，最后一个表达式就是返回值（无需 `return`）

### 控制流：if 也是表达式

- `if condition { }` — 条件**必须是 `bool`**，不会自动转换整数
- `if` 可以作为表达式赋值：`let number = if condition { 5 } else { 6 };` — 两个分支类型必须一致
- `loop` 可以返回值：`let result = loop { break counter * 2; }`
- 循环标签：`'counting_up: loop { break 'counting_up; }` 解决嵌套循环跳出问题
- `for` 是最惯用的循环：`for element in arr { }`
- Range：`1..4` 是 [1,4)，`(1..4).rev()` 反转
