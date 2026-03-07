fn if_else() {
    /*
        - Conditionally execute a logical block of code
        - Consists of `if`, `else if`, and `else` blocks
    */

    // Example 1
    // Finally evaluates to unit type
    let x = 6;

    if x == 6 {
        println!("x == 6");
    } else if x > 6 {
        println!("x > 6");
    } else {
        println!("x < 6");
    }

    // Example 2
    // Finally evaluates to x
    let x = 6;

    if x == 6 {
        println!("x == 6");
        x // note no semicolon
    } else if x > 6 {
        println!("x > 6");
        x // note no semicolon
    } else {
        println!("x < 6");
        x // note no semicolon
    }; // note semicolon

    // Example 3
    /* Error
    because the evaluated expression is not terminated
        if x == 6 {
            println!("x == 6");
            x // note no semicolon
        } else if x > 6 {
            println!("x > 6");
            x // note no semicolon
        } else {
            println!("x < 6");
            x // note no semicolon
        } // note no semicolon
    */

    // Example 4
    /* Error
    because not all blocks evaluate to same type
        if x == 6 {
            println!("x == 6");
            x; // note semicolon
        } else if x > 6 {
            println!("x > 6");
            x // note no semicolon
        } else {
            println!("x < 6");
            x // note no semicolon
        };
    */

    // Example 5
    // Assigning to a variable
    let x = 6;

    let num = if x == 6 {
        println!("x == 6");
        x
    } else if x > 6 {
        println!("x > 6");
        x
    } else {
        println!("x < 6");
        x
    };
}
