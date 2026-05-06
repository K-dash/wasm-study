// PyO3 で Rust 関数を Python から呼べるようにする最小例。
//
// Stage 2 の wasm-bindgen と意図的にシグネチャを揃えてあるので、
// 「同じ関数を Rust↔JS 用と Rust↔Python 用で出すと、Cargo.toml と
// 属性マクロの違いだけで両世界に出せる」ことを体感する。

use pyo3::prelude::*;

/// 整数の足し算。Stage 2 の wasm-bindgen 版 add と同じ。
#[pyfunction]
fn add(a: i64, b: i64) -> i64 {
    a + b
}

/// 文字列の挨拶。Stage 2 の wasm-bindgen 版 greet と同じ。
/// 比較観察: PyO3 と wasm-bindgen で「&str/String の渡し方」はどう違う？
#[pyfunction]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// list[str] -> list[int] の往復例。
/// 各単語の Unicode コードポイント数を返す（バイト数ではない）。
#[pyfunction]
fn count_chars(words: Vec<String>) -> Vec<usize> {
    words.iter().map(|s| s.chars().count()).collect()
}

/// PyO3 の #[pymodule] が Python のモジュール初期化関数を生成する。
/// pyo3 0.23+ では引数が `&Bound<'_, PyModule>` のみのスタイル。
#[pymodule]
fn pyhello(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(greet, m)?)?;
    m.add_function(wrap_pyfunction!(count_chars, m)?)?;
    Ok(())
}
