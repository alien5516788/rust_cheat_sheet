pub mod advanced;
pub mod basics;
pub mod flow_control;
pub mod miscellaneous;


fn main() {
    
    struct Person {
        age: i32,
    }
    
    impl Drop for Person {
        fn drop(&mut self) {
            println!("Dropping Person with age: {}", self.age);
        }
    }
    
    {
        let p = Person { age: 25 };
    }
    

   
}
