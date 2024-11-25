// Infinite loop
while true {
    println!("Forever");
    break; // ignore the break
}

// Loop that terminates
let mut i = 0;

while i < 10 {
    println!("Cycle {}", i);
    i += 1;
    // Loop is evaluated to a unit type
}

// Loop that explicitly terminates
i = 0;

while true {
    if i == 10 {
        break; /* Break the loop.
                Loop is evaluated to a unit type.
                Loop cannot be evaluated to an explicit type */
    }
    println!("Cycle {}", i);
    i += 1;
}

// Assigning to a variable
/* Loop can be assigned only to a unit type
    Because loop cannot be evaluated to an explicit type */
i = 0;

let num = while true {
    if i == 10 {
        break; /* Break the loop.
        Loop is evaluated to a unit type.
        Loop cannot be evaluated to an explicit type */
    }
    println!("Cycle {}", i);
    i += 1;
};

i = 0;

let num = {
    while true {
        if i == 10 {
            break; /* Break the loop.
            Loop is evaluated to a unit type.
            Loop cannot be evaluated to an explicit type */
        }
        println!("Cycle {}", i);
        i += 1;
    }
};

// Loop that skips a cycle
i = 0;

while i < 10 {
    if i == 5 {
        i += 1;
        continue; // Skips the execution of current cycle
    }
    println!("Cycle {}", i);
    i += 1;
};
