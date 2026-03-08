fn for_loops() {
    /*
       - Repeatedly run a logical block of code by iterating over a range or collection
       - For loop is mostly a sugar for `Iterator` trait
    */

    // Example 1
    // Iterate over a range
    for i in 1..10 {
        println!("{}", i);
    }

    // Example 2
    // Iterate over an array
    let arr = ['1', '4', '7', '2', '9'];

    for i in arr {
        println!("{}", i);
    }

    /*
        - The type of `i` is the type of the elements in the collection being iterated over
    */

    // Example 1
    let vec = [1, 4, 7, 2, 9];

    for i in vec {
        println!("{}", i); // value copied to `i`, because array elements are `Copy`
    }

    println!("{}", vec[1]);

    // Example 2
    let vec = Vec::from([1, 4, 7, 2, 9]);

    for i in vec {
        println!("{}", i); // value moved to `i`, because `Vec` elements are not `Copy`
    }

    // println!("{}", vec[1]); // error

    // Example 3
    let vec = Vec::from([1, 4, 7, 2, 9]);

    for i in &vec {
        println!("{}", i); // value borrowed
    }

    println!("{}", vec[1]);
}
