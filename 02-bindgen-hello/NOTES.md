# Stage 2: wasm-bindgen + wasm-pack でブラウザ最短ルート

> Stage 1 で手で書いた glue が「自動生成される世界」を体感する。
> 重要なのは「動かす」より「**生成された JS glue を読む**」こと。

---

## Step 1: crate 構造を読む

- [x] `Cargo.toml` を読み、`crate-type = ["cdylib"]` の意味を自分の言葉でメモした
- [x] `src/lib.rs` を読み、`#[wasm_bindgen]` がついた 3 関数の差を眺めた

メモ:
- なぜ `crate-type = ["cdylib"]` が必要？: デフォは rlibというRust専用のクレートタイプだが、これはRust同士を繋ぐために特化したメタ情報がたくさん入っているので無駄。 cdylib は C 互換の ABIの動的ライブラリであり、Rustの外から呼び出すことを前提にされたクレートタイプ。もちろん、wasm でもこれを使う。
- `#[wasm_bindgen]` を外すとどうなりそう？: ただの Rust 関数になってしまい、JS から利用できるバインディングコードが生成されない。（厳密にはマクロが噛まないので JS glue にエクスポートされない）

---

## Step 2: wasm-pack でビルド

- [x] `cd 02-bindgen-hello && wasm-pack build --target web` を実行した
- [x] `pkg/` ディレクトリが生成され、中に `.wasm` と `.js` と `.d.ts` が並んでいることを確認した
- [x] `ls -la pkg/` でサイズを記録した

メモ:
- `pkg/greet_bg.wasm` のサイズ: 17k（STAGE3のものよりもデカい）
- `pkg/greet.js` の行数（`wc -l pkg/greet.js`）: 242
- `.d.ts` の中身（型定義）: InitInput,  SyncInitInput とか

個人メモ:
- wasm_bindgen マクロを付与した関数に付いているコメントもJSバインディングコードに反映されているのね。 

---

## Step 3: ブラウザで動かす

- [x] `python3 -m http.server 8080` を `02-bindgen-hello` ディレクトリで起動した
- [x] `http://localhost:8080/web/` を開き、すべての関数の結果が表示されることを確認した
- [x] DevTools の Network タブで `pkg/greet_bg.wasm` がロードされることを確認した

---

## Step 4: 生成された glue JS を読む（このステージのキモ）

- [x] `pkg/greet.js` を開いて、関数 `greet` の glue 実装を探した
- [x] その中で「文字列 → メモリへバイト書き込み → offset/length を WASM へ渡す」流れを追った
- [x] `passStringToWasm0` のような関数の役割を理解した
- [ ] WASM 側エクスポート (`wbg__greet` などの low-level 名) の存在を確認した
- [ ] `wasm2wat pkg/greet_bg.wasm | head -50` で WAT を覗き、Stage 1 の手書き WAT と比較した

観察:
- `passStringToWasm0` がやっていることを 1 文で: 文字列を encord して、そのバイト列をwasm の線形メモリに書込みたいので malloc で書き込むメモリ番地を取得
- Stage 1 で手書きした「offset/length」が glue のどこに対応している？: const ret = wasm.greet(ptr0, len0);
- `__wbindgen_malloc` の役割は？: wasm Instance の線形メモリに len バイト分の領域を確保依頼する

個人メモ:
export 側の話
- JS 側 glue は __wbindgen_malloc(len) を呼んで「len バイト分の領域を WASM 側にお願いして確保してもらう」
- 返ってきた offset (= ポインタ) に文字列のバイトを書く
- それを (offset, len) で wasm.greet に渡す
- 用が済んだら __wbindgen_free(offset, len) で返す
つまり Stage 1 で「OFFSET = 1024 を JS 側で適当に決めた」のは素人の手抜きで、実際には WASM 側の Rust が自分のヒープアロケータで管理してるメモリを使うのが筋。それを pkg/greet.js の passStringToWasm0 が裏でやっている。

---

## 自己診断

- [ ] `wasm-bindgen` は WASM 仕様の拡張？それとも別物？
  - 自分の答え: WASM仕様の拡張。
- [ ] 同じ Rust コードを `--target web` / `--target nodejs` / `--target bundler` で出すと、何が変わる？
  - 自分の答え: バインドコードが異なる
- [ ] `pkg/greet.js` を消すと WASM はもう動かないか？理由は？
  - 自分の答え: 動かない。wasm を呼び出す窓口であるグルーコードが消えることになるので
