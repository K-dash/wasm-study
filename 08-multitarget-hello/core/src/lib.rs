//! 共通ロジック core。
//!
//! ここには WASM/Python のための属性マクロを一切持ち込まない。
//! 純粋な Rust 関数として書き、各 façade (bindings-wasm / bindings-py) から
//! ラップする。これで Stage 9 以降で expr_ir のロジックを流し込むときも
//! 「core を変更すれば全ターゲットに反映」という関係が保てる。

/// 名前を受け取って挨拶文を返す。
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// 整数の足し算。
pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

/// 各単語の Unicode コードポイント数を返す (バイト数ではない)。
pub fn count_chars(words: &[String]) -> Vec<usize> {
    words.iter().map(|s| s.chars().count()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greet_works() {
        assert_eq!(greet("Yuki"), "Hello, Yuki!");
    }

    #[test]
    fn add_works() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn count_chars_works() {
        let words = vec!["hello".to_string(), "ほげ".to_string()];
        assert_eq!(count_chars(&words), vec![5, 2]);
    }
}
