
// ===============================
// TRAIT BOUNDS
// ===============================

fn trait_bounds() {
    use std::fmt::Debug;

    fn print_debug<T: Debug>(item: T) {
        println!("{:?}", item);
    }

    fn print_debug_where<T>(item: T)
    where
        T: Debug,
    {
        println!("{:?}", item);
    }

    print_debug(5);
}

// ===============================
// MULTIPLE TRAIT BOUNDS
// ===============================

fn multiple_trait_bounds() {
    use std::fmt::Debug;

    fn combine<T: Debug + Clone>(x: T) -> T {
        println!("{:?}", x);
        x.clone()
    }

    let _v = combine(String::from("hello"));
}
