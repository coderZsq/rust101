# Rust 101

跟着 [The Rust Programming Language](https://doc.rust-lang.org/book/) 学习 Rust，每个章节对应 `projects/` 下的一个项目。

## 项目索引

| 项目 | 章节 | 主题 | 核心收获 |
|------|------|------|----------|
| [1.2 hello_world](projects/1.2%20hello_world/) | Ch.1 | 最原始的 Rust 程序 | `fn main()` 是入口，`println!` 是宏 |
| [1.3 hello_cargo](projects/1.3%20hello_cargo/) | Ch.1 | Cargo 构建系统 | Cargo 是构建、依赖、测试、发布的一站式工具 |
| [2 guessing_game](projects/2%20guessing_game/) | Ch.2 | 猜数字游戏 | Rust 没有异常/null，用 `Result` + `match` 处理一切 |
| [3 common_concepts](projects/3%20common_concepts/) | Ch.3 | 基础语法全景 | 不可变是默认，`if`/`loop` 是表达式，分号改变语义 |
| [4 ownership](projects/4%20ownership/) | Ch.4 | 所有权系统 | **Rust 的灵魂** — 用编译期规则替代 GC |
| [5 structs](projects/5%20structs/) | Ch.5 | 结构体与方法 | 数据和行为分离，`impl` 块定义方法 |
| [6 enums_matching](projects/6%20enums_matching/) | Ch.6 | 枚举与模式匹配 | 用 `enum` 表达状态空间，用 `match` 穷举处理分支 |
| [7 package_crates_modules](projects/7%20package_crates_modules/) | Ch.7 | 包、Crate 与模块 | 模块是可见性边界，`use` 只是路径别名 |
| [8 common_collections](projects/8%20common_collections/) | Ch.8 | 常用集合 | Vec、String、HashMap 与所有权规则在堆上的应用 |

## 学习路径

```
hello_world  →  hello_cargo  →  guessing_game
                                     │
                                     ▼
              structs  ←  ownership  ←  common_concepts
                  │
                  ▼
          enums_matching
                  │
                  ▼
    package_crates_modules
                  │
                  ▼
         common_collections
                  │
                  ▼
         (lifetimes, traits, generics...)
```

- **Ch.1-2**：学会搭建环境和写能跑的程序
- **Ch.3**：掌握 Rust 基础语法（变量、类型、函数、控制流）
- **Ch.4 Ownership**：这是分水岭 — 理解了它，后面的生命周期、trait、并发才能顺理成章
- **Ch.5 Structs**：开始组织自定义数据类型，为 enum、trait、泛型打基础
- **Ch.6 Enums & Matching**：开始用类型表达"有限状态"和"分支逻辑"，这是理解 `Option`、`Result` 和模式匹配的关键一步
- **Ch.7 Package, Crates & Modules**：掌握 Rust 的代码组织方式 — 模块系统、可见性控制、以及如何用 Cargo 管理外部依赖
- **Ch.8 Common Collections**：深入 Vec、String 和 HashMap — 它们都分配在堆上，动态增长，是日常开发最核心的三个容器类型
