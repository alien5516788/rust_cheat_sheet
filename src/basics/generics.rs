// Generics
// ========

/*
   Rust have 3 main kinds or generics
       1. Type generics
       2. Const generics
       3. Life time generics
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
