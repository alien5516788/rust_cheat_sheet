// Trait bounds
// ============

/*
    Trait bounds restrict generic type parameters so that
    only types implementing specific traits are allowed.

    They are used to
        Access trait methods on generic types
        Enforce compile-time behavior requirements
        Enable polymorphism with static dispatch

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

    // Trait bound using inline syntax
    fn print_description_1<T: Describable>(item: T) {
        println!("{}", item.describe());
    }

    // Trait bound using `where` clause (recommended)
    fn print_description_2<T>(item: T)
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

    // Trait bound using inline syntax
    fn combine_1<T: PartialEq + Clone>(x: T) -> bool {
        let y = x.clone();
        x == y
    }

    // Trait bound using `where` clause (recommended)
    fn combine_2<T>(x: T) -> bool
    where
        T: PartialEq + Clone,
    {
        let y = x.clone();
        x == y
    }

    // Usage
    let _v = combine_1(String::from("hello"));
}

fn trait_bounds() {
    fn combine<T>(x: T) -> bool
    where
        T: PartialEq + Clone,
    {
        let y = x.clone();
        x == y
    }

    struct User<T>
    where
        T: PartialEq + Clone,
    {
        name: String,
        age: T,
    }

    enum UserStatus<T>
    where
        T: PartialEq + Clone,
    {
        Active(T),
        Inactive,
    }

    trait Move<T>
    where
        T: PartialEq + Clone,
    {
        fn move_to(&self, x: T, y: T);
    }
}
