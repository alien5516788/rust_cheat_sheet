/* Define module named 'helper' */

pub fn shoot() {
    println!("Shooting from guns");
}

pub mod special {
/* Declare and Define module named 'special' */

    pub fn special_strike() {
        println!("Special strike using intelligence");
        intelligence::radar_data();
    }

    pub mod strikes {
    /* Declare and Define module named 'strikes' */

        pub fn air_strike() {
            println!("Attacking from sky");
        }

        pub fn boat_strike() {
            println!("Attacking from waters");
        }

    }

    mod intelligence {
    /* Declare and Define module named 'intelligence' */
    // This module is private and only accessible to items within 'special' module

        pub fn radar_data() {
            println!("Giving radar data: 27384273");
        }

        pub fn spy_data() {
            println!("Giving spy data: north south");
        }

    }

}
