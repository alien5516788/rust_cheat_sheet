// functions

/* fucntions evaluate to unit type by default */
fn fucntion() {}
fn function() -> () {}
fn fucntion() {
    2;
}
fn function() -> () {
    2;
}

/* function with custom return type */
fn function() -> i32 { 
    2
}

fn function() -> i32 { 
    return 2;
}

/* function with parameters */
fn function(a : i32, b : i32) -> i32 { 
    return a + b;
}

fn function(a : i32, b : i32, name : String) -> i32 {
    println!("Hello {}", name);
    return a + b;
}

/* functions cannot be overloaded */
/* this script won't run due to above examples having same name 
regardless of having different parameters */

/* nested functions */
fn functionA() {
    
    println!("functionA");

    fn functionB() {
        println!("functionB");
    }

    functionB(32);

}

// Function pointers
{    

    fn speak(name : String) -> () {
        println!("Hello {}", name);
    }

    // dereferencing
    speak as fn (String); // memory address

    let speak_ptr : fn (String) = speak; // memory address

    let speak_ptr : fn (String);
    speak_ptr = speak; // memory address

    let speak_ptr : fn (String);
    speak_ptr = speak as fn (String); // memory address

    // calling
    speak_ptr(String::from("Name"));

}


// closures

{

    let mut num : i32 = 0; // temp var
    
    // closure declaration
    // closure will capture the state of variable num
    let mut print_num = || { 
        println!("Hello {}", num);
        num += 1;
    };
    
    // calling
    print_num(); // output: 0
    print_num(); // output: 1
    
}
