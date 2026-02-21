fn tuple() {
    // Heterogeneous, fixed size
    let tup: (i32, char, (i32, f32)) = (2, 'h', (2, 5.5));

    // Destructuring
    let (_elm0, _elm1, (_elm2, _elm3)) = tup;

    // Accessing elements
    let _elm = _elm1;
    let _elem = tup.2 .0;
}
