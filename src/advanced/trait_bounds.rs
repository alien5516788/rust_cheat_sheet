// Trait bounds
// ============

/*
    Trait bounds restrict generic type parameters so that
        only types implementing specific traits are allowed.

    They are used for:
        - Access trait methods on generic types
        - Enforce compile-time behavior requirements
        - Enable polymorphism with static dispatch

    All trait bounds are checked at compile time.
*/

fn trait_bound_syntax() {
    // Trait implementation
    trait Describable {
        fn describe(&self) -> String;
    }

    struct User {
        name: String,
    }

    impl Describable for User {
        fn describe(&self) -> String {
            format!("User: {}", self.name)
        }
    }

    // Now only types that implement the Describable trait can be passed to this function

    // Trait bound using `impl Trait` syntax (explicitly uses static dispatch without generics)
    fn print_description_1(item: impl Describable) {
        println!("{}", item.describe());
    }

    // Trait bound using generics
    fn print_description_2<T: Describable>(item: T) {
        println!("{}", item.describe());
    }

    // Trait bound using `where` clause (For complex trait bounds)
    fn print_description_3<T>(item: T)
    where
        T: Describable,
    {
        println!("{}", item.describe());
    }

    // Usage
    let user = User {
        name: String::from("Alien"),
    };

    print_description_1(user);
}

fn multiple_trait_bounds() {
    // Multiple trait bounds can be added using `+` operator.
    // ex: PartialEq and Clone traits from std lib

    fn combine_1(x: impl PartialEq + Clone) -> bool {
        let y = x.clone();
        x == y
    }

    fn combine_2<T: PartialEq + Clone>(x: T) -> bool {
        let y = x.clone();
        x == y
    }

    fn combine_3<T>(x: T) -> bool
    where
        T: PartialEq + Clone,
    {
        let y = x.clone();
        x == y
    }
}

fn trait_bounds() {
    fn combine<T: PartialEq + Clone>(x: T) -> bool {
        let y = x.clone();
        x == y
    }

    struct User<T: PartialEq + Clone> {
        name: String,
        age: T,
    }

    enum UserStatus<T: PartialEq + Clone> {
        Active(T),
        Inactive,
    }

    trait Move<T: PartialEq + Clone> {
        fn move_to(&self, x: T, y: T);
    }
}
