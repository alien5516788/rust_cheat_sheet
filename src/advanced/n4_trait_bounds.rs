fn trait_bounds() {
    /*
        - Trait bounds restrict function parameters or generic type parameters so that
            only types implementing specific traits are allowed
        - They are used for:
            - Access trait methods on generic types
            - Enforce compile-time behavior requirements
            - Enable polymorphism with static dispatch
        - All trait bounds are checked at compile time
    */

    // Trait implementation
    struct User {
        name: String,
    }

    struct Product {
        id: u32,
    }

    trait Describable {
        fn describe(&self);
    }

    impl Describable for User {
        fn describe(&self) {
            println!("User: {}", self.name);
        }
    }

    impl Describable for Product {
        fn describe(&self) {
            println!("Product ID: {}", self.id);
        }
    }

    // Example 1
    // Trait bound using generics
    fn print_description_1<T: Describable>(item: T) {
        /*
           - Takes a generic type `T` that implements `Describable` trait
           - Allows passing any type that implements `Describable`
        */
        item.describe();
    }

    let user = User {
        name: String::from("Alien"),
    };

    let product = Product { id: 42 };

    print_description_1(user);
    print_description_1(product);

    // Example 2
    // Trait bound using `impl Trait` syntax
    fn print_description_2(item: impl Describable) {
        /*
            - Same as above
            - Doesn't expose unnecessary generics
            - Avoids turbofish notation for explicit type
        */
        item.describe();
    }

    let user = User {
        name: String::from("Alien"),
    };

    let product = Product { id: 42 };

    print_description_2(user);
    print_description_2(product);

    // Example 3
    // Trait bound using `where` clause
    fn print_description_3<T>(item: T)
    where
        T: Describable,
    {
        /*
            - Same as above
            - Allows adding complex trait bounds without cluttering the function signature
        */
        item.describe();
    }

    let user = User {
        name: String::from("Alien"),
    };

    let product = Product { id: 42 };

    print_description_3(user);
    print_description_3(product);
}

fn multiple_trait_bounds() {
    /*
        - Multiple trait bounds can be added using `+` operator
    */

    // Example
    // `PartialEq` and `Clone` traits from std lib
    fn combine<T: PartialEq + Clone>(x: T) -> bool {
        /*
            - Takes a generic type `T` that implements `PartialEq` and `Clone` traits
        */
        let y = x.clone();
        x == y
    }
}

fn optional_trait_bounds() {
    /*
        - Optional trait bounds can be added using `?` operator
        - Only works with Sized trait currently
            - Rust adds an implicit `Sized` bound to trait bounds
            - `?` allows to relax that restriction
    */

    // Example
    // `Clone` and `Sized` traits from std lib
    fn combine(x: impl Clone + ?Sized) {
        /*
            - Takes a generic type `T` that implements `Clone` trait
            - But Sized trait may or may not be implemented
        */
        let _y = x.clone();
        println!("I just cloned x");
    }
}

fn more_examples() {
    // Example 1
    fn combine<T: PartialEq + Clone>(x: T) -> bool {
        let y = x.clone();
        x == y
    }

    // Example 2
    struct Person<T: PartialEq + Clone> {
        name: String,
        age: T,
    }

    // Example 3
    enum UserStatus<T: PartialEq + Clone> {
        Active(T),
        Inactive,
    }

    // Example 4
    trait Move<T: PartialEq + Clone> {
        fn move_to(&self, x: T, y: T);
    }
}
