pub fn shoot() {
    println!("Shooting from guns");
}

pub mod special {

    pub fn special_strike() {
        println!("Special strike using intelligence");
        intelligence::radar_data();
    }

    pub mod strikes {

        pub fn air_strike() {
            println!("Attacking from sky");
        }

        pub fn boat_strike() {
            println!("Attacking from waters");
        }

    }

    mod intelligence {

        pub fn radar_data() {
            println!("Giving radar data: 27384273");
        }

        pub fn spy_data() {
            println!("Giving spy data: north south");
        }

    }

}
