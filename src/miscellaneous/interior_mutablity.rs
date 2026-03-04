use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, Mutex};
use std::thread;

/// 1. THE "INTERIOR MUTABILITY" PATTERN
/// Purpose: Mutate data through an immutable reference (&T).
/// Niche Point: It moves "Borrow Checking" from Compile-time to Runtime.
struct Robot {
    // We use RefCell because we want to change 'battery' even if
    // the Robot instance itself is immutable.
    battery: RefCell<u8>,
}

fn main() {
    // --- SINGLE THREADED AREA ---

    // 2. Rc (Reference Counted)
    // Purpose: Shared ownership. Multiple variables "own" the same heap memory.
    // Niche Point: Rc::clone() only increments a counter; it doesn't copy the data.
    //
    let shared_data = Rc::new(RefCell::new(100));

    {
        let clone_a = Rc::clone(&shared_data);
        // Interior Mutability: We change the value through an immutable clone!
        *clone_a.borrow_mut() -= 10;

        // Niche Point: If you call .borrow_mut() twice in the same scope,
        // it will PANIC (Crash). This is the "Runtime Borrow Checker".
    } // clone_a goes out of scope here.
      // Internally: The 'Drop' trait is called, decrementing the Rc counter.

    // 3. WHY WE CAN'T SEND Rc TO THREADS
    // Niche Point: Rc is !Send because its counter is a non-atomic integer.
    // If two threads updated it at once, the count would break (Data Race).

    // --- MULTI-THREADED AREA ---

    // 4. Arc (Atomic Reference Counted)
    // Purpose: The thread-safe version of Rc.
    // 5. Mutex (Mutual Exclusion)
    // Purpose: The thread-safe version of RefCell.
    //
    let safe_shared_data = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&safe_shared_data);
        let handle = thread::spawn(move || {
            // .lock() blocks the thread until the data is available.
            // Niche Point: Unlike RefCell (which panics), Mutex makes you WAIT.
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *safe_shared_data.lock().unwrap());
}

/*
  FINAL RECAP TABLE (The "Hidden" Logic):

  | Type      | Logic Location | Thread Safe? | Failure Mode        |
  |-----------|----------------|--------------|---------------------|
  | &mut T    | Compile-time   | Yes (Unique) | Compiler Error      |
  | RefCell   | Runtime        | NO           | Thread Panic        |
  | Mutex     | Runtime        | YES          | Thread Sleep/Block  |
  | Rc        | Heap Counter   | NO           | Counter Corruption  |
  | Arc       | Atomic Counter | YES          | (Slower performance)|

  NICHE GOTCHA:
  Memory Leaks! If Rc A points to Rc B, and Rc B points to Rc A,
  the counter never hits 0. They stay in memory forever.
  To fix this, use 'Weak<T>' pointers.
*/
