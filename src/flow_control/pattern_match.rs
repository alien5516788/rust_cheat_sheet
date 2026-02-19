fn pattern_match() {
    
    // Used match patterns
    // Similar to switch case
    let num1 = 2;
    
    match num1 {
        1 => println!("Num 1"),
        2 => println!("Num 2"),
        3 => println!("Num 3"),
        4 => println!("Num 4"),
        other1 => println!("Num default"), // Default values
        other2 => println!("Num default {}", other2) // Default value, never matched
    }
    
    let num2 = 10;
    
    match num2 {
        1 => println!("Num 1"),
        2 => println!("Num 2"),
        3 => println!("Num 3"),
        4 => println!("Num 4"),
        _ => println!("Num default default"), /* Default value, empty variable */                       
    }

}
