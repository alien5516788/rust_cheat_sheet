pub mod advanced;
pub mod basics;
pub mod flow_control;
pub mod miscellaneous;


fn main() {
    
    #[derive(Debug)]
    struct Person {
        age: i32,
    }
    
    let mut arr2 = [Person { age: 25 }, Person { age: 30 }, Person { age: 35 }];
    
    println!("{:?}", arr2);
    let s_mut: &mut [Person] = &mut arr2[1..3]; // [Person { age: 30 }, Person { age: 35 }]

    s_mut[0].age = 99;
    println!("{:?}", arr2);
   
}
