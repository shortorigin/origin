(module
  (memory (export "memory") 1)
  (data (i32.const 64) "[]")
  (data (i32.const 96) "{\22strategy_id\22:\22slow\22,\22timestamp\22:\222026-03-05T00:00:00Z\22,\22state\22:{\22runtime\22:\22wasmtime\22,\22strategy_id\22:\22slow\22}}")
  (func (export "init") (param i32 i32) (result i32)
    i32.const 0)
  (func $burn (local $i i32)
    i32.const 5000000
    local.set $i
    (block $exit
      (loop $loop
        local.get $i
        i32.eqz
        br_if $exit
        local.get $i
        i32.const 1
        i32.sub
        local.set $i
        br $loop)))
  (func (export "on-market-event") (param i32 i32) (result i64)
    call $burn
    i64.const 274877906946)
  (func (export "on-timer") (param i32 i32) (result i64)
    call $burn
    i64.const 274877906946)
  (func (export "snapshot-state") (result i64)
    i64.const 412316860525)
)
