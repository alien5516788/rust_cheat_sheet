/*
    - Rust have 3 main kinds or generics
        1. Type generics
        2. Const generics
        3. Life time generics
    - Generics allows mutiple concrete types to be created out of generic types
    - By default generics use static dispatch
*/

/*
    Generic Kind	 Trait Bounds	 Trait Objects	  Lifetime Bounds
    Type (T)	         Yes	          Yes	            Yes
    Const (N)	         No	              No	            No
    Lifetime ('a)	     No	              No	            Yes
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
    /*
        - Const generics support only integer types, bool, char, and references to them
    */
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
    /*
        - Life time generics on functions define the lifetime of the arguments and return value
        - Life time generics on other items define the lifetime of attributes and items themselves
        - Life time generic is always applied on references
    */

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        /*
            - 'a ties input and output references together
            - Meaning: The returned reference lives at least as long as both inputs
        */
        if x.len() > y.len() {
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
