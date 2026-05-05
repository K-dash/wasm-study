;; ASCII 小文字 'a'..'z' を 'A'..'Z' に「in-place で」書き換える関数。
;; 戻り値は書き換えた文字数。
;;
;; これが「文字列を WASM に渡す」最小の現実例:
;;   - 文字列の中身は呼び出し前に JS が線形メモリへバイト列として配置
;;   - 関数の引数は (offset, len) の i32 ペアだけ
;;   - WASM 側は線形メモリを load/store して in-place 書き換え
;;   - 結果として JS 側の view からも書き換え結果が見える（共有メモリ）
;;
;; 新しく登場する命令:
;;   - block / loop / br_if / br : ループと break
;;   - if / then / else          : 条件分岐
;;   - i32.and / i32.ge_s / i32.le_s / i32.eq : 比較・論理積（0/1 の i32）
(module
  (memory (export "memory") 1)

  (func $to_upper (export "to_upper")
        (param $offset i32) (param $len i32)
        (result i32)
    (local $i i32)       ;; ループカウンタ (0..len)
    (local $count i32)   ;; 書き換え件数
    (local $addr i32)    ;; 現在見ているアドレス = offset + i
    (local $b i32)       ;; 現在のバイト

    ;; $i = 0; $count = 0; （local は宣言時に 0 なので明示は不要だが、教科書的に書く）
    i32.const 0
    local.set $i
    i32.const 0
    local.set $count

    ;; for (; i < len; i++) { ... } 相当を block + loop + br_if で書く
    (block $break
      (loop $continue
        ;; if (i >= len) break
        local.get $i
        local.get $len
        i32.ge_s
        br_if $break

        ;; $addr = $offset + $i
        local.get $offset
        local.get $i
        i32.add
        local.set $addr

        ;; $b = memory[$addr]   (1 byte unsigned)
        local.get $addr
        i32.load8_u
        local.set $b

        ;; if ('a' <= $b && $b <= 'z')
        local.get $b
        i32.const 0x61   ;; 'a'
        i32.ge_s
        local.get $b
        i32.const 0x7a   ;; 'z'
        i32.le_s
        i32.and          ;; -> 0 or 1

        (if
          (then
            ;; memory[$addr] = $b - 0x20   ('a' - 'A' = 0x20)
            local.get $addr
            local.get $b
            i32.const 0x20
            i32.sub
            i32.store8

            ;; $count++
            local.get $count
            i32.const 1
            i32.add
            local.set $count
          )
        )

        ;; $i++
        local.get $i
        i32.const 1
        i32.add
        local.set $i

        br $continue   ;; ループ先頭に戻る
      )
    )

    local.get $count   ;; 戻り値
  )
)
