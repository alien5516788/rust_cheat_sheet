// Generics
// ========

/*
    Rust have 3 main kinds or generics
        1. Type generics
        2. Const generics
        3. Life time generics

    Generics allows mutiple concrete types to be created out of generic types
*/

fn type_generics() {
    fn return_tup<T, U>(arg1: T, arg2: U) -> (T, U) {
        (arg1, arg2)
    }

    struct Point<T, U> {
        x: T,
        y: U,
    }

    enum Output1<T> {
        Value(T),
        None,
    }

    trait Communicate<T> {
        fn speak(&self, arg: T) -> T;
    }
}

fn const_generics() {
    // Const generics support only integer types, bool, char, and references to them

    fn return_value<T, const U: i32>(_arg1: T) -> i32 {
        34 + U
    }

    struct List<T, const N: usize> {
        t: T,
        l: [i32; N],
    }

    enum Output<T, const N: usize> {
        Value1([i32; N]),
        Value2(T),
    }

    trait FindLocation<const A: i32> {
        fn is_existing(&self, location: i32) -> bool {
            location + A > 10
        }
    }
}

fn lifetime_generics() {
    // Life time generics on functions define the lifetime of the arguments and return value
    // Life time generics on other items define the lifetime of attributes and items themselves

    fn largest<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
        if x > y {
            x
        } else {
            y
        }
    }

    struct Holder<'a> {
        name: &'a str,
    }

    enum Message<'a> {
        Text(&'a str),
        Number(i32),
    }

    trait Printer<'a> {
        fn print(&self, s: &'a str);
    }
}

// Default type for generics
// =========================

fn default_type() {
    // Default type for generic parameter are not supported for functions

    struct Item<T, U = f32> {
        t: T,
        l: U,
    }

    enum Output3<T, U = f32> {
        Value1(T),
        Value2(U),
    }

    trait MyTrait<T = i32> {
        fn do_something(&self, value: T);
    }
}

// Generic implementations
// =======================

fn implementing_generics_traits_and_structs() {
    // Generic struct
    /*
        Generic struct allows multiple implementations
            for different concrete types of same struct.

        There is a thing to watchout.
            when there is a universal implementation, functions or methods cannot be
            redundant in concrete implementations as they would overlap with the universal implementation.

        Structs still can have multiple implementation blocks for the same concrete type,
            as long as they are not redundant.
    */
    struct Container<T> {
        item: T,
    }

    // Generic impl for all T
    impl<U> Container<U> {
        fn new(item: U) -> Self {
            Self { item }
        }

        fn get_item(&self) -> &U {
            &self.item
        }
    }

    // Specialized impl for specific type
    impl Container<i32> {
        fn double(&self) -> i32 {
            self.item * 2
        }
    }

    // Generic trait
    /*
        Generic trait allows multiple implementations
            for the same type but with different type parameter.

        There is also a thing to watchout.
            When there is a universal implementation, there cannot be implementations
            for any concrete types and vice versa. Becuase they would overlap with the universal implementation.

        Traits still have to be fully implemented in one block.
    */
    trait Interact<T> {
        fn interact(&self, value: T);
    }

    struct Robot {
        id: i32,
    }

    // Implementing generic trait for concrete type
    // Either universal implementation or concrete implementations can present, not both
    /*
        impl<U> Interact<U> for Robot {
            fn interact(&self, value: U) {
                println!("Robot {} received a message", self.id);
            }
        }
    */

    // Implementing generic trait for concrete type
    impl Interact<String> for Robot {
        fn interact(&self, value: String) {
            println!("Robot {} received message: {}", self.id, value);
        }
    }

    // Implementing same trait but different type parameter
    impl Interact<i32> for Robot {
        fn interact(&self, value: i32) {
            println!("Robot {} received number: {}", self.id, value);
        }
    }
}
