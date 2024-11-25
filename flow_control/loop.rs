// Infinite loop
loop {
    println!("Forever");
    break; // ignore the break
}

// Loop that terminates
let mut i = 0;

loop {
    if i == 10 {
        break; /* Break the loop.
                Loop is evaluated to a unit type. */
    }
    println!("Cycle {}", i);
    i += 1;
}

i = 0;

loop {
    if i == 10 {
        break 10; /* Break the loop.
                    Loop is evaluated to 10 . */
    }
    println!("Cycle {}", i);
    i += 1;
};

// Assigning to a variable
i = 0;

let num = loop {
    if i == 10 {
        break 10; /* Break the loop.
                    Loop is evaluated to 10 . */
    }
    println!("Cycle {}", i);
    i += 1;
};

i = 0;

let num = {
    loop {
        if i == 10 {
            break 10; /* Break the loop.
            Loop is evaluated to 10 . */
        }
        println!("Cycle {}", i);
        i += 1;
    }
};

// Loop that skips a cycle
i = 0;

loop {
    if i == 5 {
        i += 1;
        continue; // Skips the execution of current cycle
    }
    if i == 10 {
        break;
    }
    println!("Cycle {}", i);
    i += 1;
}
