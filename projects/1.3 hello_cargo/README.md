# Hello Cargo — Rust 的包管理器与构建系统

## 本质

Cargo 是 Rust 世界的"瑞士军刀"：构建、依赖管理、测试、发布一把梭。用 Cargo 创建项目是 Rust 开发的标准起点。

## 关键知识点

- **`cargo new`** — 自动生成标准项目结构：`Cargo.toml` + `src/main.rs`
- **`Cargo.toml`** — 项目元数据（名称、版本、edition）和依赖声明
- **`edition = "2024"`** — Rust 的版本迭代策略，不同 edition 可以引入语法变更，但编译器始终向后兼容
- **`cargo build` / `cargo run`** — 编译和编译+运行，debug 模式下产物在 `target/debug/`
- **`cargo build --release`** — 优化编译，产物在 `target/release/`
