fn array() {
    // Homogeneous, fixed size

    let arr1: [[i32; 2]; 3] = [[2, 5], [6, 1], [8, 1]]; // Normal
    let arr2: [[i32; 2]; 3] = [[2; 2]; 3]; // Short hand

    // Destructuring
    let [[elm0, elm1], [elm2, elm3], [elm4, elm5]] = arr1;

    // Accessing elements
    let elm = elm4;
    let elm = arr1[2][0];
}
