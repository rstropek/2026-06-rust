//! # Traits, Traits, Traits — a guided tour of Rust's trait system
//!
//! This sample walks through the most important building blocks of Rust traits
//! using a small "billing" domain (consulting work, flat fees, ...). Read it
//! top to bottom — each section builds on the previous one. The concepts we
//! covered, in order of appearance:
//!
//! 1. **`Default`** — a standard-library trait for "give me a sensible value".
//! 2. **Defining a trait** with *required* and *provided* (default) methods.
//! 3. **Implementing a trait** for our own type.
//! 4. **Implementing traits for foreign/primitive types** (e.g. `f32`) and the
//!    fact that method names can collide (`abs`).
//! 5. **The newtype pattern** (`FlatFee`) and **overriding a default method**.
//! 6. **`impl Trait` in argument position** (static dispatch / generics).
//! 7. **`impl Trait` in return position** and its "single concrete type" rule.
//! 8. **Blanket implementations** — implement a trait for *every* type that
//!    already implements another trait.
//! 9. **Implementing a trait for arrays** using const generics.
//! 10. **Dynamic dispatch** with trait objects (`Box<dyn Trait>`).
//! 11. **`Display`**, **supertraits**, and **trait bounds** via `where`.
//! 12. **Associated types** vs. **generic type parameters** on a trait.
//!
//! Some blocks are intentionally left commented out — they are the
//! "why doesn't this compile?" counter-examples we discussed live.

// We keep a few unused items around purely for teaching, so silence the
// warnings that would otherwise clutter the build output.
#![allow(dead_code, unused_variables)]

use std::fmt::Display;

// ---------------------------------------------------------------------------
// 1. The domain type + the `Default` trait
// ---------------------------------------------------------------------------

/// A unit of consulting work we might want to bill for.
struct ConsultingWork {
    what: String,
    hours: f32,
    rate: f32,
}

// `Default` is a standard-library trait. Implementing it lets callers write
// `ConsultingWork::default()` or `Default::default()` to get a ready-made value
// without spelling out every field. Many std APIs rely on `Default`.
impl Default for ConsultingWork {
    fn default() -> Self {
        Self {
            what: String::from("Rust Consulting"),
            hours: 8.0,
            rate: 100.0,
        }
    }
}

// ---------------------------------------------------------------------------
// 2. Defining a trait: required vs. provided methods
// ---------------------------------------------------------------------------

/// Anything that can appear on an invoice.
///
/// A trait is a set of behavior that types can opt into (similar to an
/// interface in other languages, but more powerful).
trait Billable {
    // Required methods: no body here, so every implementor MUST provide one.
    fn description(&self) -> &str;
    fn total(&self) -> f32;

    // Provided (default) method: it already has a body built from the required
    // methods above. Implementors get it "for free" but may override it (see
    // `FlatFee` below).
    fn formatted_total(&self) -> String {
        format!("${:.2} for {}", self.total(), self.description())
    }
}

// ---------------------------------------------------------------------------
// 3. Implementing a trait for our own type
// ---------------------------------------------------------------------------

impl Billable for ConsultingWork {
    fn description(&self) -> &str {
        &self.what
    }

    fn total(&self) -> f32 {
        self.hours * self.rate
    }
    // Note: we do NOT implement `formatted_total`, so `ConsultingWork` uses the
    // default implementation from the trait.
}

// ---------------------------------------------------------------------------
// 4. Implementing traits for foreign / primitive types
// ---------------------------------------------------------------------------

// Traits let us add behavior to types we didn't define — even primitives like
// `f32`. This ad-hoc trait exists to demonstrate method-name resolution:
// `f32` ALREADY has an inherent `abs` method, so our trait's `abs` "shadows"
// nothing — inherent methods win unless we disambiguate (see `main`).
trait AbsTrait {
    fn abs(self) -> f32;
}
impl AbsTrait for f32 {
    fn abs(self) -> f32 {
        if self < 0.0 { -self } else { self }
    }
}

// We can also make a bare `f32` `Billable` (a flat fee is just a number).
// This is why `print_total(&200.0_f32)` works later on.
impl Billable for f32 {
    fn description(&self) -> &str {
        "Flat fee"
    }

    fn total(&self) -> f32 {
        *self
    }
}

// ---------------------------------------------------------------------------
// 5. The newtype pattern + overriding a default method
// ---------------------------------------------------------------------------

/// The "newtype" pattern: a tuple struct wrapping a single `f32`.
///
/// Wrapping the primitive in a distinct type gives it its own identity and
/// lets us attach a *different* trait implementation than plain `f32` has.
struct FlatFee(f32);

impl Billable for FlatFee {
    fn description(&self) -> &str {
        "Flat fee"
    }

