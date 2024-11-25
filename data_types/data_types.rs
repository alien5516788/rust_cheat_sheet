// primitive types // note completed !!!!  

// unit type
{
    (); /* A type similar to void. */

    let x : () = (); /* only the value can be assign to is also () . */
 }          
         
// never type
{

    ! /* Yes this is the symbol */ 
        /* Basically the program will not continue to execute,
        after encountering never type.
        Rarely used. */

    fn infinite_loop() -> ! { 
        /* Never returns the control back to caller 
        or evaluate to a type */
        loop { }
    }

}


// literals
{
    67; 
    "sdf";
}



// References and pointers
let mut name1 = String::from("My Name"); // ignore this
let mut name2 = String::from("Your Name"); // ignore this

// Reference
{   

    /* There can be mutiple immutable references, but only one mutable reference.
        There cannot be a mutable and an immutable refernce at the same time. */
    &name1;
    &mut name1;
    let ref1 : &String = &name1; // &Type : immutable reference
    let ref2 : &String = &name1; // &Type : immutable reference
    let mut_ref1 : &mut String = &mut name2; // &mut Type : mutable reference

    // Dereferncing refernces
    let ref_val : String = ref1; // get value
    let mut_ref_val : String = mut_ref1 ; // get mutable value

}

// Pointer
{

    /* There can be unlimited number of mutable and immutable pointers at the same time */ 
    &name1 as *const String;
    &mut name1 as *mut String;
    let ptr1 : *const String = &name1; // *const Type : immutable raw pointer
    let ptr2 : *const String = &name1; // *const Type : immutable raw pointer
    let mut_ptr1 : *mut String = &mut name1; // *mut Type : mutable raw pointer
    let mut_ptr2 : *mut String = &mut name1; // *mut Type : mutable raw pointer

    // Dereferencing pointers
    let ptr : *const String = ptr1; // get memory address
    let val : String = *ptr1; // get value
    let mut_ptr : *mut String = mut_ptr1; // get memory address
    let mut_val : String = *mut_ptr1; // get mutable value

}

// Function pointers
{    

    fn speak(name : String) -> () {
        println!("Hello {}", name);
    }

    // dereferencing
    speak as fn (String); // memory address

    let speak_ptr : fn (String) = speak; // memory address

    let speak_ptr : fn (String);
    speak_ptr = speak; // memory address

    let speak_ptr : fn (String);
    speak_ptr = speak as fn (String); // memory address

    // calling
    speak_ptr(String::from("Name"));

}
