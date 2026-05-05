# Stage 1: WAT で線形メモリと境界を体感

> 各ステップごとに「やった」「観察したこと」を一言ずつ書く。
> 自己診断は終わったら自分の言葉で埋める。

---

## Step 1: add.wat → wat2wasm → wasmtime で動かす

- [x] `wat2wasm add.wat` で `add.wasm` を生成した
- [x] `wasm2wat add.wasm` で逆変換し、コメントが消えてるが構造は同じことを確認した
- [x] `wasmtime run --invoke add add.wasm 2 3` で 5 が返ってきた（experimental の warning は無視でOK）

観察:
- `.wasm` のサイズ: めちゃ少ない 41 バイト
- `wasm2wat` した結果と元 `add.wat` の差: $a -> i32, $b -> i32 になっている他、 type や export "add" が追加されている

---

## Step 2: ブラウザから読み込む

- [x] `python3 -m http.server` でこのディレクトリを配信した
- [x] ブラウザで `host.html` を開いて `add(2, 3) = 5` を確認した
- [x] DevTools の Network タブで `add.wasm` の MIME type が `application/wasm` になっていることを確認した

観察:
- DevTools の Source タブに WAT が出てくるか？: WATが出てきた。デバッグもできそう。add 関数の実態として利用されていることがわかる
- `instance.exports` に何が入っていたか？: add 関数が入っていた

---

## Step 3: mem.wat で線形メモリを触る

- [x] `wat2wasm mem.wat` で `mem.wasm` を生成した
- [x] `python3 -m http.server` を立てたまま、ブラウザで `host-mem.html` を開いた
- [x] JS → WASM の往復（read_byte で 65, 66, 255 が見える）を確認した
- [x] WASM → JS の往復（view[100]=7, view[101]=8）を確認した
- [x] 200..205 に "hello" の UTF-8 バイトが書き込まれていることを確認した

観察:
- `memory.buffer.byteLength` の値: 65536 = 64Kib
- WASM のエクスポートに登場する型は `i32, i64, f32, f64` だけ（自分で埋める）:
- 文字列を渡すために必要だったのは「offset」と「length」の 2 つだった:

個人メモ:
- memory.buffer は JS から WASM の線形メモリを読み書きやつ
- i32.store8/i32.load8_u は WASM 自身の線形メモリを読み書きするやつ

---

## Step 4: 「文字列」を WASM に渡す

- [x] `wat2wasm upper.wat` で `upper.wasm` を生成した
- [x] ブラウザで `host-upper.html` を開いて `"Hello, WebAssembly World!"` が `"HELLO, WEBASSEMBLY WORLD!"` に変換され、戻り値（書き換え件数）が 19 になることを確認した
- [x] `upper.wat` をスクロールして、ループ・条件分岐・load/store の各ブロックがどこにあるか目で追った

改造ハンズオン:
- [ ] 入力文字列を `Hello world` に変えて、戻り値が 9 になることを確認（'H' は元から大文字、空白は対象外）
- [ ] OFFSET を 0 にしても動くか試す（動くはず。WASM 側はオフセットを区別しない）
- [ ] OFFSET を 70000 にしたら何が起きるか試す（メモリ範囲外）

観察:
- WAT 内で **真偽値** はどう表現されている？: 数値というかi32。0 or そうでないか
- WASM の関数引数で `string` を渡せないのに、なぜブラウザ画面では文字列処理ができている？: JS が文字列を数値として線形メモリに書込む -> wasm（WAT）内で upper に変換 -> それを JS が読み出して文字列に変換

---

## 自己診断（Stage 1 終了時に埋める）

- [ ] JS から WASM に「文字列」を渡すには結局何が必要？
  - 自分の答え: 線形メモリにバイト列を書いた上で、(offset, length) を i32ペアで渡す
- [ ] エクスポート関数のシグネチャに `string` 型が出てこないのはなぜ？
  - 自分の答え: core wasm runtime は string 型をサポートしていないから
- [ ] `WebAssembly.Memory` と `WebAssembly.Instance` の関係は？
  - 自分の答え: memory は wasm が利用する線形メモリ、 Instance は WAT で定義した関数の実態
