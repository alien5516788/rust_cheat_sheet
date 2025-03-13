/* Define module named 'utils' */

pub mod vehicals; /* Declare module named 'vehicals' */

pub fn drive() {
    println!("Drive vehicals");
}

pub mod destroyer {
/* Declare and Define module named 'destroyer' */

    pub fn destroy() {
        println!("Destroy vehicals");
    }

    pub fn dissemble() {
        println!("Dissemble vehicals");
    }

}
