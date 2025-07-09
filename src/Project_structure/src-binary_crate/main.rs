mod helper;
mod utils;

fn main() {

    helper::shoot();
    helper::special::special_strike();
    helper::special::strikes::air_strike();
    // helper::special::intelligence::spy_data(); won't run because module intelligence is private

    utils::drive();
    utils::destroyer::destroy();
    utils::vehicals::truck();
    utils::vehicals::combat_vehicals::tank();

}
