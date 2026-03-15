pub mod static_visibility {
    /* @Topic: Static Visibility vs Persistence
        @Insight: Static data is permanent, but its "Name" follows standard scoping rules.
    */

    // 1. Function-Local Static (Your Example)
    pub fn local_static_demo() {
        // X is persistent across calls, but the name 'X' is private to this function.
        static mut X: i32 = 0;

        unsafe {
            X += 1;
            println!("Count: {}", X);
        }
    }

    // 2. The "Shared Private" Pattern (Module Scope)
    mod shared_private {
        // This is visible to 'up' and 'down' but NOT to the outside world.
        static mut COUNTER: i32 = 10;

        pub fn up() {
            unsafe {
                COUNTER += 1;
            }
        }
        pub fn down() {
            unsafe {
                COUNTER -= 1;
            }
        }
    }

    pub fn test() {
        shared_private::up();
        // println!("{}", COUNTER); // Error: Name 'COUNTER' is private to the module.
    }
}
