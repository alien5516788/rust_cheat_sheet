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

fn closures() {
    // Anonymous functions
    let fun = |name: String| {
        println!("Hello {}", name);
    };

    let fun_ptr: fn(String) = fun;

    fun(String::from("Name"));
    fun_ptr(String::from("Name"));

    // Captures the environment state
    let mut _num: i32 = 0; // Some variable from environment

    let mut print_num = |num| {
        // TODO: function signature
        println!("Hello {}", _num);
        _num += num;
    };

    print_num(1); // output: 1
    print_num(2); // output: 3
}
