// -------------------------------------------------------------------------------------------------
//  Copyright (C) 2015-2025 Nautech Systems Pty Ltd. All rights reserved.
//  https://nautechsystems.io
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
// -------------------------------------------------------------------------------------------------

#![allow(clippy::doc_markdown, reason = "Python docstrings")]

//! Python bindings and interoperability built using [`PyO3`](https://pyo3.rs).
//! 使用 [`PyO3`](https://pyo3.rs) 构建的 Python 绑定和互操作性。

#![allow(
    deprecated,
    reason = "pyo3-stub-gen currently relies on PyO3 initialization helpers marked as deprecated"
)]
//!
//! This sub-module groups together the Rust code that is *only* required when compiling the
//! `python` feature flag. It provides thin adapters so that NautilusTrader functionality can be
//! consumed from the `nautilus_trader` Python package without sacrificing type-safety or
//! performance.
//! 此子模块将仅在编译 `python` 特性标志时需要的 Rust 代码组合在一起。
//! 它提供薄适配器，以便可以从 `nautilus_trader` Python 包中使用 NautilusTrader 功能，
//! 而不会牺牲类型安全性或性能。

pub mod casing;
pub mod datetime;
pub mod enums;
pub mod parsing;
pub mod serialization;
/// String manipulation utilities for Python.
/// Python 的字符串操作工具。
pub mod string;
pub mod uuid;
pub mod version;

use std::fmt::Display;

use pyo3::{
    Py,
    conversion::IntoPyObjectExt,
    exceptions::{PyRuntimeError, PyTypeError, PyValueError},
    prelude::*,
    types::PyString,
    wrap_pyfunction,
};
use pyo3_stub_gen::derive::gen_stub_pyfunction;

use crate::{
    UUID4,
    consts::{NAUTILUS_USER_AGENT, NAUTILUS_VERSION},
    datetime::{
        MILLISECONDS_IN_SECOND, NANOSECONDS_IN_MICROSECOND, NANOSECONDS_IN_MILLISECOND,
        NANOSECONDS_IN_SECOND,
    },
};

/// Safely clones a Python object by acquiring the GIL and properly managing reference counts.
/// 通过获取 GIL 并正确管理引用计数来安全地克隆 Python 对象。
///
/// This function exists to break reference cycles between Rust and Python that can occur
/// when using `Arc<Py<PyAny>>` in callback-holding structs. The original design wrapped
/// Python callbacks in `Arc` for thread-safe sharing, but this created circular references:
/// 此函数用于打破 Rust 和 Python 之间可能发生的引用循环，当在持有回调的结构中使用 `Arc<Py<PyAny>>` 时。
/// 原始设计将 Python 回调包装在 `Arc` 中以便线程安全共享，但这创建了循环引用：
///
/// 1. Rust `Arc` holds Python objects → increases Python reference count.
///    Rust `Arc` 持有 Python 对象 → 增加 Python 引用计数。
/// 2. Python objects might reference Rust objects → creates cycles.
///    Python 对象可能引用 Rust 对象 → 创建循环。
/// 3. Neither side can be garbage collected → memory leak.
///    双方都无法被垃圾回收 → 内存泄漏。
///
/// By using plain `Py<PyAny>` with GIL-based cloning instead of `Arc<Py<PyAny>>`, we:
/// 通过使用基于 GIL 的克隆的普通 `Py<PyAny>` 而不是 `Arc<Py<PyAny>>`，我们：
/// - Avoid circular references between Rust and Python memory management.
///   避免 Rust 和 Python 内存管理之间的循环引用。
/// - Ensure proper Python reference counting under the GIL.
///   确保在 GIL 下正确的 Python 引用计数。
/// - Allow both Rust and Python garbage collectors to work correctly.
///   允许 Rust 和 Python 垃圾收集器正确工作。
///
/// # Safety
/// # 安全性
///
/// This function properly acquires the Python GIL before performing the clone operation,
/// ensuring thread-safe access to the Python object and correct reference counting.
/// 此函数在执行克隆操作之前正确获取 Python GIL，确保对 Python 对象的线程安全访问和正确的引用计数。
#[must_use]
pub fn clone_py_object(obj: &Py<PyAny>) -> Py<PyAny> {
    Python::attach(|py| obj.clone_ref(py))
}

/// Extend `IntoPyObjectExt` helper trait to unwrap `Py<PyAny>` after conversion.
/// 扩展 `IntoPyObjectExt` 辅助 trait 以在转换后解包 `Py<PyAny>`。
pub trait IntoPyObjectNautilusExt<'py>: IntoPyObjectExt<'py> {
    /// Convert `self` into a [`Py<PyAny>`] while *panicking* if the conversion fails.
    /// 将 `self` 转换为 [`Py<PyAny>`]，如果转换失败则 *panic*。
    ///
    /// This is a convenience wrapper around [`IntoPyObjectExt::into_py_any`] that avoids the
    /// cumbersome `Result` handling when we are certain that the conversion cannot fail (for
    /// instance when we are converting primitives or other types that already implement the
    /// necessary PyO3 traits).
    /// 这是围绕 [`IntoPyObjectExt::into_py_any`] 的便利包装器，当我们确定转换不会失败时避免繁琐的 `Result` 处理
    /// （例如，当我们转换已经实现必要 PyO3 trait 的基元或其他类型时）。
    #[inline]
    fn into_py_any_unwrap(self, py: Python<'py>) -> Py<PyAny> {
        self.into_py_any(py)
            .expect("Failed to convert type to Py<PyAny>")
    }
}

