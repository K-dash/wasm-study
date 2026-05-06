# Stage 7: PyO3 + maturin の最短ルート

> 「Rust ロジックを Python から呼ぶ」現実解は WASM ではなく PyO3。
> Stage 2 (wasm-bindgen) と意図的に同じシグネチャの関数を作ってあるので、
> 「同じ Rust core を JS 向け / Python 向けに出すと何が違うか」を比較する。

---

## Step 1: crate 構造を読む

- [x] `Cargo.toml` を読み、Stage 2 の wasm-bindgen 版との差分をメモした
  - `crate-type = ["cdylib"]` は同じ（C 互換動的ライブラリ）
  - 違うのは依存（`pyo3`）と出力先（`.so`/`.pyd` ←→ `.wasm`）
- [x] `src/lib.rs` を読み、`#[pyfunction]` と `#[wasm_bindgen]` の対応関係をメモした
- [x] `pyproject.toml` を読み、maturin が build backend として動く仕組みを確認した

メモ:
- `#[pyfunction]` と `#[wasm_bindgen]` の役割は何が同じで何が違う？: バインドする関数に対してAttribute（マクロ）を付ける点は同じだが、build 後の出力先が異なる
- なぜ `pyproject.toml` 側にも features が必要？: お作法レベルでの違いしか理解できていない

疑問点:  maturin が「Python 埋め込み」モードでビルドしようとして失敗する is 何？

---

## Step 2: maturin develop でビルド + インストール

- [x] `python3 -m venv .venv` で venv 作成した
- [x] `source .venv/bin/activate` で有効化した
- [x] `pip install maturin` で maturin を入れた（システム maturin もあるが venv に入れる方が楽）
- [x] `maturin develop` でビルド + 開発インストールに成功した

メモ:
- `maturin develop` 完了時点で `.venv/lib/python3.x/site-packages/` に何が入った？:pyhello モジュールと pyhello-0.1.0.dist-info が入った pyhello-0.1.0.dist-info なにこれ？
- `.so` ファイルのサイズ: .so ファイルはなかったように見えるが？

---

## Step 3: Python から呼ぶ

- [x] `python3 -c "import pyhello; print(pyhello.add(2, 3))"` で 5 が返ってきた
- [x] `pyhello.greet("Yuki")` で `Hello, Yuki!` が返ってきた
- [x] `pyhello.count_chars(["hello", "world", "ほげ"])` で `[5, 5, 2]` が返ってきた
- [x] `help(pyhello)` で関数一覧を確認した

メモ:
- Python から見た `pyhello.add` の型情報（`type(pyhello.add)` の結果）: builtin_function_or_method であり、C拡張として登録されている
- 例外を投げる関数を書くにはどうする（PyO3 の `PyResult`）？: Result<Ok, Err> と同じ形なので、Errにエラー型を定義してあげれば python 側で例外が発生する？

---

## Step 4: wasm-bindgen と比較する

Stage 2 の wasm-bindgen 版 (`02-bindgen-hello/`) を思い出して比較。

| 観点 | wasm-bindgen | PyO3 |
|---|---|---|
| 出力フォーマット | .wasm/.js glue | .so |
| ホスト | ブラウザ | Cpythonインタプリタ |
| ABI | core wasm | Cpython |
| 文字列の渡し方 | i32のペアとバイト列 | Rust側で |
| メモリ管理 | wasm線形メモリ | OSヒープに保存され、Python で GC される |
| ビルド CLI | wasm-pack | maturin |

---

## 自己診断

- [ ] PyO3 の `.so` は WASM ファイルではない。これはなぜ？
  - 自分の答え: PyO3 と WASM は全くの別概念。.so は Cpython 拡張モジュール
- [ ] 同じ `add(a: i64, b: i64) -> i64` を wasm-bindgen と PyO3 で出すと、glue コードはどう変わる？
  - 自分の答え:
- [ ] PyO3 で例外を Python に伝えたいときは何を使う？（調べてOK）
  - 自分の答え: PyResultの Err に PyO3 で用意されている例外エラー型を定義してあげる
- [ ] PyO3 + maturin で配布するパッケージ形式は？（`.whl`）
  - 自分の答え: ?

---

## 疑問メモ

-

---

## 模範解答（後で見返す用）

> 自分で書いた答えは消さない。下の模範解答と差分を意識して読み返す。

### Step 1 メモ

#### `#[pyfunction]` と `#[wasm_bindgen]` の役割は何が同じで何が違う？

**同じ**:
- どちらも procedural macro（属性マクロ）
- Rust 関数を「外部から呼び出せる形」に変換する仕組み
- ビルド時に glue が生成される

