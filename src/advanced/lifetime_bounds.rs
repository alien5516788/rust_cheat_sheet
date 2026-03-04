// Lifetime Bounds
// ===============

/*
    Lifetime generics and lifetime bounds are related but different concepts.

    Lifetime generics:
        - Introduce named lifetimes (e.g. 'a, 'b)
        - Describe relationships between references

    Lifetime bounds:
        - Restrict how long a type or reference must live
        - Express outlives relationships
        - Are checked at compile time

    All lifetime relationships are enforced at compile time.
*/

fn lifetime_generics() {
    /*
        Lifetime generics describe relationships between references.

        They do NOT restrict types.
        They only connect lifetimes together.
    */

    // 'a ties input and output references together
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    /*
        Meaning:
            The returned reference lives at least as long as both inputs.
    */
}

fn lifetime_bounds_on_references() {
    /*
        Lifetime bounds describe outlives relationships.

        Syntax:
            'a: 'b  =>  'a outlives 'b
    */

    fn example<'a, 'b>(x: &'a str)
    where
        'a: 'b,   // 'a must live at least as long as 'b
    {
        // This allows coercing &'a str into &'b str
        let _y: &'b str = x;
    }
}

fn lifetime_bounds_on_types() {
    /*
        Lifetime bounds can be applied to generic types.

        Syntax:
            T: 'a   =>  T must not contain references shorter than 'a
    */

    fn takes_bound<'a, T>(value: T)
    where
        T: 'a,
    {
        /*
            Meaning:
                Any references inside T must live at least as long as 'a.
        */
    }
}

fn static_lifetime_bound() {
    /*
        'static is a special lifetime representing the entire program duration.

        T: 'static means:
            T contains no non-'static references.

        It does NOT mean:
            - The value lives forever
            - The value is global
            - Memory is leaked
    */

    fn takes_static<T: 'static>(value: T) {
        /*
            Commonly required in:
                - Threads
                - Async tasks
                - Trait objects
        */
    }

    // Valid: owns its data
    takes_static(String::from("Alien"));

    // Invalid example (uncommenting would fail)
    /*
    let s = String::from("Alien");
    takes_static(&s); // error: borrowed value does not live long enough
    */
}

fn lifetime_bounds_on_trait_objects() {
    /*
        Lifetime bounds can be applied to trait objects.

        Syntax:
            dyn Trait + 'a
    */

    trait Describable {
        fn describe(&self) -> String;
    }

    fn takes_trait_object(obj: Box<dyn Describable + 'static>) {
        /*
            The trait object must not contain
            non-'static references.
        */
    }
}

fn lifetime_generics_vs_bounds_summary() {
    /*
        Lifetime generics:
            - Introduce lifetimes
            - Relate references
            - Example: fn foo<'a>(x: &'a str) -> &'a str

        Lifetime bounds:
            - Constrain lifetimes
            - Express outlives relationships
            - Example: T: 'a
                       'a: 'b

        Think of it as:

            Generics define lifetimes.
            Bounds restrict them.
    */
}
