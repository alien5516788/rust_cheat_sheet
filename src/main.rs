pub mod basics;
pub mod flow_control;
pub mod advanced;
pub mod miscellaneous;
pub mod modularity;

fn main() {
    
    trait MyIterator<T> {
        fn next(&mut self) -> i32;
    }

    
    struct Counter;
    
    impl<U> MyIterator<U> for Counter {
        fn next(&mut self) -> i32 {
            0
        }
    }

    
}
