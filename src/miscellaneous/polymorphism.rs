// Polymorphism
// ============

/*
    Polymorphism = ability for different types
        to respond to the same behaviour differently.

    In Rust, polymorphism happens mainly through:
        1. Trait bounds (static dispatch)
        2. Trait objects (dynamic dispatch)
*/

// Static Dispatch
// ===============

fn static_dispatch_via_function_pointers() {
    // Refer `basics/functions.rs -> function_pointers`
}

fn static_dispatch_via_generics() {
    /*
        Static dispatch means:
            The exact function to call is known at compile time.
            No runtime lookup.
            No vtable.
            Monomorphization happens.

        Zero runtime cost.
    */

    trait Speak {
        fn speak(&self);
    }

    struct Human;
    struct Dog;

    impl Speak for Human {
        fn speak(&self) {
            println!("Hello");
        }
    }

    impl Speak for Dog {
        fn speak(&self) {
            println!("Woof");
        }
    }

    // Generic function with trait bounds
    fn make_speak<T>(entity: T)
    where
        T: Speak,
    {
        entity.speak();
    }

    // Generic function (`impl Trait` syntactic sugar)
    // Exactly same thing as generic function with trait bounds
    fn make_speak2(entity: impl Speak) {
        entity.speak();
    }

    /*
        For each type used,
        compiler generates a separate version:

        make_speak(Human)
        make_speak(Dog)

        This is monomorphization.
    */

    let h = Human;
    let d = Dog;

    make_speak(h);
    make_speak(d);
}

// Dynamic Dispatch
// ================

fn dynamic_dispatch_trait_objects() {
    /*
        Dynamic dispatch means:
            The exact function is decided at runtime.
            Uses trait objects.
            Uses a vtable.
            Slight runtime overhead.
    */

    trait Speak {
        fn speak(&self);
    }

    struct Human;
    struct Dog;

    impl Speak for Human {
        fn speak(&self) {
            println!("Hello");
        }
    }

    impl Speak for Dog {
        fn speak(&self) {
            println!("Woof");
        }
    }

    // Trait object
    fn make_speak(entity: &dyn Speak) {
        entity.speak();
    }

    let h = Human;
    let d = Dog;

    make_speak(&h);
    make_speak(&d);

    /*
        &dyn Speak means,
            entity is a fat pointer
            contains,
                1. pointer to data
                2. pointer to vtable
    */
}

fn dynamic_dispatch_via_heap() {
    trait Speak {
        fn speak(&self);
    }

    struct Cat;

    impl Speak for Cat {
        fn speak(&self) {
            println!("Meow");
        }
    }

    let animal: Box<dyn Speak> = Box::new(Cat);

    animal.speak();

    /*
        Stored on heap.
        Called via vtable.
        Runtime dispatch.
    */
}

/*
    Trait Object = dyn Trait

    dyn Trait is unsized.
    Must be behind pointer:

        &dyn Trait
        Box<dyn Trait>
        Rc<dyn Trait>
        Arc<dyn Trait>

    Trait objects require:
        - Trait must be object safe.
*/

// Object Safety Rules (Important)

/*
    A trait is object safe if:

    - It does NOT return Self.
    - It does NOT use generic methods.
    - Methods use &self / &mut self / self.

    Otherwise it cannot become dyn Trait.
*/

// Function Pointer vs Trait Object

/*
    Function pointer: fn(i32) -> i32

    - Static dispatch
    - Just pointer to code
    - Cannot capture environment

    Trait object: dyn Fn(i32) -> i32

    - Dynamic dispatch
    - Can represent closures with captures
    - Uses vtable
*/

// Polymorphism Types in Rust

/*
    1) Parametric polymorphism
       -> Generics
       -> static dispatch

    2) Ad-hoc polymorphism
        -> Traits
        -> static or dynamic

    3) Subtype-like behaviour
        -> Trait objects (dyn Trait)
*/