impl<'py, T> IntoPyObjectNautilusExt<'py> for T where T: IntoPyObjectExt<'py> {}

/// Gets the type name for the given Python `obj`.
/// 获取给定 Python `obj` 的类型名称。
///
/// # Errors
/// # 错误
///
/// Returns a error if accessing the type name fails.
/// 如果访问类型名称失败，则返回错误。
pub fn get_pytype_name<'py>(obj: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyString>> {
    obj.get_type().name()
}

/// Converts any type that implements `Display` to a Python `ValueError`.
/// 将实现 `Display` 的任何类型转换为 Python `ValueError`。
///
/// # Errors
/// # 错误
///
/// Returns a Python error with the error string.
/// 返回带有错误字符串的 Python 错误。
pub fn to_pyvalue_err(e: impl Display) -> PyErr {
    PyValueError::new_err(e.to_string())
}

/// Converts any type that implements `Display` to a Python `TypeError`.
/// 将实现 `Display` 的任何类型转换为 Python `TypeError`。
///
/// # Errors
/// # 错误
///
/// Returns a Python error with the error string.
/// 返回带有错误字符串的 Python 错误。
pub fn to_pytype_err(e: impl Display) -> PyErr {
    PyTypeError::new_err(e.to_string())
}

/// Converts any type that implements `Display` to a Python `RuntimeError`.
/// 将实现 `Display` 的任何类型转换为 Python `RuntimeError`。
///
/// # Errors
/// # 错误
///
/// Returns a Python error with the error string.
/// 返回带有错误字符串的 Python 错误。
pub fn to_pyruntime_err(e: impl Display) -> PyErr {
    PyRuntimeError::new_err(e.to_string())
}

/// Return a value indicating whether the `obj` is a `PyCapsule`.
/// 返回一个值，指示 `obj` 是否是 `PyCapsule`。
///
/// Parameters
/// ----------
/// obj : Any
///     The object to check.
///     要检查的对象。
///
/// Returns
/// -------
/// bool
#[gen_stub_pyfunction(module = "nautilus_trader.core")]
#[pyfunction(name = "is_pycapsule")]
#[allow(
    clippy::needless_pass_by_value,
    reason = "Python FFI requires owned types"
)]
#[allow(unsafe_code)]
fn py_is_pycapsule(obj: Py<PyAny>) -> bool {
    // SAFETY: obj.as_ptr() returns a valid Python object pointer
    // 安全性：obj.as_ptr() 返回有效的 Python 对象指针
    unsafe {
        // PyCapsule_CheckExact checks if the object is exactly a PyCapsule
        // PyCapsule_CheckExact 检查对象是否恰好是 PyCapsule
        pyo3::ffi::PyCapsule_CheckExact(obj.as_ptr()) != 0
    }
}

/// Loaded as `nautilus_pyo3.core`.
/// 加载为 `nautilus_pyo3.core`。
///
/// # Errors
/// # 错误
///
/// Returns a `PyErr` if registering any module components fails.
/// 如果注册任何模块组件失败，则返回 `PyErr`。
#[pymodule]
#[rustfmt::skip]
pub fn core(_: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add(stringify!(NAUTILUS_VERSION), NAUTILUS_VERSION)?;
    m.add(stringify!(NAUTILUS_USER_AGENT), NAUTILUS_USER_AGENT)?;
    m.add(stringify!(MILLISECONDS_IN_SECOND), MILLISECONDS_IN_SECOND)?;
    m.add(stringify!(NANOSECONDS_IN_SECOND), NANOSECONDS_IN_SECOND)?;
    m.add(stringify!(NANOSECONDS_IN_MILLISECOND), NANOSECONDS_IN_MILLISECOND)?;
    m.add(stringify!(NANOSECONDS_IN_MICROSECOND), NANOSECONDS_IN_MICROSECOND)?;
    m.add_class::<UUID4>()?;
    m.add_function(wrap_pyfunction!(py_is_pycapsule, m)?)?;
    m.add_function(wrap_pyfunction!(casing::py_convert_to_snake_case, m)?)?;
    m.add_function(wrap_pyfunction!(string::py_mask_api_key, m)?)?;
    m.add_function(wrap_pyfunction!(datetime::py_secs_to_nanos, m)?)?;
    m.add_function(wrap_pyfunction!(datetime::py_secs_to_millis, m)?)?;
    m.add_function(wrap_pyfunction!(datetime::py_millis_to_nanos, m)?)?;
    m.add_function(wrap_pyfunction!(datetime::py_micros_to_nanos, m)?)?;
    m.add_function(wrap_pyfunction!(datetime::py_nanos_to_secs, m)?)?;
    m.add_function(wrap_pyfunction!(datetime::py_nanos_to_millis, m)?)?;
    m.add_function(wrap_pyfunction!(datetime::py_nanos_to_micros, m)?)?;
    m.add_function(wrap_pyfunction!(datetime::py_unix_nanos_to_iso8601, m)?)?;
    m.add_function(wrap_pyfunction!(datetime::py_last_weekday_nanos, m)?)?;
    m.add_function(wrap_pyfunction!(datetime::py_is_within_last_24_hours, m)?)?;
    Ok(())
}