**違う**:
- 出力先 ABI: `#[wasm_bindgen]` は core wasm export + JS glue、`#[pyfunction]` は CPython C-API の `PyMethodDef` 登録コード
- 走るホスト: ブラウザ/Node vs CPython インタプリタ
- 文字列等の受け渡し方式: 線形メモリ + (offset, length) vs `PyObject*` 参照渡し

#### なぜ `pyproject.toml` 側にも features が必要？

PyO3 には 2 つのモード（**拡張モジュール** / **Python 埋め込み**）があり、`extension-module` feature で切り替える。

- `Cargo.toml` の `features = ["extension-module"]`: その feature を**使える状態にする**宣言
- `pyproject.toml` の `[tool.maturin] features = [...]`: maturin がビルド時に**実際に有効化する**指定

両方書くのが推奨運用。書き忘れると Python 埋め込みモード（libpython にリンク）でビルドされて失敗する。

### Step 2 メモ

#### `maturin develop` 完了時点で site-packages に何が入った？

```
.venv/lib/python3.14/site-packages/
├── pyhello/
│   └── pyhello.cpython-314-darwin.so   ← .so 本体
└── pyhello-0.1.0.dist-info/
    ├── METADATA   ← パッケージ名・version・description
    ├── RECORD     ← インストール済みファイル一覧 + ハッシュ
    └── WHEEL      ← wheel フォーマットメタ
```

#### `dist-info` とは

Python パッケージの**メタデータディレクトリ**。pip / maturin がインストール時に生成。`pip uninstall` の追跡や依存解決に必要な情報を持つ。

#### `.so` ファイル名

`pyhello.cpython-314-darwin.so` の意味:
- `pyhello`: モジュール名
- `cpython-314`: Python 実装 + バージョン
- `darwin`: OS（macOS）
- `.so`: 動的ライブラリ拡張子

この命名規則のおかげで 1 つの venv に複数 Python バージョン用の `.so` が共存可能。

サイズは Stage 2 の `.wasm`（17k）より大きい（debug シンボル + Python ABI メタ込み）。release ビルドで小さくなる。

### Step 3 メモ

#### `type(pyhello.add)` の結果

`<class 'builtin_function_or_method'>`。これは PyO3 の関数が **C 拡張として登録されている**証拠。Python で書かれた関数なら `<class 'function'>` になる。

#### PyO3 で例外を投げる方法

`PyResult<T>` = `Result<T, PyErr>` の型エイリアス。`Err` に PyO3 の例外型を入れる:

```rust
use pyo3::exceptions::PyValueError;

#[pyfunction]
fn divide(a: i64, b: i64) -> PyResult<i64> {
    if b == 0 {
        Err(PyValueError::new_err("division by zero"))
    } else {
        Ok(a / b)
    }
}
```

Python 側では `ValueError: division by zero` として伝播。`pyo3::exceptions` に主要な組み込み例外（`PyValueError`, `PyTypeError`, `PyKeyError`, `PyIOError`, `PyRuntimeError` 等）が用意されてる。カスタム例外は `pyo3::create_exception!` マクロ。

Rust の `?` 演算子もそのまま使える。

### Step 4 比較表（完全版）

| 観点 | wasm-bindgen | PyO3 |
|---|---|---|
| 出力フォーマット | `.wasm` + JS glue (`*.js`) | `.so` / `.pyd` (ネイティブ cdylib) |
| ホスト | ブラウザ / Node.js | CPython インタプリタ |
| ABI | core wasm（i32/i64 + glue 規約） | **CPython C-API**（`PyObject*`、GIL） |
| 文字列の渡し方 | 線形メモリにバイト書込 + (offset, length) ペアを渡す | `PyObject*` 参照渡し（GIL 配下）。Rust 側は `&str` で受け取れるが、裏で PyO3 が UTF-8 抽出している |
| メモリ管理 | WASM 線形メモリ（独立、ホストと別世界） | OS ヒープ + Python GC が支配 |
| ビルド CLI | `wasm-pack` | `maturin` |
| ビルドターゲット triple | `wasm32-unknown-unknown` | ネイティブ（`aarch64-apple-darwin` 等） |
| 配布形式 | npm パッケージ (`pkg/`) | wheel (`.whl`) → PyPI |

### メモリの渡し方の本質（最重要）

両者の決定的な差はここ。「ptr を渡している」という表面の形は同じだが、**実体が共有されているかどうか**で実コストが大きく違う。

