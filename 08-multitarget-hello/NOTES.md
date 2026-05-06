# Stage 8: 共通 core crate + 各 façade を Hello で確定

> Stage 9 で expr_ir を流し込む前に、リポジトリ骨格を Hello レベルで確定する。
> 同じ `core::greet` / `core::add` / `core::count_chars` が、bindings-wasm 経由で
> ブラウザから、bindings-py 経由で Python から呼べる状態を作る。

```
08-multitarget-hello/
├── Cargo.toml             ← workspace ルート
├── core/                   ← pure Rust (#[wasm_bindgen] や #[pyfunction] なし)
│   ├── Cargo.toml
│   └── src/lib.rs
├── bindings-wasm/          ← wasm-bindgen façade
│   ├── Cargo.toml
│   └── src/lib.rs
├── bindings-py/            ← PyO3 façade
│   ├── Cargo.toml
│   ├── pyproject.toml
│   └── src/lib.rs
└── web/index.html          ← bindings-wasm の動作確認用
```

---

## Step 1: workspace 構造を読む

- [x] ルートの `Cargo.toml` を読み、`[workspace]` の members を確認した
- [x] `core/src/lib.rs` を読み、属性マクロが**ない**ことを確認した
- [x] `bindings-wasm/src/lib.rs` を読み、core を呼ぶだけの薄いラッパーであることを確認した
- [x] `bindings-py/src/lib.rs` を読み、bindings-wasm と「属性マクロだけが違う」ことを確認した

メモ:
- なぜ `core/Cargo.toml` には `crate-type` を書かない？: bindgen,pyo3 から呼び出される純粋なコアロジックを提供するだけの責務なので
- なぜ `bindings-wasm` と `bindings-py` の両方で `crate-type = ["cdylib"]` が必要？: どちらもRust以外から使われることを想定しているので
- 同じ `core::greet` を JS と Python の両方から呼ぶためのキモは何？: coreのロジックが変わったときに両方に変更が反映されるので修正漏れがなくなる

---

## Step 2: bindings-wasm をビルド + 動作確認

- [x] `cd 08-multitarget-hello/bindings-wasm && wasm-pack build --target web` を実行した
- [x] `pkg/` ディレクトリが生成され、`multitarget_wasm.js` と `multitarget_wasm_bg.wasm` ができたことを確認した
- [x] ルートで `python3 -m http.server 8080` を起動した
- [x] `http://localhost:8080/web/` を開き、3 関数の結果が表示されることを確認した

メモ:
- `pkg/multitarget_wasm_bg.wasm` のサイズ: 17k
- Stage 2 (`02-bindgen-hello/pkg/greet_bg.wasm`) と比べてどう？: 同じ

---

## Step 3: bindings-py をビルド + 動作確認

- [x] `cd 08-multitarget-hello/bindings-py && python3 -m venv .venv` で venv 作成した
- [x] `source .venv/bin/activate` で有効化した
- [x] `pip install maturin` した
- [x] `maturin develop` でビルド + 開発インストールに成功した
- [x] `python3 -c "import multitarget_py; print(multitarget_py.greet('Yuki'))"` が動いた
- [x] add / count_chars も同様に動いた

メモ:
- bindings-py は core (../core) と pyo3 を依存に取っているが、バイナリには core のロジックも一緒に含まれている。これは static link？: static link だと思われ
- 同じ Rust core が今や JS と Python の両方から呼べる状態になった。これを実現できた最小要素は？: workspace で3つを分離し、bindgen, pyo3 の関数内で core の関数を呼び出す

---

## Step 4: 体感ポイント

- [x] `bindings-wasm/src/lib.rs` と `bindings-py/src/lib.rs` を 2 つ並べて diff を取った
- [x] 「core を変更すると 2 ターゲット両方に伝播する」を確認するため、core に新しい関数 `multiply(a, b)` を追加し、2 ターゲットから呼べるか試した（任意）

観察:
- 両 façade の差分は何行くらいだった？: 数行。pyo3の場合、pymodule の定義が追加されている。bindgen の場合、線形メモリのサイズを考慮して i64 -> i32 にキャストする処理とかが別途必要になる
- 「core 不可侵」設計のメリットを 1 文で: core を SSOT にすることでロジックが共通化され、変更漏れを防止できる。
- これをやるために**追加で書いた行数**は core 1 行 + façade 各数行で済む。これが Stage 9 で expr_ir を流し込むときの「型レイアウト確定」の意味:

---

## 自己診断

- [x] cargo workspace で複数 crate を束ねるメリットは？
  - 自分の答え: バージョン更新時に 1 箇所変えれば全 façade に反映される。crate間のバージョン差分によるトラブル防止
- [x] core crate を「pure Rust」のままにする設計上の意義は？
  - 自分の答え: core を変えると 両ターゲットに自動で伝播される点。テストも core 一箇所で済む。
- [x] 同じ core を「3 ターゲット展開」しようとした場合、どのような困難が予想される？（CI、型表現の最大公約数化、サイズ最適化）
  - 自分の答え: CI でなにをするか次第だが、CIは並列で回せるので時間は微増くらい？いずれにしても各facadeが並列実行可能なので大した問題ではない
