struct Person {
    name: String,
    age: i32,
}

// Can have mutiple implementation blocks
impl Person {
    /* Method
    have a reference to self
    must be the first parameter */
    fn update_name(&mut self) {
        self.name = String::from("Other Name");
    }

    /* Cannot implement same function or method twice,
    in same or another implementation block */
}

impl Person {
    // Method
    fn speak(&self) {
        println!("I m a person");
    }

    /* Associated function
    doesn't have a reference to self */
    fn eat() {
        println!("Yum Yum");
    }
}

fn impls() {
    // Initialization
    let mut person = Person {
        name: String::from("Person Name"),
        age: 67,
    };

    // Accessing functions and methods
    /* Methods can be accessed by dot syntax,
    as well as colon syntax with explicitly giving a reference to self */
    person.update_name();
    Person::update_name(&mut person);

    person.speak();
    Person::speak(&person);

    /* Associated functions can only be accessed by colon syntax */
    Person::eat();
}

// ===============================
// GENERIC IMPL BLOCKS
// ===============================

fn generic_impl_blocks() {
    struct Pair<T> {
        a: T,
        b: T,
    }

    // Impl for all T
    impl<T> Pair<T> {
        fn new(a: T, b: T) -> Self {
            Self { a, b }
        }
    }

    // Specialized impl for f64 only
    impl Pair<f64> {
        fn sum(&self) -> f64 {
            self.a + self.b
        }
    }

    let p = Pair::new(3.0, 4.0);
    let _s = p.sum();
}
