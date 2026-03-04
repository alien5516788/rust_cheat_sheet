/*
===========================================================
RUST CONSOLE OUTPUT & FORMATTING CHEAT SHEET
===========================================================

CONTENTS:
1. Basic Printing
2. Formatting Placeholders
3. Debug Formatting
4. Width / Alignment / Padding
5. Float Precision
6. Number Bases
7. Capturing Output with format!
8. stderr and dbg!
9. Custom Display Implementation
10. ANSI Color Codes
11. 256 Colors & TrueColor
12. Clean Color Helper Pattern
===========================================================
*/

use std::fmt;

fn main() {
    /*
    ===========================================================
    1. BASIC PRINTING
    ===========================================================
    */

    println!("Hello, world!"); // prints with newline
    print!("Hello ");          // no newline
    print!("World\n");

    /*
    ===========================================================
    2. FORMAT PLACEHOLDERS
    ===========================================================
    */

    println!("Hello, {}!", "Alien");

    // Multiple values
    println!("{} is {} years old.", "Alien", 20);

    // Positional arguments
    println!("{0} likes {1}. {0} loves {1}.", "Alien", "Rust");

    // Named arguments
    println!("{name} is learning {lang}.",
        name = "Alien",
        lang = "Rust"
    );

    /*
    ===========================================================
    3. DEBUG FORMATTING
    ===========================================================
    */

    println!("{}", 42);        // Display
    println!("{:?}", vec![1,2,3]); // Debug

    #[derive(Debug)]
    struct User {
        name: String,
        age: u8,
    }

    let u = User {
        name: "Alien".into(),
        age: 20
    };

    println!("{:?}", u);    // Debug
    println!("{:#?}", u);   // Pretty Debug (multi-line)

    /*
    ===========================================================
    4. WIDTH / ALIGNMENT / PADDING
    ===========================================================
    */

    println!("{:5}", 42);     // width 5 (right aligned default)
    println!("{:>5}", 42);    // right align
    println!("{:<5}", 42);    // left align
    println!("{:^5}", 42);    // center align

    println!("{:05}", 42);    // zero padding
    println!("{:*>5}", 42);   // custom padding with *

    /*
    ===========================================================
    5. FLOAT PRECISION
    ===========================================================
    */

    println!("{:.2}", 3.141592);      // 2 decimal places
    println!("{:8.2}", 3.141592);     // width + precision

    /*
    ===========================================================
    6. NUMBER BASES
    ===========================================================
    */

    println!("{:b}", 10);  // binary
    println!("{:o}", 10);  // octal
    println!("{:x}", 10);  // hex (lowercase)
    println!("{:X}", 10);  // hex (uppercase)

    /*
    ===========================================================
    7. CAPTURE FORMATTED STRING
    ===========================================================
    */

    let s = format!("Hello, {}!", "Alien");
    println!("{}", s);

    /*
    ===========================================================
    8. STDERR & DEBUG MACRO
    ===========================================================
    */

    eprintln!("This goes to stderr");

    let value = 42;
    dbg!(value); // prints file:line + value

    /*
    ===========================================================
    9. CUSTOM DISPLAY IMPLEMENTATION
    ===========================================================
    */

    struct Person {
        name: String,
        age: u8,
    }

    impl fmt::Display for Person {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} ({})", self.name, self.age)
        }
    }

    let p = Person {
        name: "Alien".into(),
        age: 20
    };

    println!("{}", p); // uses Display

    /*
    ===========================================================
    10. ANSI COLOR CODES (RAW TERMINAL CONTROL)
    ===========================================================

    Format:
        \x1b[<code>m   → set style
        \x1b[0m        → reset

    Foreground colors:
        30 Black
        31 Red
        32 Green
        33 Yellow
        34 Blue
        35 Magenta
        36 Cyan
        37 White

    Background colors:
        40–47

    Bold:
        1
    */

    println!("\x1b[31mRed Text\x1b[0m");
    println!("\x1b[32mGreen Text\x1b[0m");
    println!("\x1b[1;34mBold Blue Text\x1b[0m");
    println!("\x1b[41mRed Background\x1b[0m");

    /*
    ===========================================================
    11. 256 COLOR MODE
    ===========================================================

    Foreground:
        \x1b[38;5;<n>m

    Background:
        \x1b[48;5;<n>m
    */

    println!("\x1b[38;5;208m256 Color Orange\x1b[0m");

    /*
    ===========================================================
    12. TRUECOLOR (24-bit RGB)
    ===========================================================

    Format:
        \x1b[38;2;R;G;Bm
    */

    println!("\x1b[38;2;255;100;0mTrueColor Custom Orange\x1b[0m");

    /*
    ===========================================================
    13. CLEAN COLOR HELPER PATTERN
    ===========================================================
    */

    success("Scan complete");
    error("Port unreachable");
}

/*
Reusable CLI-style helpers
*/

fn success(msg: &str) {
    println!("\x1b[32m{}\x1b[0m", msg);
}

fn error(msg: &str) {
    eprintln!("\x1b[1;31m{}\x1b[0m", msg);
}

/*
===========================================================
END OF CHEAT SHEET
===========================================================

NOTES:

- All formatting uses traits like:
    Display, Debug, LowerHex, UpperHex, Binary, etc.
- Formatting is compile-time checked.
- Colors are interpreted by the terminal (not Rust).
- Use crates like "colored" or "crossterm" for production CLI tools.

===========================================================
*/