| 観点 | wasm-bindgen | PyO3 |
|---|---|---|
| 関数シグネチャ的にどう見える？ | (ptr, len) を渡すので**参照渡し風** | `PyObject*` を渡すので**参照渡し風** |
| 実体が共有されてるか？ | **No**（JS heap と WASM 線形メモリは別空間） | **Yes**（同じ OS メモリ空間） |
| コピーの発生 | 必ず発生（JS heap → WASM 線形メモリ） | 通常発生しない（GIL 配下で参照貸し出し） |
| 実コスト視点 | 結果的に**値渡しと同等のコスト** | **真の参照渡し** |

整理:

```
wasm-bindgen:
  メモリ空間が独立 → ptr を渡すために先にコピーが必要
  これがオーバーヘッドの源泉
  境界をまたぐたびに UTF-8 encode + バイトコピー + (offset, length) 算出

PyO3:
  同じプロセス内で OS メモリ空間を共有 → ptr 渡しがそのまま機能
  Rust 側で &str として受け取れるが、裏で PyO3 が
  PyObject* (Python str) から UTF-8 を抽出している
```

#### PyO3 も完全ゼロコピーではない場合がある

Rust の `&str` は **UTF-8** 前提だが、Python 3 の `str` は内部的に **Latin-1 / UCS-2 / UCS-4** のいずれか（PEP 393、コードポイント範囲で動的に切り替わる）。

- 内部表現が ASCII / Latin-1 → UTF-8 互換でゼロコピー
- UCS-2 / UCS-4 → UTF-8 変換が必要、小さなコピー or キャッシュ参照が発生

ただし wasm-bindgen のように **毎回必ず全バイトをコピーする**のとは規模が違う。「**多くの場合ゼロコピー、稀にコピー**」と覚える。

`Bound<'_, PyString>` で受け取れば PyObject の参照のままアクセスできて、より明示的にゼロコピーを意識できる。

#### Stage 9 以降への含意（後で効く）

- **PyO3 façade**: dict/list を受け取って評価。文字列が短いキーなら オーバーヘッド軽微、ゼロコピーで進む
- **wasm-bindgen façade**: JSON 文字列を受け取って評価。**JSON シリアライズ + 線形メモリへのコピー**が毎回入る → 起動時のレイテンシに直接効く
- 「Storefront で WASM はバンドルサイズだけでなく評価レイテンシでも JS evaluator に負ける」原因の一部がここ。**WASM はメモリ空間が独立なので、IO の境界が必ずコピー点になる**

### 自己診断

#### Q1: PyO3 の `.so` は WASM ファイルではない。なぜ？

PyO3 は**ネイティブ ABI（CPython C-API）の拡張モジュール**を生成するツール。WASM はサンドボックス環境のバイナリで別レイヤー。線形メモリも WASI も無関係。

ビルドターゲットは `wasm32-*` ではなく**ネイティブ triple**（`aarch64-apple-darwin` 等）。dlopen で CPython が読み込む形式。

#### Q2: 同じ `add(a: i64, b: i64) -> i64` を wasm-bindgen と PyO3 で出すと、glue コードはどう変わる？

- **wasm-bindgen**: `pkg/greet.js` に薄いラッパー関数。JS の `add(a, b)` が `wasm.add(a, b)` を呼ぶだけ
- **PyO3**: マクロが `PyMethodDef` テーブル登録コード + `unsafe extern "C"` ラッパーを生成。CPython がそのテーブル経由で関数を見つけて呼ぶ

**共通点**: 数値だけで完結する関数なら glue は薄い。**文字列が入ると爆発的に膨らむ**（PyO3 は `PyObject*` ↔ `&str` 変換、wasm-bindgen は線形メモリへの書込 + offset/length 計算）。

#### Q3: PyO3 で例外を Python に伝えたいときは何を使う？

`PyResult<T>` を返り値型にして、`Err` に `pyo3::exceptions` の例外型（`PyValueError`, `PyTypeError` 等）を入れる。`Result<T, PyErr>` の型エイリアスなので Rust の `?` 演算子もそのまま使える。

#### Q4: PyO3 + maturin で配布するパッケージ形式は？

**`.whl` (wheel)**。

```
maturin build         # .whl を生成
maturin publish       # PyPI へアップロード
pip install pyhello   # ユーザー側で .whl をダウンロード + 展開
```

ネイティブ依存があるので、配布側 OS / Python バージョンごとに別ファイルが必要（`pyhello-0.1.0-cp314-cp314-macosx_11_0_arm64.whl` のような名前）。CI で **manylinux / macOS arm64 / x86_64 / Windows のビルドマトリクス**を組む必要がある。これが PyO3 配布の運用コスト。
