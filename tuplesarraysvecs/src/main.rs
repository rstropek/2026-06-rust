// Silences "unused variable" warnings. We rebind the same names over and over
// in this teaching sample, so most bindings are never read. Don't do this in
// real code -- those warnings are valuable.
#[allow(unused_variables)]

// Topics:
// * tuples (incl. the unit type, destructuring, swapping)
// * arrays (fixed-size, on the stack)
// * vectors (growable, on the heap)
// * slices (borrowed views into a sequence)

fn main() {
    // TUPLES -- a fixed-length group of values that may have DIFFERENT types.
    let x = (); // The "unit type": a tuple with zero elements. Rust's "nothing"
                // value, comparable to `void` in C. It's what expressions/functions
                // return when they have no meaningful value.
    let x = (42, true); // A 2-tuple mixing an i32 and a bool. (Shadowing reuses `x`.)
    let x = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10); // Tuples can have many elements...
    let x = ((42, 13), (5, true));           // ...and can be nested arbitrarily.
    println!("{x:?}"); // `:?` is the DEBUG format -- needed because tuples don't
                       // implement the normal `Display` (`{}`) formatting.

    let x = (42, "42");
    let (a, b) = x; // DESTRUCTURING: pull the tuple apart into separate bindings.
                    // `a` becomes 42, `b` becomes "42".

    let a = 1;
    let b = 2;
    let (a, b) = (b, a); // A neat idiom: build a tuple, then destructure it to
                         // SWAP two values in a single line (a=2, b=1).

    // ARRAYS -- a fixed-size sequence where every element has the SAME type.
    // The length is part of the type and known at compile time; arrays live on
    // the stack (no heap allocation).
    let numbers = [1, 2, 3, 4, 5];          // type is `[i32; 5]`
    let numbers = [-1; 10];                 // "repeat" syntax: ten copies of -1
    let array_or_arrays = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]; // arrays nest -> a 2D grid

    const LENGTH: usize = 10; // `const` is a compile-time constant. Array lengths
                              // must be `usize`, the pointer-sized unsigned integer.
    let mut numbers = [-1; LENGTH]; // `mut` so we can change elements below.

    let first = numbers[0]; // index with `[]`; indices start at 0
    numbers[0] = 42;        // assigning to an element requires the array be `mut`
    println!("{:?}", numbers);

    // VECTORS -- a GROWABLE sequence, stored on the heap. Use a `Vec` when you
    // don't know the length up front or it needs to change at runtime.
    let mut numbers = vec![1, 2, 3, 4, 5]; // `vec!` macro builds a Vec<i32>
    numbers.push(6);                        // grow it by one element (needs `mut`)

    // Like arrays/tuples, vectors can hold any single element type -- including
    // strings, tuples, or arrays of tuples (the element type just has to match).
    let names = vec!["Alice", "Bob", "Charlie"];
    let tuples = vec![(1, "one"), (2, "two"), (3, "three")];
    let vector_of_arrays_of_tuples = vec![[("a", 1), ("b", 2)], [("c", 3), ("d", 4)]];

    // SLICES -- a BORROWED view into a contiguous run of elements. A slice does
    // not own its data; it just points into an existing array or vector.
    let numbers = vec![1, 2, 3, 4, 5];
    let slice: &[i32] = &numbers;        // borrow the WHOLE vector as a slice
    let slice: &[i32] = &numbers[1..4];  // `1..4` is a half-open range: indices
                                         // 1, 2, 3 (the end, 4, is EXCLUDED).
    calculate_length_manually(slice);    // pass the slice to a function
}

// Taking `&[i32]` (a slice) rather than `Vec<i32>` or `[i32; N]` makes this
// function flexible: it accepts a borrow of an array OR a vector OR a sub-range
// of either, without copying or taking ownership.
fn calculate_length_manually(items: &[i32]) -> usize {
    let mut count = 0;
    for _ in items {   // iterate over the slice; `_` ignores each element since
        count += 1;    // we only care about how many there are
    }
    count // tail expression -> the returned length (a `usize`)
}
