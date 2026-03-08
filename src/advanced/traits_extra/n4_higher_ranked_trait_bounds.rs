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
