// ===============================================================
// RUST MACRO SYSTEM CHEAT SHEET
// ===============================================================
// Metaprogramming: Writing code that generates other code.
// Built into the Abstract Syntax Tree (AST) -> Safer than C macros.
// ===============================================================

// ---------------------------------------------------------------
// 1. DECLARATIVE MACROS (macro_rules!)
// ---------------------------------------------------------------
// - Uses: Pattern matching on code tokens.
// - Best for: Variadic functions, repetitive boilerplate, simple DSLs.
// - Hygiene: Variables inside the macro don't leak out (mostly).



macro_rules! say_hello {
    // () indicates no arguments
    () => {
        println!("Hello!");
    };
    // ($name:expr) matches a single expression
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
    // $(...)* matches zero or more repetitions
    ($($x:expr),*) => {
        $(
            println!("Hello, {}!", $x);
        )*
    };
}

// ---------------------------------------------------------------
// 2. PROCEDURAL MACROS (The "Function-like" Macros)
// ---------------------------------------------------------------
// - Requirement: Must be defined in a separate crate of type `proc-macro`.
// - Logic: Functions that take `TokenStream` and return `TokenStream`.
// - Power: Can execute any Rust code during compilation.

// A. CUSTOM DERIVE
// Usage: #[derive(MyTrait)]
// Purpose: Automatically implements a trait for a struct or enum.
// Example: #[derive(Serialize, Deserialize)] from Serde.

// B. ATTRIBUTE-LIKE MACROS
// Usage: #[my_attribute]
// Purpose: Creates custom attributes for items (functions, structs).
// Example: #[tokio::main] or #[get("/")] in Rocket/Actix.

// C. FUNCTION-LIKE MACROS
// Usage: my_macro!(...)
// Purpose: Looks like declarative, but uses complex logic/parsing.
// Example: sqlx::query!("SELECT...") or format_args!(...).

// ---------------------------------------------------------------
// QUICK COMPARISON TABLE
// ---------------------------------------------------------------
/*
 | Feature         | Declarative (macro_rules!) | Procedural               |
 |-----------------|----------------------------|--------------------------|
 | Engine          | Pattern Matching           | Imperative Rust Code     |
 | Input           | Token fragments            | TokenStream              |
 | Definition      | Same crate/module          | Must be a separate crate |
 | Use Case        | Simple repetitions         | Complex logic/Derives    |
*/

// ---------------------------------------------------------------
// WHY USE THEM?
// ---------------------------------------------------------------
// 1. DRY: Eliminate structural boilerplate.
// 2. DSL: Create domain-specific languages (like HTML inside Rust).
// 3. Variadics: Handle unknown number of arguments (e.g., vec![1, 2, 3]).

fn main() {
    // Calling our declarative macro examples:
    say_hello!();             // Match 1
    say_hello!("Alice");      // Match 2
    say_hello!("Bob", "Eve"); // Match 3 (variadic)
}

// [Inference] Based on the Rust compiler's architecture, Procedural macros
// are significantly more resource-intensive at compile time than Declarative ones.
