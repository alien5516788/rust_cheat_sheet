pub mod scope_vs_lifetime {
    /*
        - Scope and Lifetime are related but represent different concepts in Rust
        - Scope: A compile-time property of a variable. It defines where a name is valid.
        - Lifetime: A property of a reference. It defines how long the data being pointed to is valid.
    */

    // 1. Scope (The "Container")
    fn scope_explanation() {
        /*
            - Scope is defined by curly braces `{ }`
            - When a variable goes "out of scope," its memory is cleaned up (Dropped)
        */

        let x = 10; // x starts here
        {
            let y = 20; // y starts here
            println!("{}", x + y);
        } // y's scope ends here. y is dropped.

        // println!("{}", y); // Error: y is not in this scope
    } // x's scope ends here. x is dropped.

    // 2. Lifetime (The "Duration")
    fn lifetime_explanation() {
        /*
            - A lifetime is the actual duration for which a reference is valid
            - A reference's lifetime cannot be longer than the scope of the variable it points to
        */

        let r; // r is a reference variable
        {
            let x = 5; // x's scope starts
            r = &x; // r borrows x. r's lifetime begins.
            println!("{}", r); // Works: x is still alive
        } // x's scope ends. x is dropped.

        // println!("{}", r); // Error: r's lifetime ended because x is dead (Dangling reference)
    }

    // 3. The Gap: When Scope and Lifetime diverge
    fn scope_vs_lifetime_mismatch() {
        /*
            - A variable's scope can be larger than the reference's lifetime
            - This is often where "Lifetime Generics" come in to bridge the gap
        */

        let x = String::from("data"); // Scope of x is the whole function

        let r1 = &x; // Lifetime of r1 starts
        println!("{}", r1);
        // r1's lifetime ends here because it's never used again (Non-Lexical Lifetimes)

        let r2 = &x; // Lifetime of r2 starts
        println!("{}", r2);
        // r2's lifetime ends here

        // x is still in scope, but r1 and r2 are "dead"
    }

    // 4. Lifetime Generics (Connecting different durations)
    fn lifetime_generics_revisited() {
        /*
            - Generic lifetimes ('a) don't change how long a variable lives
            - They only "label" the relationship so the compiler can verify them
        */

        fn get_first<'a>(s: &'a str) -> &'a str {
            s // The return must live at least as long as 'a
        }

        let outer = String::from("Long");
        let result;
        {
            let inner = String::from("Short");
            // result = get_first(&inner); // Error: result would live longer than inner
            result = get_first(&outer); // Success: outer lives long enough
        }
        println!("{}", result);
    }
}