- [ ] 同じ `add.wasm` がブラウザでも `wasmtime` でも動いた。これは WASM のどの特性？
  - 自分の答え: portable の特性。wasm runtime は OS/CPU に依存しない wasm module を動かすためのサンドボックス環境 

---

## 疑問メモ

-

---

## 模範解答（後で見返す用）

> 自分で書いた答えは消さない。下の模範解答と差分を意識して読み返す。

### Step 1 観察

- `.wasm` のサイズ: 41 バイト（命令本体は数バイト、残りは type/function/export section のメタデータ）
- `wasm2wat` した結果と元 `add.wat` の差:
  - コメント (`;;`) はバイナリに残らない（変換時に消える）
  - ローカル名 `$a` `$b` も name section（カスタムセクション）にしか残らないため、デフォルトでは消える
  - 略記された `type` セクションが陽に展開される
  - `(export "add")` が明示される
  - 結論: バイナリにとって本質は「型シグネチャ + 命令列」、ローカル名は本質ではない

### Step 2 観察

- DevTools の Source タブ: WAT が表示され、ブレークポイントを置いて命令単位でデバッグ可能
- `instance.exports`: `add` 関数オブジェクトのみ（今回は memory も export してないので、関数 1 個だけ）

### Step 3 観察

- `memory.buffer.byteLength`: 65536（= 1 page = 64KiB）
- WASM のエクスポートに登場する**型**: `i32`, `i64`, `f32`, `f64` の 4 つだけ（SIMD 拡張で `v128` が増えるが別途）
- 文字列を渡すために必要だったのは: `offset`（メモリ上のアドレス）と `length`（バイト数）の i32 ペア

### Step 4 観察

- WAT 内の真偽値: **`i32` で表現**。0 = false、非ゼロ = true。`bool` 型は WASM に存在しない。比較命令（`i32.eq`, `i32.ge_s` 等）の結果は 0/1 の i32
- なぜ string 型なしで文字列処理できる？:
  - JS が `TextEncoder` で文字列を UTF-8 バイト列に encode してメモリに書き込む
  - WASM は「バイト列」として処理（文字列という概念は持たない、ただのバイト操作）
  - JS が結果を `TextDecoder` で復元
  - 責務分担: **JS が encode/decode、WASM はバイト処理**

### 自己診断

#### Q1: JS から WASM に「文字列」を渡すには結局何が必要？

線形メモリにバイト列を書き込み、`(offset, length)` の i32 ペアを WASM 関数に渡す。文字列は UTF-8 にエンコードしてバイト列化する。WASM 側はそれをバイト列として処理する。

#### Q2: エクスポート関数のシグネチャに `string` 型が出てこないのはなぜ？

core wasm の型システムには `i32 / i64 / f32 / f64`（+ SIMD 拡張の `v128`）しかなく、`string` という型は存在しない。これがコンポーネントモデル（WIT）が**言語中立な型**として `string` を導入する根本動機。

#### Q3: `WebAssembly.Memory` と `WebAssembly.Instance` の関係は？

`Module` から instantiate されて生まれる実行単位が `Instance`。`Memory` は `Instance` に所属するリソースの 1 つで、`Instance.exports.memory` として外に open される。

```
Module (静的)
  ↓ instantiate
Instance (動的、実行時の状態を持つ)
 ├── exports.<func>     (Function)
 ├── exports.<memory>   (Memory)   ← 線形メモリは Instance のリソース
 ├── exports.<global>   (Global)
 └── exports.<table>    (Table)
```

同じ Module を 2 回 instantiate するとそれぞれ別の Memory を持つ。複数 Instance が同じ Memory を共有することも可能（import 経由）。

#### Q4: 同じ `add.wasm` がブラウザでも `wasmtime` でも動いた。これは WASM のどの特性？

**Portable**（移植性）と **Safe**（サンドボックス）の合わせ技。WASM は OS/CPU 抽象を持たず、ホスト（ブラウザ / wasmtime）が必要な import を提供すれば同じバイナリで動く。ホストが切り替わってもバイナリは不変。
