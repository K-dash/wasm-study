//! PyO3 façade。
//!
//! bindings-wasm と同じ core を依存に取り、属性マクロを #[pyfunction] に変えるだけ。
//! 「同じ Rust ロジックが、属性マクロの付替えだけで JS と Python の両方に出る」
//! を体感するのが Stage 8 の主目的。

use multitarget_core as core;
use pyo3::prelude::*;

#[pyfunction]
fn greet(name: &str) -> String {
    core::greet(name)
}

#[pyfunction]
fn add(a: i64, b: i64) -> i64 {
    core::add(a, b)
}

#[pyfunction]
fn count_chars(words: Vec<String>) -> Vec<usize> {
    core::count_chars(&words)
}

#[pymodule]
fn multitarget_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(greet, m)?)?;
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(count_chars, m)?)?;
    Ok(())
}
