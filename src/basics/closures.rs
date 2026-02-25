// Closures
// ========

fn closures() {
    // Anonymous functions
    let fun = |name: String| {
        println!("Hello {}", name);
    };

    let fun_ptr: fn(String) = fun;

    fun(String::from("Name"));
    fun_ptr(String::from("Name"));

    // Captures the environment state
    let mut _num: i32 = 0; // Some variable from environment

    let mut print_num = |num| {
        // TODO: function signature
        println!("Hello {}", _num);
        _num += num;
    };

    print_num(1); // output: 1
    print_num(2); // output: 3
}

// Closures & move Keyword TODO
// ========================

/*
    Closures are anonymous functions that can capture
        variables from their surrounding environment.

    Syntax:
        let c = |params| { body };

    Closures automatically decide how to capture variables:
        - &T       (immutable borrow)
        - &mut T   (mutable borrow)
        - T        (move / ownership)

    The `move` keyword forces capture by value.
*/

fn normal_closure_capture() {
    /*
        Without `move`,
            Rust captures variables in the least restrictive way.
    */

    let x = 10; // Copy type

    let c = || {
        println!("x = {}", x);
    };

    /*
        x is captured by immutable reference (&x)
            because we only read it.
    */

    c();
    println!("Still usable: {}", x); // Works
}

fn closure_mutable_capture() {
    /*
        If closure modifies a variable,
            Rust captures by mutable reference (&mut T)
    */

    let mut count = 0;

    let mut increment = || {
        count += 1;
    };

    increment();
    increment();

    println!("count = {}", count);
}

fn closure_move_capture() {
    /*
        `move` forces capture by value.

        The closure takes ownership
            of variables it uses.
    */

    let s = String::from("Hello");

    let c = move || {
        println!("{}", s);
    };

    c();

    /*
        s is moved into closure.
        Cannot use s here anymore.
    */
    // println!("{}", s); // ERROR
}

fn move_with_copy_types() {
    /*
        If the captured type implements Copy,
            it is copied instead of moved.
    */

    let x = 5; // i32 is Copy

    let c = move || {
        println!("{}", x);
    };

    c();

    /*
        x still usable,
            because it was copied.
    */
    println!("Still usable: {}", x);
}

fn partial_capture_example() {
    /*
        Closure captures only variables it actually uses.
    */

    let a = String::from("A");
    let b = String::from("B");

    let c = move || {
        println!("{}", a);
    };

    /*
        Only `a` is moved.
        `b` is untouched.
    */

    println!("{}", b); // Works
    c();
}

fn thread_usage() {
    use std::thread;

    /*
        Threads require 'static closures.

        That means:
            The closure must own everything it uses.

        Therefore, `move` is almost always required.
    */

    let s = String::from("Hello from thread");

    let handle = thread::spawn(move || {
        println!("{}", s);
    });

    /*
        s is moved into thread.
        Main thread cannot use it anymore.
    */

    handle.join().unwrap();
}

fn thread_multiple_values() {
    use std::thread;

    /*
        All used variables are moved
            into the closure.
    */

    let name = String::from("Alien");
    let age = 20;

    let handle = thread::spawn(move || {
        println!("{} is {}", name, age);
    });

    /*
        name is moved (String)
        age is copied (i32 is Copy)
    */

    handle.join().unwrap();
}

fn arc_example_with_move() {
    use std::sync::Arc;
    use std::thread;

    /*
        move does NOT clone automatically.

        If multiple threads need ownership,
            you must clone explicitly.
    */

    let data = Arc::new(5);

    let data2 = Arc::clone(&data);

    let handle = thread::spawn(move || {
        println!("{}", data2);
    });

    /*
        data still usable,
            because Arc was cloned.
    */

    println!("{}", data);

    handle.join().unwrap();
}

/*
    Important Notes
    ===============

    1. `move` affects how variables are captured,
        not how they are used inside the closure.

    2. `move` does NOT move everything in scope.
        Only variables actually used are captured.

    3. Closure trait implementation depends on usage:

        - If it only reads -> Fn
        - If it mutates    -> FnMut
        - If it consumes   -> FnOnce

    4. `move` does NOT automatically make a closure FnOnce.
        It depends on how captured values are used.

    5. Threads require move because:
        Borrowed references may not live long enough.
*/
