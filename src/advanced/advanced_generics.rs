// ==========================================================
// AUTO TRAITS
// ==========================================================

// Auto traits are automatically implemented by Rust if all the fields in
// a type satisfy the auto trait. Example: Send, Sync
//
// Primary purpose:
// - Indicate thread-safety guarantees or other automatic properties.
// Use cases:
// - Multi-threading: Send for moving types across threads, Sync for shared access.
fn requires_send<T: Send>(_: T) {}

fn auto_trait_example() {
    let x = 5;
    requires_send(x); // i32 is Send, so this works
}

// ==========================================================
// MARKER TRAITS
// ==========================================================

// Marker traits are traits without methods. They mark types as having
// some property.
//
// Primary purpose:
// - Encode metadata about a type without adding behavior.
// Use cases:
// - std::marker::Copy, std::marker::Sized
trait Marker {}

struct MyType;

impl Marker for MyType {} // Marks MyType with this trait

// ==========================================================
// PHANTOM DATA
// ==========================================================

// PhantomData is a zero-sized type that allows you to indicate
// that a generic type parameter is logically used, without actually storing it.
//
// Primary purpose:
// - Track ownership, lifetimes, or generic type info without runtime cost.
// Use cases:
// - Safe wrappers around raw pointers
// - Lifetime tracking for generics
use std::marker::PhantomData;

struct Wrapper<T> {
    value: i32,
    _marker: PhantomData<T>, // Phantom type
}

impl<T> Wrapper<T> {
    fn new(value: i32) -> Self {
        Self {
            value,
            _marker: PhantomData,
        }
    }
}

// ==========================================================
// ZERO-SIZED TYPES (ZST)
// ==========================================================

// Zero-sized types occupy no memory. They can be used to encode
// type information or state without runtime cost.
//
// Primary purpose:
// - Encode type info or represent a "unit of behavior" without storage.
// Use cases:
// - Marker types, compile-time state machines
struct Empty;

fn zst_example() {
    let _e = Empty; // Zero runtime footprint
}

// ==========================================================
// BLANKET IMPLEMENTATION
// ==========================================================

// Blanket implementations apply a trait to any type that satisfies
// a set of bounds.
//
// Primary purpose:
// - Reduce boilerplate
// - Automatically implement traits for many types
// Use cases:
// - std::fmt::Debug, Copy, Clone, etc.
trait MyTrait {
    fn do_it(&self);
}

impl<T: std::fmt::Debug> MyTrait for T {
    fn do_it(&self) {
        println!("{:?}", self); // Implemented for all Debug types
    }
}

// ==========================================================
// NEGATIVE TRAIT IMPL (Nightly only)
// ==========================================================

// Negative trait implementations allow explicitly marking a type
// as NOT implementing an auto trait.
//
// Primary purpose:
// - Provide fine-grained control over auto traits
// Use cases:
// - Preventing Send/Sync on specific types
// Note: Nightly feature only
// #![feature(negative_impls)]
// auto trait MyAutoTrait {}
// impl !MyAutoTrait for i32 {}

// ==========================================================
// SPECIALIZATION (Nightly only)
// ==========================================================

// Specialization allows more specific implementations to override
// more general ones.
//
// Primary purpose:
// - Provide default behavior while allowing optimized overrides
// Use cases:
// - Library authors optimizing trait behavior
// Note: Nightly feature only
// #![feature(specialization)]
// trait Example {
//     fn value(&self) -> i32;
// }
// default impl<T> Example for T {
//     fn value(&self) -> i32 {
//         0
//     }
// }
// impl Example for i32 {
//     fn value(&self) -> i32 {
//         *self
//     }
// }

// ==========================================================
// CONST GENERIC TRAIT IMPL
// ==========================================================

// Const generics allow using constant values as type parameters.
//
// Primary purpose:
// - Generalize over array sizes, numeric constants, etc.
// Use cases:
// - Fixed-size arrays, buffers
trait ArrayTrait {
    fn size() -> usize;
}

impl<const N: usize> ArrayTrait for [i32; N] {
    fn size() -> usize {
        N
    }
}

// ==========================================================
// HIGHER-RANKED TRAIT BOUNDS (HRTB)
// ==========================================================

// HRTBs allow specifying that a function must accept closures
// that work for **any** lifetime.
//
// Primary purpose:
// - Generic over lifetimes in a function/closure argument
// Use cases:
// - Callbacks, generic iterator/stream combinators
fn accepts_for_all_lifetimes<F>(f: F)
where
    F: for<'a> Fn(&'a str),
{
    f("hello");
}

