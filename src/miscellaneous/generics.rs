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
    // Generic struct
    struct Point<T, U> {
        x: T,
        y: U,
    }

    let p = Point { x: 1, y: 2.4 };
    let p = Point::<i32, f64> { x: 1, y: 2.4 };

    // Generic struct with a const parameter
    struct List<T, const U: usize> {
        t: T,
        l: [i32; U],
    }

    let l = List {
        t: "Hello",
        l: [1, 2, 3],
    };
    let l = List::<&str, 3> {
        t: "Hello",
        l: [1, 2, 3],
    };

    // Generic struct with default type parameter
    struct Item<T, U = f32> {
        t: T,
        l: U,
    }

    let i = Item {
        t: "World",
        l: 3.14,
    };
    let i = Item::<&str> {
        t: "World",
        l: 3.14,
    };
}

fn generics_enums() {
    // Generic enum
    enum Output1<T> {
        Value(T),
        None,
    }
    
    let o = Output1::Value("Hello");
    let o = Output1::<&str>::None;

    // Generic enum with default type parameter and const parameter
    enum Output2<T, const U: usize> {
        Value1([i32; U]),
        Value2(T),
    }
    
    let o = Output2::Value1::<[i32; 3], 3>([1, 2, 3]);
    let o = Output2::<&str, 3>::Value2("Hello");

    // Generic enum with default type parameter
    enum Output3<T, U = f32> {
        Value1(T),
        Value2(U),
    }
    
    let o = Output3::Value1::<&str>("Hello");
    let o = Output3::<&str>::Value2(3.14);

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
}
