// Impls
// =====

/*
    Used to define behaviours for structs,
        as well as implementations for traits.

    Methods are just functions inside an impl block.

    There can be multiple impl blocks for the same type.

    Impl blocks do not store data, only behaviour.
*/

fn implementing_behaviours() {
    struct Person {
        name: String,
        age: i32,
    }

    // Can have multiple implementation blocks
    impl Person {
        // Method
        // must have self / &self / &mut self as first parameter
        fn speak(&self) {
            println!("I am a person");
        }

        // Associated function (no self)
        // Often used as constructor
        fn new(name: String, age: i32) -> Self {
            Person { name, age }
        }

        // Associated function
        // doesn't have a reference to self
        fn eat() {
            println!("Yum Yum");
        }
    }

    impl Person {
        // Method
        fn update_name(&mut self) {
            self.name = String::from("Other Name");
        }

        /*
            Cannot implement same function or method twice
                (even across multiple impl blocks)
        */
    }

    // Accessing functions and methods
    /*
        Methods can be accessed by dot syntax,
            as well as colon syntax by explicitly passing self
    */
    let mut person = Person::new(String::from("John"), 30);

    person.update_name();
    Person::update_name(&mut person);

    person.speak();
    Person::speak(&person);

    /*
        Associated functions can only be accessed
        using :: syntax
    */
    Person::eat();
}

fn implementing_traits() {
    struct Person {
        name: String,
        age: i32,
    }

    trait Move {
        // Better to include &self unless it is purely static behaviour
        fn walk(&self);
    }

    impl Move for Person {
        fn walk(&self) {
            println!("Walking");
        }
    }
}
