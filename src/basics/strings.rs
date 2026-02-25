// ==============================
// Rust String Cheatsheet
// ============================== TODO

fn main() {
    // ------------------------------
    // 1️⃣ String Literals (&str)
    // ------------------------------
    let s_literal: &str = "Hello, Rust!";
    println!("String literal: {}", s_literal);
    // Immutable, stored in binary, UTF-8, type: &'static str

    // ------------------------------
    // 2️⃣ Growable String (String)
    // ------------------------------
    let mut s = String::from("Hello");
    s.push_str(", Rust!");
    s.push('🌍'); // Append a single char (can be multi-byte)
    println!("Growable String: {}", s);
    // Mutable, heap-allocated, can grow/shrink

    // Convert String to &str
    let s_ref: &str = &s;
    println!("As &str: {}", s_ref);

    // ------------------------------
    // 3️⃣ Byte String Literals (b"")
    // ------------------------------
    let bytes: &[u8] = b"Hello"; // type: &[u8; 5]
    println!("Byte string literal: {:?}", bytes);
    // ASCII only, immutable, raw bytes

    // ------------------------------
    // 4️⃣ UTF-8 Strings and Bytes
    // ------------------------------
    let utf8_str = "你好"; // 2 Chinese characters
    println!("UTF-8 bytes: {:?}", utf8_str.as_bytes());
    println!("Length in chars: {}", utf8_str.chars().count());
    println!("Length in bytes: {}", utf8_str.len());
    // len() = bytes, chars().count() = Unicode scalar values

    // ------------------------------
    // 5️⃣ Raw Strings (r"")
    // ------------------------------
    let path = r"C:\Users\Alien\Documents";
    println!("Raw string: {}", path);

    let quote = r#"He said, "Hello!""#;
    println!("Raw string with quotes: {}", quote);
    // Useful to avoid escaping backslashes or quotes

    // ------------------------------
    // 6️⃣ Common Operations
    // ------------------------------
    let mut text = String::from("Rust");

    // Append string
    text.push_str(" is awesome!");

    // Append single character
    text.push('🚀');

    // Iterate over characters
    println!("Characters:");
    for c in text.chars() {
        println!("{}", c);
    }

    // Convert to bytes
    let bytes = text.as_bytes();
    println!("Bytes: {:?}", bytes);

    // ------------------------------
    // 7️⃣ Summary Table in Code
    // ------------------------------
    println!("\nSummary:");
    println!("&str        => immutable, stored in binary, UTF-8, literal");
    println!("String      => mutable, heap-allocated, growable, UTF-8");
    println!("b\"...\"      => immutable, byte string literal, ASCII only");
    println!("r\"...\"      => raw string literal, no escaping needed");
}
