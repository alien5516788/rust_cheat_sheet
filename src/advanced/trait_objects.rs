// Trait Objects
// =============

/*
    Trait objects allow for dynamic dispatch in Rust.

    They let you work with values whose concrete type is unknown at compile time,
        as long as they implement a specific trait.

    Key points:
        - Type is erased at compile time
        - Only the trait (behavior) is known
        - Uses dynamic dispatch via a vtable at runtime
        - Requires a pointer (`&`, `Box`, `Rc`, etc.) because size is unknown
        - Trait must be object-safe
*/

fn trait_objects() {
    // Trait implementation
    struct User {
        name: String,
    }

    struct Product {
        id: u32,
    }

    trait Describable {
        fn describe(&self) -> String;
    }

    impl Describable for User {
        fn describe(&self) -> String {
            format!("User: {}", self.name)
        }
    }

    impl Describable for Product {
        fn describe(&self) -> String {
            format!("Product ID: {}", self.id)
        }
    }

    // Trait object using `dyn Trait` syntax
    /*
        - `item` can be any type implementing Describable
        - Dispatch is dynamic (resolved at runtime via v-table)
     */
    fn print_description(item: &dyn Describable) {
        println!("{}", item.describe());
    }

    // Usage
    let user = User {
        name: String::from("Alien"),
    };
    let product = Product { id: 42 };

    // Both work with the same function
    print_description(&user);
    print_description(&product);
}

fn multiple_trait_objects() {
    // Multiple trait objects can be added using `+` operator.
    // ex: Send and Sync traits from std lib
    fn print_send(item: &(dyn Send + Sync)) {
        println!("Just look at parameters");
    }

    // However, multiple non-auto traits are NOT allowed directly ========= TODO
    trait Describable {
        fn describe(&self) -> String;
    }
    // You need a supertrait:
    trait Displayable: Describable {
        fn display(&self);
    }

    fn print_supertrait(item: &dyn Displayable) {
        item.display();
    } // ==============================
}
