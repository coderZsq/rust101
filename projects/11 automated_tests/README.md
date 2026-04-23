# Automated Tests — 用测试守护 Rust 代码

## 本质

Rust 内置了强大的测试框架，无需第三方库即可编写和运行单元测试与集成测试。`#[test]` 标注测试函数，`cargo test` 一键执行，`assert!` 系列宏验证结果。测试不是事后的补充，而是 Rust 开发流程的一等公民。

## 关键知识点

### 编写和运行测试

测试函数用 `#[test]` 属性标注，`cargo test` 会自动发现并运行所有测试：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

- `#[cfg(test)]`：测试模块只在 `cargo test` 时编译，不会进入正式构建
- `use super::*;`：将父模块的代码引入测试作用域

### 断言宏

Rust 提供三种核心断言宏：

```rust
// assert! — 验证布尔值
assert!(larger.can_hold(&smaller));

// assert_eq! — 验证相等（左右两值实现 PartialEq + Debug）
assert_eq!(4, add_two(2));

// assert_ne! — 验证不相等
assert_ne!(result, 0);
```

**自定义失败消息**：断言宏支持格式化参数，测试失败时输出额外信息：

```rust
assert!(
    result.contains("Carol"),
    "Greeting did not contain name, value was `{}`",
    result
);
```

### 测试 panic

`should_panic` 属性验证函数是否按预期 panic：

```rust
#[test]
#[should_panic(expected = "less than or equal to 100")]
fn greater_than_100() {
    Guess::new(200);
}
```

- `expected` 参数：检查 panic 消息是否包含指定字符串，确保 panic 的原因正确
- 如果函数没有 panic，测试会失败

### 在测试中使用 Result

测试函数可以返回 `Result<T, E>`，用 `?` 运算符处理错误：

```rust
#[test]
fn it_works2() -> Result<(), String> {
    let result = add(2, 2);
    if result == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}
```

注意：返回 `Result` 的测试不能使用 `#[should_panic]`，应在 `Err` 中表明失败。

### 控制测试运行

```bash
# 运行所有测试
cargo test

# 并行运行（默认），或串行运行（避免测试间冲突）
cargo test -- --test-threads=1

# 显示 println! 输出（默认被隐藏）
cargo test -- --show-output

# 按名称过滤运行
cargo test it_adds

# 忽略某些测试（用 #[ignore] 标注），单独运行被忽略的测试
cargo test -- --ignored
```

### 单元测试 vs 集成测试

| | 单元测试 | 集成测试 |
|---|---|---|
| 位置 | `src/` 内的 `#[cfg(test)] mod tests` | 项目根目录下的 `tests/` 目录 |
| 可见性 | 能测试私有函数 | 只能使用 crate 的公共 API |
| 目的 | 验证单个模块的内部逻辑 | 验证多个模块协作是否正确 |

### 忽略测试

```rust
#[test]
#[ignore]
fn expensive_test() {
    // 耗时较长的测试
}
```

## 这一章真正建立的直觉

- **`#[test]` 就是全部配置**：不需要测试框架、不用装依赖，标注一下就是测试
- **三种断言覆盖所有场景**：`assert!` 验证真假、`assert_eq!` 验证相等、`#[should_panic]` 验证失败路径
- **测试也是 Rust 代码**：用 `Result` + `?` 处理错误，用 `use super::*` 访问私有函数，测试和业务代码共享同一套类型系统
- **`cargo test` 是瑞士军刀**：过滤、并行、串行、显示输出、忽略——一个命令全部搞定
