// This crate-level attribute silences warnings about unused variables, unused
// `mut`, etc. We only use it here so the teaching sample compiles cleanly even
// though many bindings exist purely to demonstrate a concept. Do NOT do this in
// real code -- those warnings are valuable.
#![allow(unused_variables, unused_mut, unused)]

// Topics:
// * statements vs. expressions
// * match statement/expression

// A simple struct: a named record with two fields. Both fields are owned `i32`
// values that live inline inside the struct (no heap allocation, no pointers).
struct Point {
    x: i32,
    y: i32,
}

// Structs can contain other structs by value. A `Line` therefore embeds two
// full `Point` values directly -- its memory layout is four `i32`s laid out
// contiguously, not pointers to Points stored elsewhere.
struct Line {
    start: Point,
    end: Point,
}

fn main() {
    // `let` introduces a new variable binding. By default bindings are
    // IMMUTABLE; we add `mut` here because we want to change `i` afterwards.
    // The type is inferred as `i32` (Rust's default integer type) from `42`.
    let mut i = 42; // declare and assign
    i += 1; // allowed only because `i` is `mut`
    println!("i = {i}"); // `{i}` interpolates the variable directly into the string

    // You can split declaration and the first assignment. Rust still enforces
    // that a variable is initialized before it is read, so this is safe.
    // Note: no `mut` here, yet the assignment below is allowed because it is the
    // FIRST assignment to an as-yet-uninitialized binding, not a mutation.
    let i; // declare
    i = 42; // assign (initialization, not mutation)

    // SHADOWING: `let` with a name that already exists creates a brand-new
    // variable that hides ("shadows") the previous one. The old `i` still
    // exists but is no longer reachable by that name. Shadowing can even change
    // the type, and lets us reuse a name without needing `mut`.
    let mut i = 42;

    // A common shadowing idiom: rebinding to an immutable copy to "freeze" the
    // value so it cannot be modified in the section of code that follows.
    let i = i; // "Freeze" the variable (new immutable binding hides the mutable one)
    // do something with i, but not modify it
    let mut i = i; // "Unfreeze" the variable (rebind as mutable again)

    i += 1;

    // Constructing struct instances. `p` is immutable, so neither it nor its
    // fields can be changed after creation.
    let p = Point { x: 0, y: 0 };

    // `l` is `mut`, which makes the WHOLE value mutable. Mutability in Rust is a
    // property of the binding, not of individual fields -- so making `l` mutable
    // also lets us mutate its nested fields.
    let mut l = Line {
        start: Point { x: 0, y: 0 },
        end: Point { x: 1, y: 1 },
    };
    l.start.x = 1; // reach into the nested struct and assign a field

    // STATEMENTS vs. EXPRESSIONS -- a core Rust idea.
    // A block `{ ... }` is an EXPRESSION: it evaluates to the value of its last
    // expression (one with no trailing semicolon). Here the block runs the
    // `println!` statement and then evaluates to `42`, which is assigned to `x`.
    let x = {
        println!("Calculating x"); // statement (ends in `;`, produces no value)
        42 // tail expression -> this is the block's value
    };

    // A closure (anonymous function / "lambda"). `|args| { body }`. This one
    // takes no arguments and returns `42u8`. The `u8` suffix pins the literal's
    // type to an 8-bit unsigned integer, which also fixes the closure's return
    // type via inference.
    let lambda = || { 42u8 };
    let result = lambda(); // call it like a normal function

    let some_random_value = 5;
    // `match` is an EXPRESSION too: it evaluates to the value of the arm that
    // matches, so we can assign its result directly to `msg`. Every arm must
    // produce the same type (here: &str), and the match must be EXHAUSTIVE --
    // every possible value has to be covered, which is why `_` is required.
    // 0...4 lost, 5 draw, 6...9 win
    let msg = match some_random_value {
        0 | 1 | 2 | 3 | 4 => "You lost", // `|` matches any of several patterns
        5 => "You draw",                  // a single literal value
        6..=9 => "You win",               // `..=` is an inclusive range pattern
        n if n > 10 => "Too high",        // `n if ...` is a "match guard": binds n, then tests a condition
        _ => "Invalid value",             // `_` is the catch-all wildcard, makes the match exhaustive
    };
    println!("{msg}");
 }

// A function returning `i32` (declared with `-> i32`). The last line is the
// return value: `result` has no semicolon, so it is the function's tail
// EXPRESSION. We could also write `return result;`, but the expression form is
// idiomatic Rust.
fn add(x: i32, y: i32) -> i32 {
    println!("Doing some math"); // statement
    let result = x + y;          // statement
    result                       // tail expression -> the returned value
}

fn div(x: i32, y: i32) -> i32 {
    // `if` is an EXPRESSION in Rust, not just a control-flow statement. Both
    // branches evaluate to an `i32`, and the whole `if/else` becomes the
    // function's return value. (Both arms must have the same type.)
    // if-expression, not statement
    if y == 0 { 0 } else { x / y }
}

fn get_sum() -> i32 {
    let mut sum = 0;
    let mut counter = 1;
    // `loop` is an infinite loop -- and, like other blocks, it is an EXPRESSION.
    // `break sum;` exits the loop AND hands `sum` back as the loop's value,
    // which (being the function's tail expression) becomes the return value.
    loop {
        sum += counter;
        counter += 1;
        if counter > 100 {
            break sum; // break WITH a value -> this is what `loop` evaluates to
        }
    }
}