@target rust
// @compile-expect: enum SystemReturnRustReturn \{
// @compile-expect: _system_return_stack: Vec<SystemReturnRustReturn>
// @compile-expect: fn _set_system_return_for_get_value\(
// @compile-expect: pub fn get_value\(&mut self\) -> i64
// @meta: rs_compile

system SystemReturnRust {
    interface:
        get_value(): i64 = 42

    machine:
        $Start {
            get_value() {
                // Exercise Rust V3 `system.return` support: assignment plus
                // handler return should be lowered to a typed return enum
                // slot update and a `return;` in the generated Rust.
                system.return = 100;
                return;
            }
        }
}
