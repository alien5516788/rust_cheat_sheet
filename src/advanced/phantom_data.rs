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