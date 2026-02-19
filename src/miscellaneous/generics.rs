fn generic_functions() {
    // Generic function
    fn return_value_1<T, U>(arg1: T, arg2: U) -> i32 {
        34
    }

    // Generic function with a const parameter
    // Only integer, boolean and char are supported as const generic parameters
    fn return_vlaue_2<T, const U: i32>(arg1: T) -> i32 {
        34 + U
    }

    let x = return_value_1(1, 4.5);
    let x = return_vlaue_2::<f32, 42>(1.2);

    // For functions, default type parameters are not supported
}

fn gneric_structs() {
    struct Point<T, U> {
        x: T,
        y: U,
    }
}

fn generics() {
    // Struct generics

    // Generic enum
    enum Output<T> {
        Value(T),
        None,
    }

    // Generic trait
    trait GenericTrait<T> {
        fn generic_method(&self, arg: T) -> T;
    }

    impl<T> GenericTrait<T> for Point<T, T> {
        fn generic_method(&self, arg: T) -> T {
            arg
        }
    }

    // Life time generics
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // const generics
    struct ArrayHolder<T, const N: usize> {
        arr: [T; N],
    }

    let numbers = ArrayHolder::<i32, 5> {
        arr: [1, 2, 3, 4, 5],
    };
}
