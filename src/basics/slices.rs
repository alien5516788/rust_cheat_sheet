use std::mem::{size_of, size_of_val}; // TODO

fn main() {
    println!("=== 1️⃣ Slice From Array ===");

    // Fixed-size array (length is part of the type)
    let arr: [i32; 5] = [10, 20, 30, 40, 50];

    // A slice is a VIEW into existing contiguous memory.
    // It does NOT own data.
    let slice: &[i32] = &arr[1..4];

    println!("Array: {:?}", arr);
    println!("Slice (arr[1..4]): {:?}", slice);

    // Compile-time size of array type
    println!("Size of [i32; 5]: {}", size_of::<[i32; 5]>());

    // &array is a thin pointer (just address)
    println!("Size of &[i32; 5]: {}", size_of::<&[i32; 5]>());

    // &[i32] is a fat pointer (address + length)
    println!("Size of &[i32]: {}", size_of::<&[i32]>());

    // Runtime size of the slice contents
    println!("Runtime size of slice contents: {}", size_of_val(slice));

    println!("\n=== 2️⃣ Slice From Vec ===");

    let vec = vec![100, 200, 300, 400];

    // Vec owns heap memory.
    // A slice just borrows that memory.
    let vec_slice: &[i32] = &vec;

    println!("Vec: {:?}", vec);
    println!("Slice from Vec: {:?}", vec_slice);

    println!("Vec len(): {}", vec.len());
    println!("Slice len(): {}", vec_slice.len());

    println!("\n=== 3️⃣ Mutable Slice ===");

    let mut numbers = [1, 2, 3, 4];

    {
        // Mutable slice gives mutable access
        let slice_mut: &mut [i32] = &mut numbers[1..3];

        // Modify through the slice
        slice_mut[0] = 99;
        slice_mut[1] = 88;
    }

    println!("Modified array via slice: {:?}", numbers);

    println!("\n=== 4️⃣ What a Slice Really Is (Conceptually) ===");

    /*
        Conceptually, a slice reference is like:

        struct SliceRef<T> {
            ptr: *const T,
            len: usize,
        }

        It stores:
            - pointer to first element
            - number of elements

        It does NOT store capacity.
        It does NOT own memory.
    */

    println!("Size of &str (also a slice of bytes): {}", size_of::<&str>());
}
