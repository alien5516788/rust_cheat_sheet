fn str_vs_string_usage() {
    // This function accepts &str, works with both &str and String
    fn greet(name: &str) {
        println!("Hi, {}!", name);
    }

    // Using &str (the lightweight, immutable type)
    let name: &str = "Alice";

    // Passing &str to a function works perfectly
    greet(name);

    // Concatenation without needing String? We can use format! macro
    let greeting = format!("Hello, {}!", name); // returns a String
    println!("{}", greeting);

    // Using String only when necessary
    let mut dynamic_name = String::from("Bob");

    // We want to modify it -> we need String
    dynamic_name.push_str(" the Builder");
    greet(&dynamic_name); // still pass as &str because function accepts &str

    // &str is the noraml guy in rust as opposed to other languages
}
