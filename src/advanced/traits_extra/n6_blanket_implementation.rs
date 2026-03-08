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
