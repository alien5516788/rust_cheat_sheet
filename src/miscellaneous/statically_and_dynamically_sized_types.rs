use std::mem::{size_of, size_of_val}; // TODO

fn main() {
    println!("=== SIZED TYPES ===");

    // Fixed-size array: length is part of the type
    let fixed: [i32; 4] = [10, 20, 30, 40];

    // The compiler knows its size at compile time:
    // 4 elements * 4 bytes each = 16 bytes
    println!("Size of [i32; 4]: {}", size_of::<[i32; 4]>());

    // Reference to a sized type -> THIN pointer
    let fixed_ref: &[i32; 4] = &fixed;
    println!(
        "Size of &[i32; 4] (thin pointer): {}",
        size_of::<&[i32; 4]>()
    );

    println!("\n=== DYNAMICALLY SIZED TYPES (DST) ===");

    // Slice: [i32] is dynamically sized
    // Its length is NOT part of the type
    let slice: &[i32] = &fixed[..];

    // You CANNOT do:
    // size_of::<[i32]>()  // ❌ compile error (unsized type)

    // But you CAN ask at runtime:
    println!("Runtime size of slice contents: {}", size_of_val(slice));

    // Reference to a DST -> FAT pointer
    println!("Size of &[i32] (fat pointer): {}", size_of::<&[i32]>());

    println!("\n=== STR (also a DST) ===");

    let text: &str = "hello";

    // &str is also a fat pointer (ptr + length)
    println!("Size of &str (fat pointer): {}", size_of::<&str>());

    println!("\n=== TRAIT OBJECT (also DST) ===");

    let value = 42;
    let obj: &dyn std::fmt::Display = &value;

    // Trait object reference is also fat:
    // (data pointer + vtable pointer)
    println!(
        "Size of &dyn Display (fat pointer): {}",
        size_of::<&dyn std::fmt::Display>()
    );
}
