
pub fn item() {
    println!("item");
}


pub mod user {

    pub fn user1() {
        println!("user1");
        some::some3();
    }

    fn user2() {
        println!("user2");
    }


    pub mod some {
        pub fn some3() {
            println!("some3");
        }
    }


}