# Enums & Matching — 用类型穷举状态

## 本质

如果说 struct 适合表达"一个对象有哪些字段"，那 enum 更适合表达"一个值可能是哪几种情况之一"。Rust 用 `enum` 把状态空间建模出来，再用 `match` 强制你把每一种情况都想清楚，这就是它减少遗漏分支和空值错误的关键方式。

## 关键知识点

### `enum`：同一个类型的多种变体

- `IpAddr::V4(u8, u8, u8, u8)` 和 `IpAddr::V6(String)` 说明：**同一个 enum 的不同变体可以携带不同类型、不同形状的数据**
- `Message::Move { x: i32, y: i32 }` 展示了 enum 变体既可以像元组，也可以像匿名 struct
- 和 struct 相比，enum 不是"字段集合"，而是"可能性集合"

### `Option<T>`：Rust 用类型取代 `null`

- `Option<T>` 只有两种情况：`Some(T)` 或 `None`
- `let y: Option<i8> = Some(5);` 不能直接和普通整数相加，必须先显式处理空值情况
- **本质**：Rust 不允许你假装"这个值一定存在"，缺失值必须被代码正面处理

### `match`：穷举式分支

- `match` 会检查所有分支是否覆盖完整，这叫 **exhaustive matching**
- `Coin::Quarter(state)` 这种写法不仅在判断变体，也能顺手把内部数据解构出来
- 分支可以是表达式，所以 `match` 常被拿来返回值，比如 `plus_one`

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
```

### `_` 和绑定变量：决定你要不要吃掉剩余情况

- `_ => reroll()` 表示"其他情况我不关心，只给一个统一处理"
- `other => move_player(other)` 表示"其他情况我也接受，但我要把具体值绑定出来继续用"
- **区别**：`_` 是忽略，`other` 是接住

### `if let`：只关心一种情况时更简洁

- 当你只想匹配一个分支、其他都归为"算了"时，`if let` 比 `match` 更短
- 例如 `if let Coin::Quarter(state) = coin { ... } else { ... }`
- **经验法则**：需要穷举所有情况，用 `match`；只关心一个模式，用 `if let`

### `let ... else`：新版的早返回写法

这个例子：

```rust
let Coin::Quarter(state) = coin else {
    return None;
};
```

等价于"如果不是我想要的模式，就立刻提前返回"。它很适合把"失败路径"提前甩出去，让主逻辑更平。

### enum 也能有方法

- `impl Message { fn call(&self) { ... } }`
- `impl UsState { fn existed_in(&self, year: u16) -> bool { ... } }`
- 这说明 enum 和 struct 一样，都可以通过 `impl` 组织行为

## 这一章真正建立的直觉

- **struct** 解决的是"这个东西长什么样"
- **enum** 解决的是"这个值可能是什么"
- **match** 解决的是"每一种情况该怎么办"
- **Option/Result** 则把"可能缺失 / 可能失败"变成类型系统的一部分

学到这里，Rust 的代码风格会开始显得很不一样：很多别的语言依赖注释、约定或运行时报错来表达的东西，Rust 更倾向于直接塞进类型里，让编译器帮你兜底。
