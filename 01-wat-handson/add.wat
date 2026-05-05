;; 最小の WAT: i32 を 2 つ受けて合計を返す関数を export するだけ。
;; これを wat2wasm でコンパイルすると 60 バイト前後の .wasm が出る。
(module
  (func $add (export "add") (param $a i32) (param $b i32) (result i32)
    local.get $a
    local.get $b
    i32.add
  )
)
