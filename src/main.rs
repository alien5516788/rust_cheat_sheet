#![allow(warnings, dead_code)]

use std::marker::PhantomData;

pub mod advanced;
pub mod basics;
pub mod flow_control;
pub mod miscellaneous;

fn main() {
    // Simple idea is compiler strips any field with `PhantomData` at compile time (after any verification is done)
    // 
    struct Meter (i32);
    struct Foot (i32);
    
    struct Person<U> {
        height: U,
    }

    let p = Person::<Meter> {
        height: Meter(170),
    };
}
