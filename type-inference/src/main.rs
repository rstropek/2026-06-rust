#![allow(unused_variables)]

fn main() {
    // --- Default integer type ---
    // No annotation and no other constraint: the compiler falls back to the
    // default integer type `i32` for an integer literal.
    let i = 42; // i: i32

    // --- Annotation on the binding ---
    // The type annotation drives inference: the literal `42` is typed as `u8`
    // because the variable is declared `u8`.
    let i: u8 = 42; // i: u8 (variable shadowing, not reassignment)

    // --- Literal type suffix ---
    // The type can also be attached directly to the literal. Here the suffix
    // `u8` fixes the type, so no annotation on the binding is needed.
    let i = 42u8; // i: u8

    // --- Inference flows "backwards" from a later constraint ---
    let user_input = "42"; // user_input: &str (string slice)
    // `parse()` is generic: `parse::<T>()`. We never write the turbofish here.
    // Instead the annotation `: i32` on the binding tells `parse` WHICH type to
    // produce. Inference is bidirectional - it does not only flow left-to-right.
    let user_input: i32 = user_input.parse().unwrap(); // user_input: i32

    // --- Unification across an expression ---
    let a = 21u16; // a: u16 (fixed by the suffix)
    // `b` would default to i32 on its own, BUT it is used in `a + b`. Rust does
    // not allow mixing integer types, so inference UNIFIES `b` with `a`'s type.
    let b = 21; // b: u16 (inferred from the addition below, not from the literal)
    let c = a + b; // c: u16

    // --- Inference from a function argument ---
    // `a` has no annotation, but it is passed to `do_something`, whose parameter
    // is `u16`. That constraint determines the type of `a`.
    let a = 42; // a: u16 (because do_something expects u16)
    do_something(a);

    // --- Inference from a return type ---
    // The type of `a` comes from the function's declared return type.
    let a = return_something(); // a: u16

    // --- Inference uses the WHOLE function body, not just earlier lines ---
    // At this line the element type of the Vec is still unknown...
    let mut numbers = Vec::new();
    // ...the later `push(42u8)` is what fixes it to `Vec<u8>`. The compiler looks
    // ahead within the scope to resolve the type.
    numbers.push(42u8); // numbers: Vec<u8>
}

fn do_something(a: u16) {
    // do something with a
}

fn return_something() -> u16 {
    // The literal `42` is inferred as `u16` because it becomes the return value,
    // and the function signature promises `u16`.
    let a = 42; // a: u16
    return a;
}
