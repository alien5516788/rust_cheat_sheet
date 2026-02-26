/*
    Fact:
        - Closures are actually structs with function pointers.
        - Fn, FnMut, FnOnce are special traits implemented by compiler.
           Their trait bound syntax is different and made to look like a function signature.
        -Fn(i32, i32) -> i32 is a syntax sugar for Fn<(i32, i32), Output = i32>
*/

// TODO
fn closure_syntax() {
    // When we write this
    let x = 5;
    let add_x = |y: i32| y + x;

    // It internelly converted to something like this
    /*
        struct __Closure {
            x: i32,
        }

        impl Fn<(i32), Output = i32> for __Closure {
            fn call(&self, y: i32) -> i32 {
                self.x + y
            }
        }
    */
}
