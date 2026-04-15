# Common Collections — 堆上数据的三种核心容器

## 本质

与数组和元组不同，Vector、String 和 HashMap **都分配在堆上**，可以在运行时动态增长。它们是 Rust 日常开发中使用频率最高的三个集合类型，分别对应：**序列存储**、**文本处理**和**键值查找**。

## 关键知识点

### `Vec<T>`：可变长数组

```rust
let mut v = Vec::new();
v.push(1);
v.push(2);

let v = vec![1, 2, 3];  // 宏初始化
```

**访问元素**：索引访问会 panic，`.get()` 返回 `Option`：

```rust
let third = &v[2];          // 越界时 panic
let third = v.get(2);       // 越界时返回 None
```

**借用规则与修改**：
- 不能同时拥有不可变引用和可变引用
- 向 Vec push 元素可能导致重新分配内存，从而使已有引用失效

```rust
let mut v = vec![1, 2, 3];
let first = &v[0];   // 不可变引用
// v.push(4);        // 编译错误！
println!("{first}");
```

**遍历**：

```rust
for i in &v { println!("{i}"); }       // 不可变遍历
for i in &mut v { *i += 10; }          // 可变遍历
```

**用 enum 存储异构类型**：

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
];
```

### `String`：可增长的 UTF-8 文本

`String` 本质上是一个 `Vec<u8>` 的包装，**保证内容永远是合法的 UTF-8**。

**创建与增长**：

```rust
let mut s = String::from("foo");
s.push_str("bar");   // 追加字符串切片
s.push('!');         // 追加单个字符
```

**拼接**：`+` 运算符会夺取左操作数的所有权：

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;   // s1 被 move，s2 仍可用

// 更推荐：不转移所有权
let s = format!("{s1}-{s2}-{s3}");
```

**索引的陷阱**：

Rust **不允许**直接用数字索引访问 `String` 的字符，因为 Unicode 字符的字节长度不固定：

```rust
let hello = String::from("Здравствуйте");
// let c = hello[0];  // 编译错误！
```

正确做法是按需求取 **字节**、**标量值** 或 **字形簇**：

```rust
let s = &hello[0..4];  // 前 4 个字节："Зд"（每个字符 2 字节）

for c in hello.chars() { println!("{c}"); }
for b in hello.bytes() { println!("{b}"); }
```

### `HashMap<K, V>`：键值对映射

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
```

**访问与遍历**：

```rust
let score = scores.get("Blue").copied().unwrap_or(0);

for (key, value) in &scores {
    println!("{key}: {value}");
}
```

**所有权**：对于实现了 `Copy` 的类型（如 `i32`），值会被复制进 HashMap；对于 `String` 等类型，所有权会被 move 进去。

**更新策略**：

```rust
// 直接覆盖
scores.insert(String::from("Blue"), 25);

// 只在键不存在时才插入
scores.entry(String::from("Yellow")).or_insert(50);

// 基于旧值更新
let mut map = HashMap::new();
for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
```

## 这一章真正建立的直觉

- **Vec = 动态数组**，增长和访问都很高效，但要注意借用规则与重新分配
- **String ≠ 字符数组**，它是 UTF-8 字节流，索引操作被禁止是为了避免 Unicode 陷阱
- **HashMap = 默认键值容器**，`entry().or_insert()` 是处理"不存在则插入"最地道的写法
- 这三个集合都依赖所有权系统：**要么存储实现了 `Copy` 的值，要么 move 所有权**
