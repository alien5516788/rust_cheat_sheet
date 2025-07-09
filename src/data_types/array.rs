// homogeneous, fixed size
[[2, 5], [6, 1], [8, 1]];

let arr = [[2, 5], [6, 1], [8, 1]];

let arr : [[i32; 2]; 3] = [[2, 5], [6, 1], [8, 1]];

let [[elm0, elm1], [elm2, elm3], [elm4, elm5]] : [[i32; 2]; 3] = [[2, 5], [6, 1], [8, 1]];

// array short hand
[[2; 2]; 3];

let arr = [[2; 2]; 3];

let arr : [[i32; 2]; 3] = [[2; 2]; 3];

let [[elm0, elm1], [elm2, elm3], [elm4, elm5]] : [[i32; 2]; 3] = [[2; 2]; 3];

// accessing elements
let elm = elm4;
let elm = arr[2][0];
