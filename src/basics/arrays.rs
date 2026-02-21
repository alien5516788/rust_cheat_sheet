fn array() {
    // Homogeneous, fixed size
    let _arr: [[i32; 2]; 3] = [[2, 5], [6, 1], [8, 1]];

    // Short hand
    let _arr = [[2; 2]; 3]; // Short hand

    // Destructuring
    let [[_elm0, _elm1], [_elm2, _elm3], [_elm4, _elm5]] = _arr;

    // Accessing elements
    let _elm = _elm4;
    let _elm = _arr[2][0];
}
