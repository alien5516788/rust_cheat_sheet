// Associated Types vs Generic Traits
// ==================================

/*
    Associated types are often cleaner when the type is intrinsic to the trait,
        while generics are better when you want more flexibility or multiple type parameters.

    Associated types prevents multiple implementations for the same struct (type).

    Associated types often reduce the need for turbofish syntax ( ::<Type> ).
*/

fn generic_traits() {
    /*
        Generic trait
            - The type is a generic parameter
            - Multiple implementations with diffrent concrete types are possible
    */
    trait Task<T> {
        fn process(&self, value: T);
    }

    // Example
    struct Machine;

    impl Task<String> for Machine {
        fn process(&self, value: String) {
            println!("Logging: {}", value);
        }
    }

    impl Task<i32> for Machine {
        fn process(&self, value: i32) {
            println!("Logging: {}", value);
        }
    }
}

fn associated_types() {
    /*
        Associated type
            - The type is tied to the trait impl (becomes a memeber of the struct)
            - Can have as many types as we want
            - Unlike generic traits, trait with associated types cannot be implemented again
                with different concrete types for associated types
    */
    trait System {
        type Item;
        type Source;
        fn process(&self, value: Self::Item);
    }

    // Example
    struct Logger;

    impl System for Logger {
        type Item = String; // concrete type chosen here
        type Source = i32;

        fn process(&self, value: Self::Item) {
            println!("Logging: {}", value);
        }
    }
}