- [x] 今の構成でブラウザ用と Python 用のビルドはどちらが先？並列にできる？
  - 自分の答え: 完全に独立しているので並列にできる。

---

## 疑問メモ

-

---

## 模範解答（後で見返す用）

> 自分で書いた答えは消さない。下の模範解答と差分を意識して読み返す。

### Step 1 メモ

#### なぜ `core/Cargo.toml` には `crate-type` を書かない？

デフォルトの **rlib** のみで十分。core は同じ workspace 内の他 Rust crate（façade）から呼ばれるだけなので、外部 ABI に晒す必要がない。`cdylib` を足すと外部 ABI 用の追加処理が無駄に入る。

#### なぜ `bindings-wasm` と `bindings-py` の両方で `crate-type = ["cdylib"]` が必要？

WASM ホスト（ブラウザ / wasmtime）と CPython という「**Rust 以外**」のコンシューマから dlopen される前提。C 互換 ABI の動的ライブラリ形式が必要。

#### 同じ `core::greet` を JS と Python の両方から呼ぶためのキモ

- core を **path 依存**（`{ path = "../core" }`）で各 façade から取り込む
- 各 façade で同じ関数を呼ぶ**薄いラッパー**を書く
- ラッパーに付ける**属性マクロ**だけが違う（`#[wasm_bindgen]` / `#[pyfunction]`）

### Step 2 メモ

#### サイズ感

`pkg/multitarget_wasm_bg.wasm` が 17k で Stage 2 の `greet_bg.wasm` と同等。理由は **core が薄いから**。expr_ir の AST + eval を Stage 9 で入れた段階で大きくなる。

### Step 3 メモ

#### core が `bindings-py` のバイナリに含まれるのは static link？

**Yes**。core は rlib として bindings-py の cdylib に**静的にリンク**される。bindings-py の `.so` には core のロジックが埋め込まれた状態で 1 ファイルになる。動的リンク（実行時に core を探しに行く）ではない。

#### 同じ Rust core が JS と Python の両方から呼べる状態を実現する最小要素

1. cargo workspace で 3 crate（core / bindings-wasm / bindings-py）を分離
2. 各 façade で core を path 依存に取り込み
3. 各 façade で `core::xxx()` を呼ぶ薄いラッパー
4. ラッパーに**ターゲット固有の属性マクロ**を付ける

### Step 4 観察

- **両 façade の差分**: 数行。pyo3 は `#[pymodule]` と関数登録テーブルが追加で必要、wasm-bindgen は JS Number の 53bit 制約により `i64 → i32` キャストが必要
- **「core 不可侵」設計のメリット**: SSOT としての core を保つことで、ロジックの共通化 + 修正漏れ防止 + 単体テストの高速化（OS/WASM 非依存）
- **Stage 9 への含意**: `core/src/` に AST + eval を追加するだけで、façade はラッパー数行追加するだけで 2 ターゲットから呼べる。これが「型レイアウトを Hello で確定」しておく効果

### 自己診断

#### Q1: cargo workspace で複数 crate を束ねるメリット

- `[workspace.dependencies]` で依存バージョン一元管理 → crate 間バージョン食い違い防止
- `target/` ディレクトリ共有でビルドキャッシュ共有（同じ依存を 2 回ビルドしない）
- `cargo test --workspace` で全 crate のテストを一括実行
- path 依存で crate 間参照が綺麗に書ける（`crates.io` を経由しない）

#### Q2: core crate を「pure Rust」のままにする設計上の意義

- core 変更が**両ターゲットに自動伝播**（修正漏れがなくなる）
- core テストは OS/WASM 非依存で**高速**に走る
- 新ターゲット（Component, CF Workers 等）追加時に**新 façade を 1 つ書くだけ**
- ビルド時に core 差分ビルドは初回 1 回のみ、以降の façade は core を再ビルドしない

#### Q3: 同じ core を「3 ターゲット展開」しようとした場合、どのような困難が予想される？（補足込み）

並列実行で時間軸の問題は小さいが、**他の次元**の困難がある:

1. **ビルドマトリクス管理コスト**: PyO3 wheel は OS × Python バージョン × アーキで数十パターン、WASM は `wasm32-unknown-unknown` と `wasm32-wasip2` で別ビルド。CI workflow が爆発する
2. **型表現の最大公約数化**: PyO3 / wasm-bindgen / Component で扱える型が違う。core は共通分で書く必要があり、`Box<dyn Trait>` や non-Send なものが使えない等の制約が出る
3. **WASM 互換性の落とし穴**: core で `tokio` `std::fs` `std::process` 等を使うと WASM ビルドが壊れる。feature flag での切り分けが必要になる
4. **サイズ最適化の非対称**: WASM は数 KB の戦い、PyO3 はそうでもない。core を肥大化させると WASM 版だけ被害（serde_json 等の依存に注意）

#### Q4: ブラウザ用と Python 用のビルドはどちらが先？並列にできる？

**完全独立、並列可能**。順序は任意。

`target/` ディレクトリが workspace で共有されるので、core の差分ビルドは初回 1 回のみ。以降は各 façade のビルド + リンクだけ走り、core は再ビルドされない。

CI では各 façade を別ジョブとして並列実行するのが王道。
