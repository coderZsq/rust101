# Hello World — 最原始的 Rust 程序

## 本质

`rustc main.rs` 手动编译，不依赖任何构建工具。理解 Rust 程序的入口就是 `fn main()`，输出靠宏 `println!`。

## 关键知识点

- **`fn main()`** — 程序入口，Rust 没有全局代码，一切从 main 开始
- **`println!`** — 这是一个**宏**（macro），不是函数，`!` 是宏的标志
- **`rustc`** — 直接用编译器编译单个文件，适合极简实验，但真实项目都用 Cargo