    fn total(&self) -> f32 {
        self.0 // access the wrapped value by position
    }

    // Here we OVERRIDE the trait's default `formatted_total`, so `FlatFee`
    // formats itself differently from `ConsultingWork`.
    fn formatted_total(&self) -> String {
        format!("{} costs ${:.2}", self.description(), self.total())
    }
}

// ---------------------------------------------------------------------------
// 6. `impl Trait` in argument position (static dispatch)
// ---------------------------------------------------------------------------

// `&impl Billable` means "a reference to some concrete type that implements
// `Billable`". The compiler generates a specialized copy of this function for
// each type it's called with (monomorphization) — zero runtime cost. It is
// shorthand for `fn print_total<T: Billable>(item: &T)`.
fn print_total(item: &impl Billable) {
    println!("{}", item.formatted_total());
}

// ---------------------------------------------------------------------------
// 7. `impl Trait` in return position
// ---------------------------------------------------------------------------

// COUNTER-EXAMPLE (left commented on purpose):
// `-> impl Billable` promises the caller ONE specific hidden type. You cannot
// return `f32` from one branch and `ConsultingWork` from another, because those
// are different concrete types. To do that you'd need dynamic dispatch (see
// `Box<dyn Billable>` further down).
/*
fn generate_billable() -> impl Billable {
    if true /* imagine: some runtime condition */ {
        150.0_f32 // returns a flat fee
    } else {
        ConsultingWork::default() // does NOT work with impl
    }
}
*/

// This version compiles: every path returns the same concrete type (`f32`).
fn generate_billable() -> impl Billable {
    150.0_f32 // returns a flat fee
}

fn generate_abs() -> impl AbsTrait {
    -150.0_f32 // returns a flat fee
}

// ---------------------------------------------------------------------------
// 8. Blanket implementations
// ---------------------------------------------------------------------------

/// Something worth loyalty points.
trait Pointworthy {
    fn points(&self) -> u32;
}

// A *blanket implementation*: "for EVERY type `T` that is `Billable`, here is
// how it becomes `Pointworthy`." We never implement `Pointworthy` by hand —
// every `Billable` gets it automatically. (This is how std provides `ToString`
// for anything that is `Display`, for example.)
impl<T: Billable> Pointworthy for T {
    fn points(&self) -> u32 {
        (self.total() / 10.0) as u32
    }
}

// ---------------------------------------------------------------------------
// 9. Implementing a trait for arrays (const generics)
// ---------------------------------------------------------------------------

fn generate_pointworthy() -> impl Pointworthy {
    // An array of `ConsultingWork` is `Billable` (see the impl below), and
    // therefore also `Pointworthy` thanks to the blanket impl above.
    [ConsultingWork::default(), ConsultingWork::default()]
    //[1.0, 2.0, 3.0]
    //ConsultingWork::default()
    //42.0
    //FlatFee(300.0)
}

// `const C: usize` is a *const generic*: it makes this impl cover arrays of
// ANY length `[T; C]` whose element type `T` is itself `Billable`. The total
// of an array is the sum of its elements' totals.
impl<T: Billable, const C: usize> Billable for [T; C] {
    fn description(&self) -> &str {
        "Fees"
    }

    fn total(&self) -> f32 {
        self.iter().map(|item| item.total()).sum()
    }
}

// ---------------------------------------------------------------------------
// 10. Dynamic dispatch with trait objects (`Box<dyn Trait>`)
// ---------------------------------------------------------------------------

// When the concrete type is only known at runtime, use a *trait object*.
// `Box<dyn Billable>` is a heap-allocated value plus a vtable; the method call
// is resolved at runtime. This is exactly what `impl Billable` return types
// could NOT do — here each branch returns a *different* concrete type, unified
// behind `dyn Billable`.
fn create_billable_dyn(hours: f32, rate: f32, what: Option<&str>) -> Box<dyn Billable> {
    match what {
        Some(what) => Box::new(ConsultingWork {
            what: what.to_string(),
            hours,
            rate,
        }),
        None => Box::new(hours * rate), // an `f32`, boxed as `dyn Billable`
    }
}

// A `Vec` of trait objects can hold a mix of concrete types — impossible with a
// plain `Vec<T>`, which must be homogeneous.
fn create_billables_dyn() -> Vec<Box<dyn Billable>> {
    vec![
        create_billable_dyn(8.0, 100.0, Some("Rust Consulting")),
        create_billable_dyn(5.0, 150.0, None),
        Box::new(FlatFee(300.0)),
    ]
}

fn print_dynamic_dispatch(item: &Box<dyn Billable>) {
    println!("{}", item.formatted_total());
}

