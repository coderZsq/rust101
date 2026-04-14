# Package, Crates & Modules — 组织代码的边界

## 本质

Rust 的模块系统解决的是"代码该放在哪、谁能看见谁"的问题。**模块是可见性的边界，文件系统是模块的载体，`use` 只是给长路径起别名**。理解 `crate → mod → pub → use → super` 这一整条链，就能驾驭任何规模的 Rust 项目结构。

## 关键知识点

### Package vs Crate vs Module

| 概念 | 是什么 | 类比 |
|------|--------|------|
| **Package** | 一个 Cargo 项目，包含 `Cargo.toml` | 一个仓库 / 一个可分发单元 |
| **Crate** | 编译的最小单元，分 binary crate 和 library crate | 一个 `.rlib` 或一个可执行文件 |
| **Module** | 代码的逻辑分组，控制可见性 | 一个命名空间 |

- 一个 Package 最多只能有 **1 个 library crate**，但可以有 **多个 binary crate**
- `src/main.rs` 是 binary crate 的根，`src/lib.rs` 是 library crate 的根

### `mod`：声明模块存在

模块可以内联写，也可以映射到文件：

```rust
// 内联模块
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 文件映射
mod garden;  // Rust 会自动找 src/garden.rs 或 src/garden/mod.rs
```

**关键规则**：
- `mod front_of_house;` 在 `src/lib.rs` 中 → 找 `src/front_of_house.rs`
- `pub mod hosting;` 在 `src/front_of_house.rs` 中 → 找 `src/front_of_house/hosting.rs`
- 子目录里的模块必须通过父模块里的 `mod` 声明才能被编译器发现

### `pub`：打开可见性

Rust 中一切都是 **private by default**。想让外部访问，必须显式加 `pub`：

- `pub fn` — 函数公开
- `pub mod` — 模块公开（但模块内部仍然默认私有）
- `pub struct` — struct 公开，但字段默认仍私有
- `pub enum` — enum 一旦公开，所有变体自动公开

```rust
pub struct Breakfast {
    pub toast: String,      // 外部可读写
    seasonal_fruit: String, // 外部不可见
}
```

### 路径：绝对 vs 相对

- **绝对路径**：以 `crate::` 开头，从 crate 根开始
- **相对路径**：以 `self::`、`super::` 或当前模块中的名字开头

```rust
// 绝对路径
crate::front_of_house::hosting::add_to_waitlist();

// 相对路径（同一 crate 内）
front_of_house::hosting::add_to_waitlist();

// super：跳到父模块
super::deliver_order();
```

### `use`：引入路径到作用域

`use` 只是把长路径绑定成一个短名字，不改变可见性：

```rust
use std::io::{self, Write};        // 引入 io 和 io::Write
use std::collections::*;           // glob 引入（尽量少用）
use crate::front_of_house::hosting;// 引入模块本身
```

**惯用法**：
- 函数：引入到父模块 `use crate::front_of_house::hosting;` 然后 `hosting::add_to_waitlist()`
- struct/enum：直接引入 `use std::collections::HashMap;`
- 同名冲突：用 `as` 重命名

### `pub use`：重导出

`pub use` 让你把某个路径在自己的模块中重新公开出去，改变它在 API 中的暴露位置：

```rust
pub use crate::front_of_house::hosting;
```

这样外部用户可以直接用 `my_crate::hosting::add_to_waitlist()`，而不需要知道 `front_of_house` 这个内部层级。

### 外部依赖

在 `Cargo.toml` 中加入：

```toml
[dependencies]
rand = "0.8.5"
```

Cargo 会自动从 [crates.io](https://crates.io) 下载并编译。使用时直接 `use rand::Rng;`。

## 这一章真正建立的直觉

- **文件结构 ≠ 模块结构**，模块结构由 `mod` 声明决定
- **可见性是显式的**，没有 `pub` 就意味着外部看不见
- **`use` 只是别名**，不创造权限，只减少打字
- **`pub use` 是 API 设计工具**，它让你可以隐藏内部目录结构，给用户更干净的路径

项目结构从小到大的组织方式由此建立：函数 → 模块 → crate → package → 依赖生态。
