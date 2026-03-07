fn tuple() {
    /*
        - Heterogeneous
        - Fixed size
    */
    let tup: (i32, char, (i32, f32)) = (2, 'h', (2, 5.5));

    // Destructuring
    let (_elm0, _elm1, (_elm2, _elm3)) = tup;

    // Accessing elements
    let _elm = _elm1;
    let _elem = tup.2 .0;
}

fn arrays() {
    /*
        - Homogeneous
        - Fixed size
    */
    let _arr: [[i32; 2]; 3] = [[2, 5], [6, 1], [8, 1]];

    // Short hand
    let _arr = [[2; 2]; 3];

    // Destructuring
    let [[_elm0, _elm1], [_elm2, _elm3], [_elm4, _elm5]] = _arr;

    // Accessing elements
    let _elm = _elm4;
    let _elm = _arr[2][0];
}

fn slice_type() {
    /*
        - A slice is a view into a contiguous (being contiguous is important) data (`[Type]`)
        - It does not own the data, only borrows it
        - More accurately is a representation of and exisiting data
        - Slice just tells the contiguous data is this type (`[T]`)
        - The slice type `[T]` is dynamically sized and must be always behind
            a pointer type (e.g. `&[T]`, `Box<[T]>` etc)
        - The pointer type holds the memory address of the first element and the length of the slice
            as a fat pointer
    */

    // Example 1
    let arr1 = [10, 20, 30, 40, 50];

    let _s1: &[i32] = &arr1[..]; // whole slice
    let _s2: &[i32] = &arr1[1..4]; // [20, 30, 40]
    let _s3: &[i32] = &arr1[2..]; // [30, 40, 50]
    let _s4: Box<[i32]> = arr1[..2].to_vec().into_boxed_slice(); // [10, 20]

    // Example 2
    struct Person {
        age: i32,
    }
    let mut arr2 = [Person { age: 25 }, Person { age: 30 }, Person { age: 35 }];

    let s_mut: &mut [Person] = &mut arr2[1..3]; // [Person { age: 30 }, Person { age: 35 }]
    s_mut[0].age = 99;
}
