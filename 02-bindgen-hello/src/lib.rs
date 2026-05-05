// Stage 1 で「線形メモリにバイト書いて offset/length を渡す」を手動で書いた。
// この crate ではそれを wasm-bindgen に全部任せる。
//
// ポイント:
// - #[wasm_bindgen] が付いた関数は、build 時に自動で JS glue が生成される
// - &str / String / Vec<u8> / 構造体 などを「普通に」書ける
// - 関数の export 名 (JS 側で見える名前) は Rust の関数名そのまま

use wasm_bindgen::prelude::*;

/// 文字列を受け取り、文字列を返す関数。
/// この 1 行が wasm-bindgen の真骨頂。Stage 1 で書いた upper.wat 相当の
/// メモリ操作 (offset/length 計算 + UTF-8 encode/decode) は全部自動化される。
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// 比較用: i32 だけしか出てこない関数。
/// これは Stage 1 の add.wat とほぼ同じ。glue にとっても "面白くない"。
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 比較用: Vec<u8> を返す関数。
/// バイト列の所有権をどう JS に渡すかを生成 glue で見るため。
#[wasm_bindgen]
pub fn make_bytes(n: u32) -> Vec<u8> {
    (0..n as u8).collect()
}
