// heterogeneous, fixed size
(2, 'h', (2, 5));

let tup = (2, 'h', (2, 5));

let tup : (i32, char, (i32, i32)) = (2, 'h', (2, 5));

let (elm0, elm1, (elm2, elm3)) : (i32, char, (i32, i32)) = (2, 'h', (2, 5));

// accessing elements
let elm = i2;

let elem = tup.2.0;