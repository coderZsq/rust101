# Guessing Game — 第一个"真正的" Rust 程序

## 本质

通过一个猜数字小游戏，串联 Rust 的核心语法：外部依赖、I/O、变量、类型转换、`match`、`loop`。这是一次从"Hello World"到"能交互的程序"的跨越。

## 关键知识点

- **外部 crate** — `rand = "0.8.5"` 在 `Cargo.toml` 声明，`use rand::Rng` 引入 trait，Cargo 自动下载和编译
- **`let mut`** — 变量默认不可变，`mut` 显式声明可变性。Rust 的哲学：安全通过默认不可变来实现
- **变量遮蔽（Shadowing）** — `let guess: u32` 重新声明了同名变量但类型不同，这**不是**赋值，而是新变量。这是 Rust 中"类型转换"的惯用方式
- **`match` 是表达式** — `match guess.trim().parse()` 对 `Result` 做 pattern matching，`Ok(num) => num` 提取值，`Err(_) => continue` 忽略错误。Rust 没有 null/异常，用 `Result` + `match` 处理错误
- **`match` 用于比较** — `guess.cmp(&secret_number)` 返回 `Ordering` 枚举，`match` 分支处理三种情况
- **`loop`** — 无限循环，用 `break` 退出。Rust 有 `loop`、`while`、`for` 三种循环
- **`&` 借用** — `&secret_number` 传引用而不转移所有权，这里第一次触碰到所有权概念
