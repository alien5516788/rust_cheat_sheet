fn main() {

    let pi : f32 = single_crate::constants::pi;
    println!("{}", pi);

    let x : f32 = single_crate::operations::addition::add(11.0, pi);
    println!("{}", x);

}
