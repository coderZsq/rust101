# Error Handling — Rust 的错误处理哲学

## 本质

Rust 将错误分为两类：**可恢复错误**（`Result<T, E>`）和 **不可恢复错误**（`panic!`）。Rust 没有异常机制，所有错误处理都是显式的、类型驱动的。这种设计让程序中的失败路径在编译期就被强制考虑，从而大幅降低运行时意外崩溃的概率。

## 关键知识点

### `panic!`：不可恢复的错误

当程序进入一种无法继续的状态时，调用 `panic!` 宏。默认情况下，程序会展开（unwind）调用栈并打印错误信息后终止。

```rust
fn main() {
    panic!("crash and burn");
}
```

在 `Cargo.toml` 中可以配置 release 模式下的 panic 行为为直接中止，而不展开栈：

```toml
[profile.release]
panic = 'abort'
```

### `Result<T, E>`：可恢复的错误

`Result` 是一个枚举，表示操作可能失败：

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

**用 `match` 显式处理**：

```rust
let greeting_file_result = File::open("hello.txt");

let greeting_file = match greeting_file_result {
    Ok(file) => file,
    Err(error) => match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file: {:?}", e),
        },
        other_error => panic!("Problem opening the file: {:?}", other_error),
    },
};
```

### 快捷方法：`unwrap`、`expect`、`unwrap_or_else`

当上下文允许程序直接崩溃时，可以使用快捷方法减少样板代码：

```rust
let file = File::open("hello.txt").unwrap();              // 失败时 panic
let file = File::open("hello.txt").expect("自定义错误信息"); // 失败时带信息 panic

let file = File::open("hello.txt").unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
        File::create("hello.txt").unwrap_or_else(|error| {
            panic!("Problem creating the file: {:?}", error);
        })
    } else {
        panic!("Problem opening the file: {:?}", error);
    }
});
```

**注意**：`unwrap` 和 `expect` 适合原型开发、测试或确实不可能失败的场景（如硬编码值解析），生产代码中应优先使用 `match` 或 `?`。

### `?` 运算符：错误的优雅传播

`?` 运算符是 `match` 的语法糖，用于将错误从当前函数向上传播：

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

当 `?` 遇到 `Err` 时，会立即从函数返回该错误；遇到 `Ok` 时则解包出值继续执行。

`?` 还可以链式简化：

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

### `?` 与 `Option` 的结合

`?` 同样适用于 `Option`：

```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

如果 `text.lines().next()` 返回 `None`，`?` 会立即返回 `None`；否则继续执行后续操作。

### 返回 `Result` 的 `main` 函数

`main` 函数也可以返回 `Result`，这样程序中的 `?` 可以直接在顶层使用：

```rust
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("hello.txt")?;
    Ok(())
}
```

### 用类型系统保证有效性

Rust 鼓励将验证逻辑封装进类型构造函数中，让无效状态无法被表示：

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```

一旦构造出 `Guess`，就可以确信它的值一定在合法范围内。

## 这一章真正建立的直觉

- **没有异常，只有类型**：Rust 用 `Result` 和 `Option` 强制你显式处理每一个可能失败的地方
- **`?` 是错误传播的杀手锏**：它让处理 `Result` 的代码像写同步代码一样简洁，同时不隐藏失败路径
- **`unwrap` 是便利但危险的工具**：只在"真的不可能失败"或"失败就该立刻崩溃"时使用
- **类型即约束**：把验证放进构造函数，能让非法值在编译期或构造期就被拦截，而不是潜伏到运行时
