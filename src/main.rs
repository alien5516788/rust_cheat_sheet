#![allow(warnings)]

pub mod advanced;
pub mod basics;
pub mod flow_control;
pub mod miscellaneous;

fn main() {
    
    // Required method
    pub trait Clone2: Sized {
        // Required method
        fn clone1(&self) -> Self;
        
        fn clone2(self: &Self) -> Self;
        
        
    
        // Provided method
        fn clone_from(&mut self, source: &Self);
    } 
    
    
}
