// Traits
// ======

/*
   - Traits defines behaviours
   - Similar to a struct but for methods
   - Unlike structs standalone objects cannot be created with traits,
       instead they should be implemented on structs
   - Traits are the fundamental way of doing polymorphism in rust
*/

fn traits() {
    // Trait definition
    trait Move {
        fn walk(); // contains method signature

        fn run(&self);

        fn fly(&self) {
            println!("I'm flying ...");
        } // can have a default implementation
    }

    // Trait implementation
    struct Person {
        name: String,
        age: i32,
    }

    struct Animal {
        name: String,
        weight: i32,
    }

    /*
        - Trait should be fully implemented for the struct except default implementation
        - Default implementation may or may not be implemented again
        - Cannot have multiple trait implementation of same trait for the same struct
    */
    impl Move for Person {
        fn walk() {
            println!("I am walking");
        }

        fn run(&self) {
            println!("{} is running", self.name);
        }

        fn fly(&self) {
            println!("{} is flying. But with an airplane", self.name);
        }
    }

    impl Move for Animal {
        fn walk() {
            println!("Animal is walking");
        }
        fn run(&self) {
            println!("{} is running", self.name);
        }
    }

    // Accessing functions and methods
    let mut _person = Person {
        name: String::from("Person Name"),
        age: 67,
    };

    let _animal = Animal {
        name: String::from("Animal Name"),
        weight: 80,
    };

    Person::walk();
    Person::run(&_person);
    _person.run();
    _person.fly();

    Animal::walk();
    Animal::run(&_animal);
    _animal.run();
}

// Marker Traits
// =============

/*
    - Marker traits are traits without methods. They mark types as having
       some property.
    - Nothing is special about marker traits them selves, they are traits without methods.
        - Just like structs, enums without fields (not unit structs)
        - Only usage is special.
    - Primary purposeis to encode metadata about a type without adding behavior.
    - Use cases:
       - To make trait bounds without having methods.
*/

fn marker_traits() {
    trait Marker {}

    struct MyType;

    // Marks 'MyType' with this trait
    impl Marker for MyType {}

    // Usage example
    fn do_nothing(arg: impl Marker) {
        println!("I did nothing with the argument.");
    }
}
