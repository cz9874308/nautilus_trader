# 双语注释格式示例

本文档展示如何为 Rust 代码添加双语注释（中英文并存）的具体格式。

## 示例 1：模块级文档注释

### 原始代码

```rust
//! Core foundational types and utilities for [NautilusTrader](http://nautilustrader.io).
//!
//! The `nautilus-core` crate is designed to be lightweight, efficient, and to provide zero-cost abstractions
//! wherever possible. It supplies the essential building blocks used across the NautilusTrader
//! ecosystem, including:
//!
//! - Time handling and atomic clock functionality.
//! - UUID generation and management.
//! - Mathematical functions and interpolation utilities.
//! - Correctness validation functions.
//! - Serialization traits and helpers.
//! - Cross-platform environment utilities.
//! - Abstractions over common collections.
```

### 添加中文注释后

```rust
//! Core foundational types and utilities for [NautilusTrader](http://nautilustrader.io).
//! NautilusTrader 的核心基础类型和工具。
//!
//! The `nautilus-core` crate is designed to be lightweight, efficient, and to provide zero-cost abstractions
//! wherever possible. It supplies the essential building blocks used across the NautilusTrader
//! ecosystem, including:
//! `nautilus-core` crate 设计为轻量级、高效，并在可能的地方提供零成本抽象。
//! 它为整个 NautilusTrader 生态系统提供基础构建块，包括：
//!
//! - Time handling and atomic clock functionality.
//!   时间处理和原子时钟功能。
//! - UUID generation and management.
//!   UUID 生成和管理。
//! - Mathematical functions and interpolation utilities.
//!   数学函数和插值工具。
//! - Correctness validation functions.
//!   正确性验证函数。
//! - Serialization traits and helpers.
//!   序列化 trait 和辅助工具。
//! - Cross-platform environment utilities.
//!   跨平台环境工具。
//! - Abstractions over common collections.
//!   通用集合的抽象。
```

## 示例 2：函数注释

### 原始代码

```rust
/// Creates a new [`UUID4`] instance.
///
/// The UUID value is stored as a fixed-length C string byte array.
#[must_use]
pub fn new() -> Self {
    // ...
}
```

### 添加中文注释后

```rust
/// Creates a new [`UUID4`] instance.
/// 创建一个新的 [`UUID4`] 实例。
///
/// The UUID value is stored as a fixed-length C string byte array.
/// UUID 值存储为固定长度的 C 字符串字节数组。
#[must_use]
pub fn new() -> Self {
    // ...
}
```

## 示例 3：带参数的函数注释

### 原始代码

```rust
/// Converts the [`UUID4`] to a C string reference.
///
/// # Panics
///
/// Panics if the internal byte array is not a valid C string (does not end with a null terminator).
#[must_use]
pub fn to_cstr(&self) -> &CStr {
    // ...
}
```

### 添加中文注释后

```rust
/// Converts the [`UUID4`] to a C string reference.
/// 将 [`UUID4`] 转换为 C 字符串引用。
///
/// # Panics
/// # 可能 panic 的情况
///
/// Panics if the internal byte array is not a valid C string (does not end with a null terminator).
/// 如果内部字节数组不是有效的 C 字符串（不以空终止符结尾），则会 panic。
#[must_use]
pub fn to_cstr(&self) -> &CStr {
    // ...
}
```

## 示例 4：带返回值的函数注释

### 原始代码

```rust
/// Returns the raw UUID bytes (16 bytes).
///
/// This method is optimized for serialization where the UUID bytes
/// are needed directly without string conversion overhead.
#[must_use]
pub fn as_bytes(&self) -> [u8; 16] {
    // ...
}
```

### 添加中文注释后

```rust
/// Returns the raw UUID bytes (16 bytes).
/// 返回原始 UUID 字节（16 字节）。
///
/// This method is optimized for serialization where the UUID bytes
/// are needed directly without string conversion overhead.
/// 此方法针对序列化进行了优化，在需要直接使用 UUID 字节而无需字符串转换开销的场景下使用。
#[must_use]
pub fn as_bytes(&self) -> [u8; 16] {
    // ...
}
```

