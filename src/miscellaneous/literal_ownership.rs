fn main() {
    // --- CONCEPT 1: PRIMITIVE LITERALS ---
    // The value '5' doesn't have an "owner" in the heap sense.
    // It is a Copy type, stored on the stack.
    5;
    // [Inference] Based on observed LLM behavior, the compiler likely
    // removes the line above during optimization because it has no effect.

    // --- CONCEPT 2: VARIABLE OWNERSHIP ---
    // 's1' is the clear owner of this String.
    // It will live until the end of this main() function.
    let s1 = String::from("I am owned by s1");
    print_string(&s1);
    // s1 still owns it here.

    // --- CONCEPT 3: TEMPORARY OWNERSHIP ---
    // Here, we create a String and immediately take a reference.
    // No variable name owns this String.
    // The "Statement" is the owner.
    print_string(&String::from("I am a temporary"));

    // AT THIS POINT: The "temporary" string is already dropped/freed.
    // The memory is gone before we even reach the next line.

    // --- CONCEPT 4: STRING LITERALS (&str) ---
    // This is different. "Hello" is stored in the binary itself.
    // Its "owner" is the program memory (Static storage).
    let s2: &str = "I live forever";
    print_str(s2);

    println!("End of main: s1 will be dropped now.");
}

fn print_string(s: &String) {
    println!("Printing String: {}", s);
}

fn print_str(s: &str) {
    println!("Printing str slice: {}", s);
}

pub mod literal_lifetimes {
    /* @Topic: Literals and Constant Promotion
        @Insight: Not all literal references are equal. Some are promoted
                  to 'static, while others are tied to local scopes.
    */

    pub fn demonstrate_promotion() {
        /*
            - CONSTANT PROMOTION: When you take a reference to a literal
              (like &34), Rust "promotes" it to a static location in the
              program binary.
            - This reference effectively has a &'static i32 lifetime.
        */
        let promoted: &'static i32 = &34;

        println!("Promoted value: {}", promoted);
    }

    pub fn demonstrate_non_promotion() {
        /*
            - If the literal is used to initialize a variable, that variable
              is stored on the STACK.
            - A reference to that variable is NOT 'static; it is tied to
              the local scope.
        */
        let x = 34; // x is on the stack
        let local_ref = &x; // local_ref has a local lifetime, NOT 'static

        // let invalid: &'static i32 = &x; // Error: x does not live long enough

        println!("Local value: {}", local_ref);
    }

    pub fn string_literals() {
        /*
            - String literals (e.g., "hello") are ALWAYS &'static str.
            - They are stored directly in the "Data Segment" of the binary.
        */
        let s: &'static str = "I am in the binary";
    }
}
