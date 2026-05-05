;; 線形メモリを export して、JS と WASM の双方向に読み書きする最小例。
;;
;; ポイント:
;; - (memory (export "memory") 1) で 1 page (64KiB) を確保し、JS に open する
;; - i32.load8_u / i32.store8 が「メモリの 1 バイトを読む / 書く」命令
;;   * 第1引数（オフセット）はスタック先頭から取られる（スタックマシンなので）
;; - ここには「文字列」も「配列」も登場しない。あるのはバイト列とアドレス(i32)だけ
(module
  ;; 1 page = 65,536 bytes の線形メモリを確保し、name "memory" で export
  (memory (export "memory") 1)

  ;; メモリ[offset] の 1 バイトを符号なし整数として返す
  (func $read_byte (export "read_byte") (param $offset i32) (result i32)
    local.get $offset
    i32.load8_u
  )

  ;; メモリ[offset] に value（の下位 8bit）を書き込む
  (func $write_byte (export "write_byte") (param $offset i32) (param $value i32)
    local.get $offset
    local.get $value
    i32.store8
  )
)
