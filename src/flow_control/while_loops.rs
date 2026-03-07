fn while_loop() {
    /*
       - Repeatedly run a logical block of code with a condition
    */

    // Example 1
    // Infinite loop
    while true {
        println!("Forever");
        break; // ignore `break`
    }

    // Example 2
    // While loop that terminates
    let mut i = 0;

    while i < 10 {
        println!("Cycle {}", i);
        i += 1;
        // Loop is evaluated to a unit type
    }

    // Example 3
    // While loop that explicitly terminates
    let mut i = 0;

    while true {
        if i >= 10 {
            break; /* Break the loop.
                   Loop is evaluated to a unit type.
                   Loop cannot be evaluated to an explicit type */
        }
        println!("Cycle {}", i);
        i += 1;
    }

    // Example 4
    // Assigning to a variable
    /* While loop can be assigned only to a unit type,
    because loop cannot be evaluated to an explicit type */
    let mut i = 0;

    let _num = while true {
        if i >= 10 {
            break; /* Break the loop.
                   Loop is evaluated to a unit type.
                   Loop cannot be evaluated to an explicit type */
        }
        println!("Cycle {}", i);
        i += 1;
    };

    // Example 5
    // While loop that skips a cycle
    let mut i = 0;

    while i < 10 {
        if i == 5 {
            i += 1;
            continue; // skips the execution of current cycle
        }
        println!("Cycle {}", i);
        i += 1;
    }
}
