fn loops() {
    /*
       - Repeatedly run a logical block of code
    */

    // Example 1
    // Infinite loop
    loop {
        println!("Forever");
        break; // ignore `break`
    }

    // Example 2
    // Loop that terminates
    let mut i = 0;

    loop {
        if i >= 10 {
            break; /* Break the loop.
                   Loop is evaluated to a unit type. */
        }
        println!("Cycle {}", i);
        i += 1;
    }

    // Example 3
    // Loop that returns a value
    let mut i = 0;

    _ = loop {
        if i >= 10 {
            break 10; /* Break the loop.
                      Loop is evaluated to 10 . */
        }
        println!("Cycle {}", i);
        i += 1;
    };

    // Example 4
    // Assigning to a variable
    let mut i = 0;

    let _num = loop {
        if i >= 10 {
            break 10; /* Break the loop.
                      Loop is evaluated to 10 . */
        }
        println!("Cycle {}", i);
        i += 1;
    };

    // Example 5
    // Loop that skips a cycle
    let mut i = 0;

    loop {
        if i >= 5 {
            i += 1;
            continue; // skips the execution of current cycle
        }
        if i >= 10 {
            break;
        }
        println!("Cycle {}", i);
        i += 1;
    }
}