## 示例 5：结构体注释

### 原始代码

```rust
/// Represents a Universally Unique Identifier (UUID)
/// version 4 based on a 128-bit label as specified in RFC 4122.
#[repr(C)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct UUID4 {
    /// The UUID v4 value as a fixed-length C string byte array (includes null terminator).
    pub(crate) value: [u8; 37],
}
```

### 添加中文注释后

```rust
/// Represents a Universally Unique Identifier (UUID)
/// version 4 based on a 128-bit label as specified in RFC 4122.
/// 表示一个通用唯一标识符（UUID）版本 4，基于 RFC 4122 中指定的 128 位标签。
#[repr(C)]
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct UUID4 {
    /// The UUID v4 value as a fixed-length C string byte array (includes null terminator).
    /// UUID v4 值，作为固定长度的 C 字符串字节数组（包含空终止符）。
    pub(crate) value: [u8; 37],
}
```

## 示例 6：行内注释

### 原始代码

```rust
bytes[6] = (bytes[6] & 0x0F) | 0x40; // Set the version to 4
bytes[8] = (bytes[8] & 0x3F) | 0x80; // Set the variant to RFC 4122

value[36] = 0; // Add the null terminator
```

### 添加中文注释后

```rust
bytes[6] = (bytes[6] & 0x0F) | 0x40; // Set the version to 4
                                      // 设置版本为 4
bytes[8] = (bytes[8] & 0x3F) | 0x80; // Set the variant to RFC 4122
                                      // 设置变体为 RFC 4122

value[36] = 0; // Add the null terminator
               // 添加空终止符
```

## 示例 7：复杂函数注释（带多个章节）

### 原始代码

```rust
/// Attempts to create a [`UUID4`] from a string representation.
///
/// The string should be a valid UUID in the standard format (e.g., "2d89666b-1a1e-4a75-b193-4eb3b454c757").
///
/// # Panics
///
/// Panics if the `value` is not a valid UUID version 4 RFC 4122.
fn from_str(value: &str) -> Result<Self, Self::Err> {
    // ...
}
```

### 添加中文注释后

```rust
/// Attempts to create a [`UUID4`] from a string representation.
/// 尝试从字符串表示创建 [`UUID4`]。
///
/// The string should be a valid UUID in the standard format (e.g., "2d89666b-1a1e-4a75-b193-4eb3b454c757").
/// 字符串应该是标准格式的有效 UUID（例如，"2d89666b-1a1e-4a75-b193-4eb3b454c757"）。
///
/// # Panics
/// # 可能 panic 的情况
///
/// Panics if the `value` is not a valid UUID version 4 RFC 4122.
/// 如果 `value` 不是有效的 UUID 版本 4 RFC 4122，则会 panic。
fn from_str(value: &str) -> Result<Self, Self::Err> {
    // ...
}
```

## 示例 8：常量注释

### 原始代码

```rust
/// The maximum length of ASCII characters for a `UUID4` string value (includes null terminator).
pub(crate) const UUID4_LEN: usize = 37;
```

### 添加中文注释后

```rust
/// The maximum length of ASCII characters for a `UUID4` string value (includes null terminator).
/// `UUID4` 字符串值的最大 ASCII 字符长度（包含空终止符）。
pub(crate) const UUID4_LEN: usize = 37;
```

## 格式规则总结

1. **英文在前，中文在后**：每行英文注释后紧跟对应的中文注释
2. **保持对齐**：中文注释与英文注释对齐，便于阅读
3. **Markdown 章节**：双语章节标题，如 `# Panics` / `# 可能 panic 的情况`
4. **代码示例**：代码示例中的注释也需要双语
5. **技术术语**：保持技术术语的准确性，必要时保留英文术语
6. **行内注释**：行内注释的中文放在下一行，与英文对齐

## 注意事项

1. **不要修改代码逻辑**：只添加注释，不改变任何代码
2. **保持格式一致**：确保所有注释遵循相同的格式
3. **术语统一**：建立术语表，确保术语翻译一致
4. **验证编译**：每次修改后都要验证代码能正常编译
5. **验证文档**：确保 `cargo doc` 能正常生成文档

