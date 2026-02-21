// Structs vs Enums
// ================

/*
   Rust data types are based on algebraic data types (ADT).
   There are 2 fundamental constructions:
        1. STRUCTS  -> Product Types  (AND types) (A × B × C)
        2. ENUMS    -> Sum Types      (OR types) (A + B + C)
*/

fn structs_vs_enums() {
    // A struct means: "This value contains ALL of these fields."
    // One possible type

    // Unit struct
    //     No fields
    //     Used as marker/type-level distinction
    struct Home;

    // Tuple struct
    //     No field names
    struct Color(i32, i32, i32, f32);

    // Named field struct
    struct Person {
        name: String,
        age: i32,
    }

    // An enum means: "This value can be ONE of several possibilities."
    // Many possible types

    // Enum with unit types
    enum Colors {
        Red,
        Green,
        Blue,
    }

    // Enum with tuple types
    enum Shapes {
        Circle(f32),
        Rectangle(f32, f32),
        Triangle(f32, f32, f32),
    }

    // Enum with named field types
    enum Organisms {
        Animal { name: String, age: i32 },
        Plant { name: String, height: f32 },
    }
}
