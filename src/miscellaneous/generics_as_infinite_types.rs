// Generics as infinite types
// ===========================

/*
    When a struct has a generic parameter, it can represent many concrete types.

    Each concrete instantiation can have its own implementation,
        or you can write a universal generic implementation.

    Generic implementations are universal ("template-style") — they work for all possible types T.

    Monomorphization: Rust creates concrete versions of generic types/functions at compile time.

    This is why generics are sometimes called “infinite types” — conceptually there is one generic definition,
        but it can generate an infinite number of concrete types.

    Specializations are useful when certain types need unique behavior (like formatting for i32 vs f32).

    You can also mix generics with trait bounds if you want certain methods to only exist for types
        implementing a trait.
*/

fn generics_as_infinite_types() {
    // Each specialization (Person<i32>, Person<f32>) is a completely different type at compile time.
    // The compiler generates separate monomorphized code for each specialization.
    struct Person<T> {
        name: String,
        age: T,
    }

    impl Person<i32> {
        // specialization for i32

        pub fn new(name: String, age: i32) -> Self {
            Person { name, age }
        }

        pub fn greet(&self) {
            println!(
                "Hello, my name is {} and I am {} years old.",
                self.name, self.age
            );
        }
    }

    impl Person<f32> {
        // specialization for f32

        pub fn greet(&self) {
            println!("Hello, I am {} and I am {} years old.", self.name, self.age);
        }
    }

    // You can mix specialized implementations and generic implementations,
    // but Rust does not allow overlapping implementations for the same type.
    struct Animal<T> {
        name: String,
        age: T,
    }

    impl<U> Animal<U> {
        // Generic implementation: works for all T

        pub fn new(name: String, age: U) -> Self {
            Animal { name, age }
        }

        pub fn speak(&self) {
            println!("I can sound like a {}", self.name);
        }
    }
}