// ---------------------------------------------------------------------------
// 11. `Display`, supertraits, and trait bounds
// ---------------------------------------------------------------------------

// Implementing the std `Display` trait defines how `{}` formats our type.
impl Display for ConsultingWork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ConsultingWork: {} hours at ${:.2}/hour for {}",
            self.hours, self.rate, self.what
        )
    }
}

// A *supertrait*: `PrintableBillable` requires that a type is BOTH `Billable`
// AND `Display`. It adds no new methods itself — it just bundles two bounds
// under one name.
trait PrintableBillable: Billable + Display {}

// Blanket impl again: anything that is both `Billable` and `Display` is
// automatically `PrintableBillable`.
impl<T> PrintableBillable for T
where T: Billable + Display {}

// Trait bounds expressed with a `where` clause. To call this, a type must be
// `PrintableBillable` (hence `Billable` + `Display`) AND `Debug`.
// (`ConsultingWork` doesn't derive `Debug`, so this fn is here to show the
// syntax rather than to be called from `main`.)
fn print_billable_summary<T>(item: &T)
where
    T: PrintableBillable + std::fmt::Debug,
{
    println!("Summary: {}", item);   // uses Display
    println!("Summary: {:?}", item); // uses Debug
    println!("Total: ${:.2}", item.total());
}

// ---------------------------------------------------------------------------
// 12. Associated types vs. generic type parameters
// ---------------------------------------------------------------------------

// An *associated type* (`type Item`) is chosen ONCE per implementing type.
// A given `Repository` implementor stores exactly one kind of item.
trait Repository {
    type Item;

    // ...
    fn get(&self, id: u32) -> Option<Self::Item>;
}

struct ConsultingWorkRepository {
    // ...
}

impl Repository for ConsultingWorkRepository {
    // Pin the associated type for THIS implementation.
    type Item = ConsultingWork;

    fn get(&self, id: u32) -> Option<Self::Item> {
        todo!("Implementation missing")
    }
}

// COMPARE — the generic-parameter version (commented out). With `Repository<T>`
// a single type could implement `Repository<ConsultingWork>`,
// `Repository<FlatFee>`, ... many times. Use associated types when there's
// exactly one natural `Item` per implementor; use a generic parameter when a
// type should support several.
/*
trait Repository<T> {
    // ...
    fn get(&self, id: u32) -> Option<T>;
}

struct ConsultingWorkRepository {
    // ...
}

impl Repository<ConsultingWork> for ConsultingWorkRepository {
    fn get(&self, id: u32) -> Option<ConsultingWork> {
        todo!("Implementation missing")
    }
}
*/

// ---------------------------------------------------------------------------
// Putting it all together
// ---------------------------------------------------------------------------

fn main() {
    // `Default::default()` from section 1; `print_total` uses static dispatch.
    let work: ConsultingWork = Default::default();
    print_total(&work);

    // A primitive `f32` is `Billable` too (section 4).
    let fee: f32 = 200.0;
    print_total(&fee);

    // `FlatFee` overrides `formatted_total`, so its output looks different.
    let flat_fee = FlatFee(300.0);
    print_total(&flat_fee);

    // `impl Billable` return type (section 7).
    let billable = generate_billable();
    print_total(&billable);

    let abs_value = generate_abs();
    println!("Absolute value is {}", abs_value.abs());

    // --- Method-name resolution demo (section 4) ---
    // Which `abs` runs? Inherent methods take priority over trait methods,
    // so plain `.abs()` and `f32::abs(..)` call std's built-in `abs`...
    let negative_fee: f32 = -150.0;
    println!(
        "Absolute value of {} is {}",
        negative_fee,
        negative_fee.abs()
    );
    println!(
        "Absolute value of {} is {}",
        negative_fee,
        f32::abs(negative_fee)
    );
    // ...whereas fully-qualified syntax forces OUR `AbsTrait::abs` to run.
    let negative_fee_trait: f32 = -150.0;
    println!(
        "Absolute value of {} is {}",
        negative_fee_trait,
        AbsTrait::abs(negative_fee_trait)
    );
    println!(
        "Absolute value of {} is {}",
        negative_fee_trait,
        // The most explicit form: "call `abs` from `AbsTrait` as implemented for `f32`".
        <f32 as AbsTrait>::abs(negative_fee_trait)
    );

    // --- Dynamic dispatch demo (section 10) ---
    let rust_consulting = create_billable_dyn(8.0, 100.0, Some("Rust Consulting"));
    print_dynamic_dispatch(&rust_consulting);

    // A heterogeneous list of billables behind `dyn Billable`.
    let billables = create_billables_dyn();
    for billable in billables {
        print_dynamic_dispatch(&billable);
    }
}
