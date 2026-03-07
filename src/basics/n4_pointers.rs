/*
    - In Rust, a "pointer" is any type that refers to data elsewhere
    - They differ by ownership, mutability, lifetime tracking, and safety
    - There are 3 main pointer types:
        - References (in-built)
        - Raw pointers (in-built, for c compatibility)
        - Smart pointers
*/

fn references() {
    /*
        - Immutable references:
            - Multiple immutable references allowed.
        - Mutable references:
            - Only one mutable reference allowed at a time.
            - Cannot have mutable and immutable references simultaneously.
        - References are the preferred pointers in rust.
        - References can be either a thin or a fat pointer depending on the type.
    */
    let num1 = 32;
    let mut num2 = 64;

    let ref1: &i32 = &num1; // immutable reference
    let _ref2: &i32 = &num1; // another immutable reference
    let mut_ref1: &mut i32 = &mut num2; // mutable reference

    // Dereferencing references
    let _ref_val: i32 = *ref1;
    let _mut_ref_val: i32 = *mut_ref1;
}

fn raw_pointers() {
    /*
        - Mostly exists for c compatibility and low level hacks
        - Rarely used
        - Unlimited number of mutable or immutable pointers allowed
        - Need an `unsafe` block to dereference
        - Note: & symbol returns memory address
    */
    let mut num1 = 32;
    let _num2 = 64;

    let ptr1: *const i32 = &num1; // immutable raw pointer
    let _ptr2: *const i32 = &num1; // another immutable raw pointer
    let mut_ptr1: *mut i32 = &mut num1; // mutable raw pointer
    let _mut_ptr2: *mut i32 = &mut num1; // another mutable raw pointer

    // Dereferencing
    unsafe {
        let _val: i32 = *ptr1;
        let _mut_val: i32 = *mut_ptr1;
    }
}

fn smart_pointers() {
    /*
        - Heap-allocated pointers with special ownership semantics
        - Box<T>, Rc<T>, Arc<T>, Cell<T>, RefCell<T>, Pin<P>
    */
}
