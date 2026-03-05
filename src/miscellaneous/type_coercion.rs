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

fn main() {
    // =========================================================================
    // 1. COERCION (Implicit Conversion)
    // The compiler does this automatically at "coercion sites."
    // These are always safe and usually involve pointers/references.
    // =========================================================================
    
    let my_string: String = String::from("Hello Rust");

    // DEREF COERCION: 
    // The function expects &str, but we give it &String.
    // Rust implicitly calls .deref() because String implements Deref<Target = str>.
    print_message(&my_string); 

    let mut mutable_value: i32 = 10;
    
    // MUTABILITY COERCION:
    // We create a mutable reference, but coerce it into an immutable one.
    let coerced_ref: &i32 = &mut mutable_value; 
    println!("Coerced immutable ref: {}", coerced_ref);


    // =========================================================================
    // 2. CASTING (Explicit Conversion via `as`)
    // You must use the 'as' keyword. This can be "lossy" (data can change).
    // =========================================================================
    
    let decimal: f64 = 65.99;

    // NUMERIC CAST:
    // Rust will NEVER implicitly turn a float into an int.
    // We must explicitly cast. Note: This truncates the decimal!
    let integer = decimal as u8; 
    
    // CHAR CAST:
    // Integers can be cast to chars (if they represent a valid Unicode point).
    let character = integer as char; 

    println!("Casting: {} -> {} -> {}", decimal, integer, character);


    // =========================================================================
    // 3. CONVERSION (Explicit Conversion via Traits)
    // The idiomatic "Rust way" for complex types using .into() or From::from().
    // =========================================================================
    
    let original_str: &str = "Type Conversion";

    // .into() is explicit but more "semantic" than a raw 'as' cast.
    // It consumes the original or creates a new owned version.
    let owned_string: String = original_str.into();

    println!("Converted to owned String: {}", owned_string);
}

/// A function expecting a string slice (&str)
fn print_message(msg: &str) {
    println!("Message: {}", msg);
}