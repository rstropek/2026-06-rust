// Topics:
// * reading from the console (stdin)
// * mutable references / borrowing (`&mut`)
// * the `Result` type and `.unwrap()`
// * turbofish syntax for generic methods (`parse::<i32>()`)
// * functions and the `#[allow(...)]` attribute
// * unit tests (`#[cfg(test)]`, `#[test]`, `assert_eq!`)

// Bring the `io` module from the standard library into scope so we can write
// `io::stdin()` instead of the fully-qualified `std::io::stdin()`.
use std::io;

fn main() {
    // Read from console.
    // `String::new()` creates an empty, growable, heap-allocated string. We need
    // `mut` because `read_line` will APPEND the typed text into this buffer, i.e.
    // it mutates `input`.
    let mut input = String::new();

    // `read_line` needs to write into our buffer, so we hand it a MUTABLE
    // REFERENCE (`&mut input`) instead of moving the String in. The function
    // borrows the buffer, fills it, and gives the borrow back when it returns.
    //
    // `read_line` returns a `Result<usize, io::Error>` -- reading can fail (I/O
    // error). `.unwrap()` says "I expect success: give me the value, or PANIC
    // (crash) if it's an `Err`." Fine for a demo; in real code we'd handle the
    // error with `match`, `?`, or `expect("...")`.
    io::stdin().read_line(&mut input).unwrap();

    // An alternative to the turbofish below: annotate the binding's type and let
    // inference flow "backwards" into `parse()`. (Shown commented out on purpose
    // so we can compare both styles.)
    //let input: i32 = input.trim().parse().unwrap();

    // `.trim()` strips surrounding whitespace -- crucially the trailing newline
    // that `read_line` leaves in the buffer when the user presses Enter.
    //
    // `parse()` is GENERIC: it can produce many types, so we must tell it which
    // one. `::<i32>` is the "turbofish" -- it pins the target type right on the
    // call. `parse` returns a `Result` (the text might not be a valid number), so
    // `.unwrap()` again extracts the `i32` or panics on bad input.
    println!("You entered: {}", input.trim().parse::<i32>().unwrap());
}

// `#[allow(dead_code)]` silences the "function is never called" warning. `add`
// is only used from the test module below, which doesn't count as a "use" in a
// normal (non-test) build, so without this attribute the compiler would warn.
#[allow(dead_code)]
fn add(a: i32, b: i32) -> i32 {
    // No `return` and no semicolon: this is the tail EXPRESSION, so it's the
    // function's return value.
    a + b
}

// `#[cfg(test)]` is CONDITIONAL COMPILATION: this whole module is compiled ONLY
// when running `cargo test`. In a normal build it disappears entirely, so test
// code adds nothing to the shipped binary.
#[cfg(test)]
mod tests {
    // The test module is a child module, so it doesn't automatically see items
    // from its parent. `use super::add;` pulls `add` in from the parent (`super`)
    // module so we can call it unqualified below.
    use super::add;

    // `#[test]` marks a function as a test. The test runner discovers every such
    // function and reports pass/fail per test.
    #[test]
    fn test_add() {
        // `assert_eq!` checks that the two arguments are equal; if not, the test
        // fails and prints both the expected and the actual value.
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }
}
