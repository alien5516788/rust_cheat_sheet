// Functions
// =========

/*
   Functions defines behaviours

   Rust supports two types of functions
       Functions
       Closures
*/

fn functions() {
    // Functions can be nested
    // Cannot be overloaded

    // Functions evaluate to unit type by default
    fn function1() {}
    fn function2() -> () {}
    fn function3() {
        2;
    }
    fn function4() -> () {
        2;
    }
    fn function5() -> () {
        2;
        () // note no semicolon
    }
    fn function6() -> () {
        return (); // explicit evalaution with return
    }

    // Function with custom return type
    fn function7() -> i32 {
        2 // note no semicolon
    }

    fn function8() -> i32 {
        return 2;
    }

    // Function with parameters
    fn function9(a: i32, b: i32) -> i32 {
        return a + b;
    }

    fn function10(a: i32, b: i32, name: String) -> i32 {
        println!("Hello {}", name);
        return a + b;
    }

    // Function call
    function1();
    _ = function9(2, 4);
}

fn function_pointers() {
    // Functions can be used as objects via pointers
    // But actual function code is never duplicated
    fn speak(name: String) -> () {
        println!("Hello {}", name);
    }

    // Here function doesn't get owned from `speak`
    let speak_ptr: fn(String) = speak; // `fn(String)` is function signature/type

    // Call
    speak_ptr(String::from("Name"));
}
