#![allow(warnings)]

pub mod advanced;
pub mod basics;
pub mod flow_control;
pub mod miscellaneous;

fn main() {
    
    let x = String::from("hello");

    fn shortest<'a>(x: &'a String, y: &'a String) -> &'a String {
        if x.len() < y.len() {
            x
        } else {
            y
        }
    }

    let result;

    {
        let y = String::from("world");
        result = shortest(&x, &y);
    }
    
    // println!("{}", result);
    
}
