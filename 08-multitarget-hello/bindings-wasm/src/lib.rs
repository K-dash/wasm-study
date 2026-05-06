//! wasm-bindgen façade。
//!
//! 中身は core の関数を呼ぶだけの「薄いラッパー」。
//! ここで属性マクロ (#[wasm_bindgen]) と JS 流の型変換を担当する。

use multitarget_core as core;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    core::greet(name)
}

/// JS の Number は 53bit までしか整数を扱えないので、
/// wasm-bindgen で i64 を返すと BigInt になる。混乱を避けたいので i32 ラップ。
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    core::add(a as i64, b as i64) as i32
}

/// JS の Array<string> から Vec<String> にマーシャリングして core を呼ぶ。
/// 戻り値は Uint32Array に近い形 (実体は Vec<usize>) として JS へ返る。
#[wasm_bindgen]
pub fn count_chars(words: Vec<String>) -> Vec<usize> {
    core::count_chars(&words)
}
