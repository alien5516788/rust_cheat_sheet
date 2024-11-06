// regular struct
struct Person {
    name : String,
    age : i32
}

Person {
    name : String::from("my name"),
    age : 21
};

let person = Person {
    name : String::from("my name"),
    age : 21
};

let person : Person = Person {
    name : String::from("my name"),
    age : 21
};

let Person { name, age } = person; // field name should equal to that of struct's

let Person { name, age } : Person = person;

// tuple struct
struct Color (i32, i32, i32, f32);

Color (255, 0, 0, 1.0f32);

let red_color = Color (255, 0, 0, 1.0f32);

let red_color : Color = Color (255, 0, 0, 1.0f32);

let Color (r, g, b, a) = red_color;

let Color (r, g, b, a) : Color = red_color;

// unit like struct
struct Vehical; // this should be implemented via traits and impls

// accessing elements
let person_age = person.age;
let person_age = age;

let red = red_color.0;
let red = r;
