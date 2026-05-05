# Stage 0: ツールチェイン地図

> このファイルは**自分で埋める**ためのテンプレ。AI に答えを書かせない（咀嚼が薄くなる）。
> 各設問に自分の言葉で書き、わからない部分は「？」で残して講師に聞く。

---

## 1. WASM ターゲット triple の対応表

`rustup target list --installed` で確認した上で、各ターゲットの典型用途を自分で埋める。

| Triple | 典型出力フォーマット | 主な用途 | ホスト例 |
|---|---|---|---|
| `wasm32-unknown-unknown` | core wasm module | OS が存在しないピュア環境での動作を想定したターゲット | ブラウザ |
| `wasm32-wasip1` | WASI Preview 1 | サーバー,ライブラリ | wastime CLI |
| `wasm32-wasip2` | Component(WASI Preview 2) | サーバー,ライブラリ | コンポーネントランタイム |

ヒント:
- "core wasm module" / "WASI Preview1 module" / "component" のいずれかが入る
- ホスト例には「ブラウザ」「wasmtime CLI」「コンポーネントランタイム」など

---

## 2. ツールチェインの分岐

「wasm-bindgen 系（ブラウザ向け実用）」と「コンポーネントモデル系（言語中立 ABI）」の 2 系統に整理。

| ツール | 系統 | 役割（自分の言葉で） |
|---|---|---|
| `wasm-pack` | bindgen | npm パッケージ化までできるやつ |
| `wasm-bindgen` (CLI / crate) | bindgen | Rust<->JS 専用の glue をつくるやつ |
| `wasm-tools` | component | component開発できるやつ |
| `wit-bindgen` | component | wit から各言語のバインディングコードを生成するやつ |
| `cargo-component` | component | rust で component を作成する cargo ライブラリ |
| `wasmtime` (CLI / crate) | 両方  | core wasm runtime/preview1/preview2 を実行できるランタイム |
| `jco` | component | component を JS から呼び出せるglue を生成できるやつ |
| `wat2wasm` / `wasm2wat` |  |  |
| `maturin` | 別世界 | PyO3 クレートを Python Wheel にビルド・配布する CLI |
| `pyo3` | 別世界 | Rust を Cpython から呼び出せるライブラリ  |

ヒント: 系統列は "bindgen系" / "Component系" / "両方" / "別世界(PyO3)" で書ける。

---

## 3. 1 行で言えるか？

書けないところは戻って復習。

- [ ] core module と component の違いを 1 行で言える？
  - 自分の答え: 線形メモリと基本型しかもたないバイナリが core module, component は core module をラップし、 WIT を使って言語中立な型付きインターフェースを提供できる
- [ ] wasm-bindgen と wit-bindgen はどちらがコンポーネントモデル世界？
  - 自分の答え: wit-bindgen。wit = component で利用するくらいの理解度
- [ ] PyO3 が依存する ABI は WASM の ABI ではない。何 ABI？
  - 自分の答え: ？ Cpython の ABI
- [ ] 「.wasm」と一口に言っても 3 種類くらいフォーマットがある。並べると？
  - 自分の答え: ? core module, wasi preview1, component(wasi preview 2)

---

## 4. 疑問メモ（埋まらなかったところ）

- 
