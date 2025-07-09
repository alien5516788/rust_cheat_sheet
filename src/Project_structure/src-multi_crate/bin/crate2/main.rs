/* Define module named 'main' */
// 'main' is the root module of the binary crate (crate root),
// hence don't need to be declared

// library crate is directly accessible

fn main() {

    let pi : f32 = multi_crate::constants::pi;
    println!("{}", pi);

    let x : f32 = multi_crate::operations::addition::add(10.0, pi);
    println!("{}", x);

}
