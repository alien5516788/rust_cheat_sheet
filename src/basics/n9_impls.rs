/*
    - Used to define behaviours and shared behaviours (traits) for structs
    - Methods are just functions inside an impl block
    - There can be multiple impl blocks for the same type
    - Impl blocks do not store data, only behaviours
*/

fn implementing_behaviours() {
    /*
        - A struct can have multiple impl blocks
    */

    // Struct definition
    struct Person {
        name: String,
        age: i32,
    }

    // Example 1
    impl Person {
        fn speak(&self) {
            /*
                - Method
                - must have self / &self / &mut self as first parameter
            */
            println!("I am a person");
        }

        fn new(name: String, age: i32) -> Self {
            /*
                - Associated function (no self)
                - Fact: `new` is often used as a constructor
            */
            Person { name, age }
        }

        fn eat() {
            /*
                - Associated function
                - doesn't have a reference to self
            */
            println!("Yum Yum");
        }
    }

    // Example 2
    impl Person {
        fn update_name(&mut self) {
            /*
               - Method
            */
            self.name = String::from("Other Name");
        }

        /*
            Cannot implement same function or method twice
                (even across multiple impl blocks)
        */
    }

    // Accessing functions and methods
    /*
        - Methods can be accessed by dot syntax,
            as well as colon syntax by explicitly passing self
    */
    let mut person = Person::new(String::from("John"), 30);

    person.update_name();
    Person::update_name(&mut person);

    person.speak();
    Person::speak(&person);

    /*
        - Associated functions can only be accessed
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
