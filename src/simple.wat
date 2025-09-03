(module
  (type (;0;) (func))
  (type (;1;) (func (param i32 i32) (result i32)))
  (type (;2;) (func (result i32)))
  (type (;3;) (func (param i32)))
  (func (;0;) (type 0)
    call 2)
  (func (;1;) (type 1) (param i32 i32) (result i32)
    (local i32)
    global.get 0
    i32.const 16
    i32.sub
    local.set 2
    local.get 2
    local.get 0
    i32.store offset=12
    local.get 2
    local.get 1
    i32.store offset=8
    local.get 2
    i32.load offset=12
    local.get 2
    i32.load offset=8
    i32.add
    return)
  (func (;2;) (type 0)
    i32.const 65536
    global.set 2
    i32.const 0
    i32.const 15
    i32.add
    i32.const -16
    i32.and
    global.set 1)
  (func (;3;) (type 2) (result i32)
    global.get 0
    global.get 1
    i32.sub)
  (func (;4;) (type 2) (result i32)
    global.get 2)
  (func (;5;) (type 2) (result i32)
    global.get 1)
  (func (;6;) (type 3) (param i32)
    local.get 0
    global.set 0)
  (func (;7;) (type 2) (result i32)
    global.get 0)
  (table (;0;) 1 1 funcref)
  (memory (;0;) 257 257)
  (global (;0;) (mut i32) (i32.const 65536))
  (global (;1;) (mut i32) (i32.const 0))
  (global (;2;) (mut i32) (i32.const 0))
  (export "memory" (memory 0))
  (export "add" (func 1))
  (export "__indirect_function_table" (table 0))
  (export "emscripten_stack_init" (func 2))
  (export "emscripten_stack_get_free" (func 3))
  (export "emscripten_stack_get_base" (func 4))
  (export "emscripten_stack_get_end" (func 5))
  (export "_emscripten_stack_restore" (func 6))
  (export "emscripten_stack_get_current" (func 7)))
