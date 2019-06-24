(module
    (func $f0 (export "hello") (param i32) (param i32) (result i32)

        i32.const 32
    )
    (func $f1 (param i32) (result i32) (local f64) (local i32) (local i32) (local f64) (local i32)
        i32.const 32
    )
    (func $f2 (param i32) (param $i i32) (result i32)
        i32.const 32
    )

    (memory 1 2)

    (data 0 (i32.const 1) "hello")
)
