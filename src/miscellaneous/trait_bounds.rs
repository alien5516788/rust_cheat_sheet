use std::fmt::{Debug, Display};

fn trait_bounds() {
    // Used for compile time poly morphism

    fn print_twice<T: Display>(item: T) {
        println!("{} {}", item, item);
    }

    print_twice(5); // Works, i32 implements Display
    print_twice("Hello");

    fn generic_function<T: Debug + Clone>(item: T) {
        let cloned_item = item.clone();
        println!("{:?} {:?}", item, cloned_item);
    }

    generic_function(5);
    generic_function("Hello");
}
