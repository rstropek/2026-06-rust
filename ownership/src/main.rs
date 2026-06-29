// Inner attribute (`#!`) applied to the whole crate/module.
// It silences "dead code" and "unused variable" warnings so the demo stays
// readable. In real projects you would usually NOT suppress these warnings.
#![allow(dead_code, unused_variables)]

// A plain struct: a custom data type that groups named fields together.
// Point does NOT derive Copy/Clone, so it has *move semantics* (see main).
struct Point {
    x: i32,
    y: i32,
}

// `#[derive(...)]` auto-generates trait implementations for us.
// - Copy:  the type can be duplicated with a cheap, bit-for-bit copy. Because
//          of this, assignment/passing makes a COPY instead of a MOVE, so the
//          original value stays usable. Only allowed for simple, stack-only
//          types (every field must itself be Copy).
// - Clone: provides an *explicit* `.clone()` method. Copy requires Clone.
#[derive(Copy, Clone)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

// Structs can be composed of other structs. Line3D can be Copy only because
// all of its fields (Point3D) are themselves Copy.
#[derive(Copy, Clone)]
struct Line3D {
    start: Point3D,
    end: Point3D,
}

// An `impl` block holds the methods and associated functions of a type.
// (Empty for now — this is where Point's behavior would live.)
impl Point {
    // Here go your functions
}

fn main() {
    // --- Stack vs. Heap -----------------------------------------------------
    let p = Point { x: 10, y: 20 }; // Point lives on the stack
    //let p = Box::new(Point { x: 30, y: 40 }); // Box<T> puts the Point on the heap;
    //                                          // the Box (a pointer) lives on the stack.
    println!("Point coordinates: ({}, {})", p.x, p.y);

    // --- Ownership & Move semantics -----------------------------------------
    // Point is NOT Copy, so this assignment MOVES ownership from `p` to `p2`.
    let p2 = p; // Transfer of Ownership
    // println!("Point coordinates: ({}, {})", p.x, p.y); -> broken, because p is no longer valid after the ownership transfer
    println!("Point coordinates: ({}, {})", p2.x, p2.y);

    // Moving values into and out of functions:
    let p3 = create_point(); // Ownership of new_point is transferred to p3
    take_point(p3); // Ownership of p3 is MOVED into take_point; p3 unusable afterwards

    // --- Borrowing (references) ---------------------------------------------
    let p4 = create_point(); // Ownership of new_point is transferred to p4
    print_point(&p4); // We lend out a *reference* (`&`); ownership stays with p4,
                      // so p4 is still usable here.

    // --- Mutable borrowing --------------------------------------------------
    let mut p5 = create_point(); // `mut` is required to hand out a &mut reference
    manipulate_point(&mut p5); // exclusive (mutable) borrow: lets the callee change p5
    let p5_borrow = &p5; // after the &mut ends, a shared (read-only) borrow is fine.
                         // Borrow rule: many shared borrows XOR exactly one mutable borrow.
    print_point(p5_borrow);

    // --- Copy semantics -----------------------------------------------------
    let p3d = Point3D { x: 1, y: 2, z: 3 };
    let p3d_2 = p3d; // NO move: because Point3D is `Copy`, this makes a *copy*,
                     // so `p3d` remains valid below. (Copy, not just Clone, is
                     // what makes plain assignment copy instead of move.)
    let p3d_3 = p3d.clone(); // `.clone()` is the explicit way to duplicate (from Clone)
    println!("Point3D coordinates: ({}, {}, {})", p3d.x, p3d.y, p3d.z);
    println!("Point3D coordinates: ({}, {}, {})", p3d_2.x, p3d_2.y, p3d_2.z);
}

// Returns an owned Point. The last expression (no semicolon) is the return value.
fn create_point() -> Point {
    let new_point = Point { x: 0, y: 0 };
    new_point // Ownership of new_point is transferred to the caller
}

// Takes ownership: the caller's value is MOVED in.
fn take_point(p: Point) {
    println!("Point coordinates: ({}, {})", p.x, p.y);
    // Ownership of p is transferred to this function and will be dropped (freed)
    // at the end of this function — this is RAII / deterministic destruction.
}

fn print_point(p: &Point) {
    println!("Point coordinates: ({}, {})", p.x, p.y);
    // `&Point` = shared (immutable) borrow. Ownership stays with the caller;
    // we may read through the reference but not modify it.
    // Read-only borrow
}

fn manipulate_point(p: &mut Point) {
    p.x += 1;
    p.y += 1;
    // `&mut Point` = exclusive (mutable) borrow. Ownership stays with the caller,
    // but we may modify the original Point through the reference.
    // Mutable borrow
}