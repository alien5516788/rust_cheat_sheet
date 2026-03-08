pub mod types_of_generics {
    /*
        - Rust have 3 main kinds or generics
            1. Type generics
            2. Const generics
            3. Life time generics
        - Generics allows mutiple concrete types to be created out of generic types
        - By default generics use static dispatch
    */

    /*
        - Generic bounds support
            |Generic Kind|	 |Trait Bounds|	 |Trait Objects|	 |Lifetime Bounds|
            Type (T)	         Yes	          Yes	            Yes
            Const (N)	         No	              No	            No
            Lifetime ('a)	     No	              No	            Yes
    */

    // 1. Type generics
    /*
        - Type generics to specify the concrete type for generic parameters
        - Allows us to write generic code that can work with multiple concrete types
    */
    fn type_generics() {
        // Example 1
        /*
           - The concrete type for `T` and `U` is determined at compile time by the caller
        */
        fn return_tup<T, U>(arg1: T, arg2: U) -> (T, U) {
            return (arg1, arg2);
        }

        return_tup(3.4, 56);
        return_tup::<f32, i32>(3.4, 56);
        return_tup::<f32, &str>(3.5, "world");

        // Example 2
        /*
            - The concrete type for `T` and `U` is determined at compile time by struct instantiation
        */
        struct Point<T, U> {
            x: T,
            y: U,
        }

        let point = Point { x: 3.4, y: 56 };
        let point = Point::<f32, i32> { x: 3.5, y: 5 };

        // Example 3
        /*
            - The concrete type for `T` is determined at compile time by enum instantiation
        */
        enum Output<T> {
            Value(T),
            None,
        }

        let output = Output::Value(34);
        let output = Output::<&str>::Value("hello");

        // Example 4
        /*
            - The concrete type for `T` is determined at compile time by trait implementation
        */
        trait Communicate<T> {
            fn speak(&self, arg: T);
        }

        struct Person;

        impl Communicate<i32> for Person {
            fn speak(&self, arg: i32) {
                println!("I yelled {}", arg);
            }
        }

        // Example 5
        /*
            - Trait implementation for `Human` that works with any type `U`
        */
        struct Human;

        impl<U> Communicate<U> for Human {
            fn speak(&self, arg: U) {
                println!("I say nothing, becuase I don't know what to say with T");
            }
        }
    }

    // 2. Const generics
    /*
        - Const generics allow specifying constant values as generic parameters instead of types
        - Const generics support only integer types, bool, char, and references to them
        - Unlike type generics, the constant value cannot be inferred from the context and must be explicitly
            provided inside the turbofish syntax
    */
    fn const_generics() {
        // Example 1
        /*
            - The constant value for `U` is determined by the caller at compile time
        */
        fn return_value<T, const U: i32>(arg1: T) -> i32 {
            return 34 + U;
        }

        return_value::<i32, 42>(34);

        // Example 2
        /*
            - The constant value for `N` is determined by struct instantiation
        */
        struct List<T, const N: usize> {
            t: T,
            l: [i32; N],
        }

        let list = List { t: 23, l: [0; 7] };
        let list = List::<i32, 4> { t: 23, l: [0; 4] };

        // Example 3
        /*
            - The constant value for `N` is determined by enum instantiation
        */
        enum Output<T, const N: usize> {
            Item([i32; N]),
            Value(T),
        }

        let output = Output::<i32, 6>::Value(34);

        // Example 4
        /*
            - Constant value for the `O` parameter is determined by trait implementation
        */
        trait FindLocation<const O: i32> {
            fn is_existing(&self, location: i32) -> bool {
                return location + O > 10;
            }
        }

        struct Rover;

        impl FindLocation<100> for Rover {} // Default implementation automatically uses constant value 100
    }

    // 3. Life time generics
    /*
        - Describe the relationship between the lifetimes of references
        - Life time generics on functions define the lifetime of the arguments and return value
        - Life time generics on other items define the lifetime of attributes and items themselves
        - Always applied on references
    */
    // TODO: Complete examples and add `'static` lifetime example
    fn lifetime_generics() {
        // Example 1
        /*
            - 'a ties input and output references together
            - Both `x` and `y` must live at least as long as both input references
        */
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        // Example 2
        /*
            - Both `name` and `title` must live at least as long as both references
        */
        struct Holder<'a> {
            name: &'a str,
            title: &'a str,
            age: i32,
        }

        // Example 3
        /*
            - References inside `Text` and `Number` must live at least as long as the reference passed in
        */
        enum Message<'a> {
            Text(&'a str),
            Number(&'a i32),
            Flag(bool),
        }

        // Example 4
        /*
            -
        */
        trait Printer<'a> {
            fn print(&self, s: &'a str);
        }
    }
}
