// Structs
// =======

/*
    Structs define types

    There are 3 types of structs
        1. Unit structs
        2. Tuple structs
        3. Named filed structs
*/

fn unit_structs() {
    struct Vehical;

    let _vehical = Vehical;
}

fn tuple_structs() {
    struct Color(i32, i32, i32, f32);

    let _color = Color(255, 0, 0, 1.0f32);

    // Destructuring
    let Color(_r, _g, _b, _a) = _color;
}

fn named_filed_structs() {
    struct Person {
        name: String,
        age: i32,
    }

    let _person = Person {
        name: String::from("my name"),
        age: 21,
    };

    // Destructuring
    let Person { name, age } = _person; // field name should equal to that of struct's
}
