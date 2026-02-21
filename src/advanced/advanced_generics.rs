// ==========================================================
// GENERIC ASSOCIATED TYPES (GATs)
// ==========================================================

trait MyIterator {
    type Item<'a>
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

// Basic implementation example

struct Numbers;

impl MyIterator for Numbers {
    type Item<'a> = &'a i32 where Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        None
    }
}


// ==========================================================
// AUTO TRAITS
// ==========================================================

// Auto traits are automatically implemented by the compiler
// Example: Send, Sync

fn requires_send<T: Send>(_: T) {}

fn auto_trait_example() {
    let x = 5;
    requires_send(x);
}


// ==========================================================
// MARKER TRAITS
// ==========================================================

// Marker trait = trait with no methods

trait Marker {}

struct MyType;

impl Marker for MyType {}


// ==========================================================
// PHANTOM DATA
// ==========================================================

use std::marker::PhantomData;

struct Wrapper<T> {
    value: i32,
    _marker: PhantomData<T>,
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

struct Empty;

fn zst_example() {
    let _e = Empty;
}


// ==========================================================
// BLANKET IMPLEMENTATION
// ==========================================================

trait MyTrait {
    fn do_it(&self);
}

impl<T: std::fmt::Debug> MyTrait for T {
    fn do_it(&self) {
        println!("{:?}", self);
    }
}


// ==========================================================
// NEGATIVE TRAIT IMPL (Nightly only)
// ==========================================================

// #![feature(negative_impls)]

// auto trait MyAutoTrait {}
// impl !MyAutoTrait for i32 {}


// ==========================================================
// SPECIALIZATION (Nightly only)
// ==========================================================

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

fn accepts_for_all_lifetimes<F>(f: F)
where
    F: for<'a> Fn(&'a str),
{
    f("hello");
}


// ==========================================================
// ASSOCIATED TYPE VS GENERIC TRAIT PATTERN
// ==========================================================

// Generic trait version
trait TraitA<T> {
    fn process(&self, value: T);
}

// Associated type version
trait TraitB {
    type Item;
    fn process(&self, value: Self::Item);
}
