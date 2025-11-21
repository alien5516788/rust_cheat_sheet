// Is like an abstract implementation shared with mutiple types
// similar to a struct but with methods
// Cannot have multiple trait blocks with same name just like structs
trait Move {

    // Contains method signature
    fn walk();
    
    // `self` parameter is used to denote that this is a member function
    fn run(&self);

    // Can have a default implementation
    fn fly(&self) {
        println!("I'm flying ...");
    }

}

struct Person {
    name : String,
    age : i32
}

struct Animal {
    name : String,
    weight : i32
}

// Trait implementation
impl Move for Person {
    // Trait should be fully implemented for the struct except default iplementation
    // Default implementation may or may not be implemented again
    // Cannot have multiple trait implementation of same trait for same struct
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

fn traits() {
    
    // Initialization
    let mut person = Person {
        name : String::from("Person Name"),
        age : 67
    };
    
    let animal = Animal {
        name : String::from("Animal Name"),
        weight : 80
    };
    
    // Accessing functions and methods
    Person::walk();
    Person::run(&person);
    person.run();
    person.fly();
    
    Animal::walk();
    Animal::run(&animal);
    animal.run();

}
