// Used match patterns
// Similar to switch case
let num = 2;

match num {
    1 => println!("Num 1"),
    2 => println!("Num 2"),
    3 => println!("Num 3"),
    4 => println!("Num 4"),
    other1 => println!("Num 5"), // Default values
    other2 => println!("Num default {}", other2)
    // pattern will be matched with the first default value
}

match num {
    1 => println!("Num 1"),
    2 => println!("Num 2"),
    3 => println!("Num 3"),
    4 => println!("Num 4"),
    _ => println!("Num default 1"), /* Default values */
    _ => println!("Num default 2")  /* If we are not going to do with the default number */
    // pattern will be matched with the first default value                               
}

let lt = "dd";

match lt {
    "a" => println!("Letter a"),
    "b" => println!("Letter b"),
    "c" => println!("Letter c"),
    "d" => println!("Letter d"),
    "e" => println!("Letter e"),
    _ => println!("Letter default") // Default value
}
