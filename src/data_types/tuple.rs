fn tuple() {
    // Heterogeneous, fixed size
 
    let tup : (i32, char, (i32, i32)) = (2, 'h', (2, 5));
    
    // Destructuring
    let (elm0, elm1, (elm2, elm3)) : (i32, char, (i32, i32)) = (2, 'h', (2, 5));
    
    // Accessing elements
    let elm = elm1;
    let elem = tup.2.0;

}
