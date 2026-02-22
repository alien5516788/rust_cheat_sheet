/*
    ============================================
    RUST POINTER TYPES — THIN vs FAT POINTERS
    ============================================

    Core Idea:
    A pointer is a value that stores a memory address.

    But not all pointers are the same size.

    There are:

    1) Thin pointers  -> just an address
    2) Fat pointers   -> address + metadata

    On a 64-bit system:
        Thin pointer = 8 bytes
        Fat pointer  = 16 bytes (usually)
*/

use std::mem;

trait Drawable {
    fn draw(&self);
}

struct Circle;

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing Circle");
    }
}

fn main() {
    /*
        =====================
        1) THIN POINTER
        =====================

        &T where T has known size at compile time.
        Contains ONLY the address.
    */

    let x = 10;
    let r: &i32 = &x;

    println!("Size of &i32: {} bytes", mem::size_of::<&i32>());
    // On 64-bit: 8 bytes

    /*
        =====================
        2) FAT POINTER — SLICE
        =====================

        A slice like &[T] does NOT know its length at compile time.

        So Rust stores:
            (data pointer, length)

        That makes it a fat pointer.
    */

    let arr = [1, 2, 3, 4];
    let slice: &[i32] = &arr;

    println!("Size of &[i32]: {} bytes", mem::size_of::<&[i32]>());
    // On 64-bit: 16 bytes
    // 8 bytes -> address
    // 8 bytes -> length

    /*
        =====================
        3) FAT POINTER — str
        =====================

        &str is actually:
            (data pointer, length)

        Same concept as slice.
    */

    let s: &str = "hello";

    println!("Size of &str: {} bytes", mem::size_of::<&str>());
    // On 64-bit: 16 bytes

    /*
        =====================
        4) FAT POINTER — TRAIT OBJECT
        =====================

        &dyn Trait stores:
            (data pointer, vtable pointer)

        The vtable contains:
            - function pointers to trait methods
            - drop function
            - size of concrete type
            - alignment info

        This enables dynamic dispatch.
    */

    let c = Circle;
    let obj: &dyn Drawable = &c;

    println!(
        "Size of &dyn Drawable: {} bytes",
        mem::size_of::<&dyn Drawable>()
    );
    // On 64-bit: 16 bytes
    // 8 bytes -> address of Circle
    // 8 bytes -> pointer to vtable

    obj.draw(); // dynamic dispatch happens here

    /*
        =====================
        5) RAW POINTERS
        =====================

        *const T
        *mut T

        Unsafe.
        No borrow checking.
        Can be thin or fat depending on T.
    */

    let raw_ptr: *const i32 = &x;

    println!("Size of *const i32: {} bytes", mem::size_of::<*const i32>());
    // Usually 8 bytes

    /*
        =====================
        6) BOX<T>
        =====================

        Box<T> is a smart pointer that owns heap memory.

        Thin pointer if T is sized.
        Fat pointer if T is DST (like dyn Trait).
    */

    let boxed = Box::new(42);
    println!("Size of Box<i32>: {} bytes", mem::size_of::<Box<i32>>());
    // Usually 8 bytes (just heap address)

    let boxed_trait: Box<dyn Drawable> = Box::new(Circle);
    println!(
        "Size of Box<dyn Drawable>: {} bytes",
        mem::size_of::<Box<dyn Drawable>>()
    );
    // Usually 16 bytes (fat pointer)

    /*
        =====================
        7) FUNCTION POINTERS
        =====================

        A function pointer points to executable code.
        Thin pointer.
    */

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    let f: fn(i32, i32) -> i32 = add;

    println!(
        "Size of fn pointer: {} bytes",
        mem::size_of::<fn(i32, i32) -> i32>()
    );
    // Usually 8 bytes

    println!("f(2, 3) = {}", f(2, 3));

    /*
        ============================================
        SUMMARY
        ============================================

        Thin pointer:
            - &T
            - *const T
            - Box<T> (if T is sized)
            - fn pointer

        Fat pointer:
            - &[T]      -> (ptr, length)
            - &str      -> (ptr, length)
            - &dyn Trait -> (ptr, vtable)
            - Box<dyn Trait>

        Why fat pointers exist:
            Rust must carry runtime metadata
            for dynamically sized types (DSTs).
    */
}
