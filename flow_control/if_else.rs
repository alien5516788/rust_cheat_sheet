let x = 6;

// finally evaluates to unit type
if x == 6 {
    println!("x == 6");
} else if x > 6 {
    println!("x > 6");
} else {
    println!("x < 6");
}

// finally evaluates to x
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

/* error 
    because the evaluated expression is not terminated */
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

/* error 
    because not all blocks evaluate to same type */
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

// assigning to a variable
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

let num = {
    if x == 6 {
        println!("x == 6");
        x
    } else if x > 6 {
        println!("x > 6");
        x
    } else {
        println!("x < 6");
        x
    }
};
