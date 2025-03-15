struct Person {
    name : String,
    age : i32
}

struct Animal {
    name : String,
    weight : i32
}

/* Is like an abstract implementation shared with mutiple types
    Cannot have multiple trait blocks with same name */
trait Walk {

    // Contains method signature
    fn walk();
    fn run(&self);

    // Can have a default implementation
    fn fly(self) {
        // self parameter is used to denote that this is a member function
        println!("I'm flying ...");
    }

}

// trait implementation
impl Walk for Person {
    // trait should be fully implemented for the struct
    // cannot have multiple trait implementation blocks for same struct
    fn walk() {
        println!("I am walking");
    }
    fn run(&self) {
        println!("{} is running", self.name);
    }

}

impl Walk for Animal {

    fn walk() {
        println!("Animal is walking");
    }
    fn run(&self) {
        println!("{} is running", self.name);
    }

}

// initialization
let mut person = Person {
    name : String::from("Person Name"),
    age : 67
};

let animal = Animal {
    name : String::from("Animal Name"),
    weight : 80
};

// accessing functions and methods
Person::walk();
Person::run(&person);
person.run();

Animal::walk();
Animal::run(&animal);
animal.run();
