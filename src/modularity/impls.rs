struct Person {
    name : String,
    age : i32
}

// can have mutiple implementation blocks
impl Person {

    /* method
        have a reference to self
        must be first parameter */
    fn update_name(&mut self) { 
        self.name = String::from("Other Name");
    }

    /* cannot implement same function or method twice,
        in same or another implementation block */

}

impl Person {
    
    // method
    fn speak(&self) { 
        println!("I m a person");
    }
    
    /* associated function
        doesn't have a reference to self */
    fn eat() {  
        println!("Yum Yum");
    }

}

// initialization
let mut person = Person {
    name : String::from("Person Name"),
    age : 67
};

// accessing functions and methods
/* methods can be accessed by dot syntax,
   as well as colon syntax with explicitly giving a reference to self */
person.update_name();
Person::update_name(&mut person);

person.speak();
Person::speak(&person);

/* associated functions can only be accessed by colon syntax */
Person::eat();
