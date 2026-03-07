fn pattern_match() {
    /*
        - Used match patterns
        - Similar to switch case
        - Matching should be exhaustive
        - Also type of matching pattern should be known at compile time
    */
    // Example
    let num = true;

    match num {
        true => println!("Num 1"),
        false => println!("Num 2"),
    }

    /*
        - Default handle to match remaining values
    */
    // Example 1
    // Default handle should be the last handle
    let num = 2;

    match num {
        1 => println!("Num 1"),
        2 => println!("Num 2"),
        3 => println!("Num 3"),
        4 => println!("Num 4"),
        other1 => println!("Num default"), // default value 1
        other2 => println!("Num default {}", other2), // default value 2 (never matched)
    }

    // Example 2
    // Wildcard pattern `_` to match and discard remaining values
    let num = 10;

    match num {
        1 => println!("Num 1"),
        2 => println!("Num 2"),
        3 => println!("Num 3"),
        4 => println!("Num 4"),
        _ => println!("Num default"), /* default value */
    }

    /*
        - The type of the matching value is same as the type of the variable being matched
    */
    // Example 1
    let x = 10;

    match x {
        val => println!("Matched {}", val), // value copied to `num`, because num (integers) are `Copy`
    }

    println!("{}", x);

    // Example 2
    let s = String::from("hello");

    match s {
        val => println!("Moved: {}", val), // value moved to `s`, because s (String) is not `Copy`
    }

    // println!("{}", s); // error

    // Example 3
    let s = String::from("hello");

    match &s {
        val => println!("Borrowed: {}", val), // value is borrowed
    }

    println!("{}", s);
}
