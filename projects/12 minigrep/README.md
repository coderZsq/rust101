# minigrep — 用 Rust 构建命令行工具

## 本质

这一章不是讲"怎么写 grep"，而是讲**怎么用 Rust 的模块系统、错误处理和生命周期，把一个混乱的 main 函数拆成可测试、可复用的 crate**。命令行工具是载体，工程化思维才是核心。

## 关键知识点

### 分离关注点：main 与 lib

Rust 项目的惯例是将核心逻辑放入 `lib.rs`，`main.rs` 只负责参数解析和错误调度：

```rust
// main.rs — 程序的入口和胶水层
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
```

- `lib.rs` 暴露公共 API：`pub fn search(...)`、`pub struct Config`
- `main.rs` 用 `use minigrep::Config` 调用，保持简洁
- 核心逻辑可独立测试，不依赖命令行环境

### 将配置封装成 Struct

用结构体收拢分散的参数，避免 main 里一堆游离变量：

```rust
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        // ...
    }
}
```

- 构造失败返回 `Result`，调用方决定如何处理（打印错误并退出）
- `&'static str` 作为错误类型：简单场景下足够用，无需自定义 Error

### 错误处理策略

| 场景 | 处理方式 | 示例 |
|---|---|---|
| 参数解析错误 | `unwrap_or_else` + `process::exit(1)` | 用户没传够参数 |
| 运行时错误 | `if let Err(e) = ...` | 文件不存在、读取失败 |
| 内部错误传播 | `?` 运算符 + `Box<dyn Error>` | `fs::read_to_string` 失败 |

```rust
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?; // ? 自动传播错误
    // ...
    Ok(())
}
```

- `Box<dyn Error>`：不需要知道具体错误类型，任何实现了 `Error` trait 的值都能装
- `eprintln!` 向 stderr 输出，避免污染 stdout（管道友好）

### 生命周期：返回文件内容的引用

`search` 返回的是文件内容中的行引用，不是克隆的新字符串：

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line); // line 是 contents 的引用
        }
    }
    results
}
```

- `'a` 确保返回的引用不会比 `contents` 活得更久
- 编译器据此检查：调用者不能在 `contents` 被 drop 后使用返回的 `Vec`
- 这是 Rust 零成本抽象的体现：安全 + 无内存拷贝

### 环境变量控制行为

用 `env::var` 读取环境变量，实现大小写不敏感搜索：

```rust
let ignore_case = env::var("IGNORE_CASE").is_ok();
```

- `is_ok()`：只要变量存在（无论值是什么）就启用
- 运行时动态行为，无需重新编译

### 测试业务逻辑

因为核心逻辑在 `lib.rs`，可以直接写单元测试：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
```

- 测试不依赖文件系统，直接传入字符串
- `#[cfg(test)]` 保证测试代码不会被打包进 release 产物

## 这一章真正建立的直觉

- **`main.rs` 越薄越好**：参数解析 + 错误调度就是它的全部职责，业务逻辑 belongs to `lib.rs`
- **`Box<dyn Error>` 是快速原型的好朋友**：不需要定义自定义错误类型，先跑起来再细化
- **生命周期不是障碍是契约**：`Vec<&'a str>` 明确告诉调用者"我借用了你的数据"，编译器帮你 enforce
- **环境变量 = 免费的功能开关**：不需要加 CLI flag 解析库，一个 `env::var` 就能做运行时配置
- **可测试性从结构设计开始**：把 IO（读文件、读环境变量）和纯逻辑（搜索、过滤）分开，测试就是送字符串进去验输出