- [ ] Component Model と wasm-bindgen の決定的な違いを 1 文で。
  - 自分の答え: Component Model はWITを使ってコンポーネントを作成・合成するやつ（まだよくわかっていないけど）。wasm-bindgen は特定言語のロジックをwasm runtime 上で実行できるようにするやつ（Rust<->JS など）

---

## 疑問メモ

-

---

## 模範解答（後で見返す用）

> 自分で書いた答えは消さない。下の模範解答と差分を意識して読み返す。

### Step 1 メモ

- `crate-type = ["cdylib"]` が必要な理由:
  - `cdylib` は **C 互換 ABI の動的ライブラリ**を出すモード（`.so` / `.dylib` / `.dll` / `.wasm`）
  - 「Rust の外」から呼ぶ前提のフォーマット
  - デフォルトの `rlib` は Rust 同士を繋ぐ専用フォーマットで、Rust 専用メタ情報を多く含むため WASM に出せない
- `#[wasm_bindgen]` を外すとどうなる:
  - マクロが噛まないので JS glue にエクスポートされない
  - 関数自体は core wasm に残るが、JS 側からは名前で呼べない（pkg/greet.js に出てこない）

### Step 2 メモ

- `pkg/greet_bg.wasm` のサイズ: 17k 程度（最適化前）。`wasm-opt -Oz` や `[profile.release] lto = true` で 5〜8k まで削れる
- `pkg/greet.js` の行数: 200〜250 行程度（バージョンによる）
- `.d.ts` の中身: TypeScript 型定義。`InitInput` / `SyncInitInput` 等の初期化用型 + 各関数のシグネチャ。Rustdoc コメントが JSDoc として転写される

### Step 4 観察

- `passStringToWasm0` がやっていること:
  1. 入力文字列を UTF-8 バイト列に encode
  2. `__wbindgen_malloc(byteLength)` で WASM 側にメモリ確保依頼
  3. 確保された offset にバイト列をコピー
  4. `(offset, length)` のペアを返す
- Stage 1 で手書きした `(offset, length)` 対応箇所: `wasm.greet(ptr0, len0)` の呼び出し。`ptr0` が offset、`len0` が length
- `__wbindgen_malloc` の役割: WASM 側 Rust の**ヒープアロケータ**に指定バイト数の領域を確保依頼する。返り値は確保された領域の offset（線形メモリ上のアドレス）。JS が「メモリのどこを使っていいか」を WASM 側に問い合わせる正しいやり方

### 自己診断

#### Q1: `wasm-bindgen` は WASM 仕様の拡張？それとも別物？

**別物（ツール / 規約レベル）**。wasm-bindgen は core wasm 仕様の上に乗る Rust↔JS 専用の慣習・ツールであって、WASM 仕様自体には何も追加しない。

仕様レベルの拡張（reference types, GC, threads, Component Model 等）は W3C / WebAssembly WG で議論される別物。混同するとコンポーネントモデルと wasm-bindgen の関係を誤解する。

#### Q2: 同じ Rust コードを `--target web` / `--target nodejs` / `--target bundler` で出すと、何が変わる？

**生成される JS glue の形式が変わる**。WASM バイナリ自体は同じ。

| target | 形式 | 用途 |
|---|---|---|
| `web` | ES Module、`init()` で fetch + instantiate | ブラウザ（プレーン HTML） |
| `nodejs` | CommonJS (`require`)、Node の `fs` でファイル読み込み | Node.js サーバ |
| `bundler` | webpack/Vite 等のバンドラ前提、`import "./xx.wasm"` 風 | Web フロント（ビルド系） |
| `no-modules` | `<script>` タグで読める古典 JS | ES Module 使えない環境 |

#### Q3: `pkg/greet.js` を消すと WASM はもう動かないか？

**動かない**。core wasm の `(import "./greet_bg.js" "...")` が未解決になり、`WebAssembly.instantiate` の段階で失敗する。

glue は単なる便利ラッパーではなく、**WASM 側からも import される必須の依存**。最近の wasm-bindgen は import が極小化されているが、それでもゼロにはならない。

#### Q4: Component Model と wasm-bindgen の決定的な違いを 1 文で

**言語中立性**の有無。

- wasm-bindgen: **Rust↔JS の 2 言語専用**の glue 生成ツール
- Component Model: **WIT という言語中立な型契約**を介して、Rust↔Go↔JS↔Python 等の任意組合せを可能にする仕様

つまり wasm-bindgen は「特定の 2 言語の間を繋ぐ便利ツール」、Component Model は「**任意の言語ペアを繋げるための共通 ABI 仕様**」。レイヤーが違う。
