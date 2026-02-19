fn functions() {
    // Functions evaluate to unit type by default
    // Functions cannot be overloaded
    // Functions can be nested

    fn function1() {}
    fn function2() -> () {}
    fn function3() {
        2;
    }
    fn function4() -> () {
        2;
    }

    // Function with custom return type
    fn function5() -> i32 {
        2
    }

    fn function6() -> i32 {
        return 2;
    }

    // Function with parameters
    fn function7(a: i32, b: i32) -> i32 {
        return a + b;
    }

    fn function8(a: i32, b: i32, name: String) -> i32 {
        println!("Hello {}", name);
        return a + b;
    }

    // Function call
    function1();
    _ = function7(2, 4);
}

fn function_pointers() {
    fn speak(name: String) -> () {
        // Ignore
        println!("Hello {}", name);
    }

    speak as fn(String); // Memory address
    let speak_ptr: fn(String) = speak;

    // Call
    speak_ptr(String::from("Name"));
}

fn closures() {
    // Captures the environment state

    let mut num: i32 = 0; // Some variable from environment

    // Closure declaration
    // Closure will capture the state of variable num
    let mut print_num = || {
        println!("Hello {}", num);
        num += 1;
    };

    // Call
    print_num(); // output: 0
    print_num(); // output: 1
}
