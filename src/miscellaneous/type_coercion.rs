use std::mem::size_of;

fn main() {
    println!("=== 1️⃣ Array to Slice Coercion ===");

    let arr: [i32; 5] = [10, 20, 30, 40, 50];

    // Fixed-size array reference (thin pointer)
    let arr_ref: &[i32; 5] = &arr;
    println!(
        "Size of &[i32; 5] (thin pointer): {}",
        size_of::<&[i32; 5]>()
    );

    // Coerce array reference to slice reference (fat pointer)
    let slice: &[i32] = &arr;
    println!("Size of &[i32] (fat pointer): {}", size_of::<&[i32]>());

    // Slices only borrow; length is stored in fat pointer
    println!("Slice contents: {:?}", slice);

    println!("\n=== 2️⃣ Sub-slice ===");

    // Slice from array range also coerces automatically
    let subslice: &[i32] = &arr[1..4]; // length = 3
    println!("Sub-slice contents: {:?}", subslice);
    println!("Sub-slice length: {}", subslice.len());

    println!("\n=== 3️⃣ Vec to Slice Coercion ===");

    let vec = vec![100, 200, 300];

    // Vec owns heap memory
    println!("Vec length: {}", vec.len());

    // &Vec<T> automatically coerces to &[T] slice
    let vec_slice: &[i32] = &vec;
    println!("Slice from Vec: {:?}", vec_slice);

    println!("\n=== 4️⃣ Trait Object Coercion ===");

    struct MyStruct;
    impl std::fmt::Display for MyStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MyStruct")
        }
    }

    let x = MyStruct;

    // Reference to concrete type
    let concrete_ref: &MyStruct = &x;

    // Coerce to trait object reference (fat pointer)
    let trait_ref: &dyn std::fmt::Display = &x;

    println!("Concrete reference: {:p}", concrete_ref);
    println!("Trait object reference: {:p}", trait_ref);

    println!("\n=== 5️⃣ Key Coercion Rules Illustrated ===");

    println!("- &T (T: Sized) -> thin pointer (one machine word)");
    println!("- &[T; N] -> &[T] (array reference to slice) -> fat pointer (ptr + length)");
    println!("- &Vec<T> -> &[T] (Vec reference coerces to slice) -> fat pointer");
    println!(
        "- &ConcreteType -> &dyn Trait (trait object reference) -> fat pointer (ptr + vtable)"
    );
}
