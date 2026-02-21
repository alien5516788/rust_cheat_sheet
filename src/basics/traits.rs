// Traits
// ======

/*
   Traits defines behaviours

   Similar to a struct but for methods

   Unlike structs standalone objects cannot be created with traits,
       instead they should be implemented on structs

   Traits are the fundamental way of doing polymorphism in rust
*/

fn traits() {
    trait Move {
        // Contains method signature
        fn walk();

        fn run(&self);

        // Can have a default implementation
        fn fly(&self) {
            println!("I'm flying ...");
        }
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

    impl Move for Person {
        // Trait should be fully implemented for the struct except default iplementation
        // Default implementation may or may not be implemented again
        // Cannot have multiple trait implementation of same trait for the same struct

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
