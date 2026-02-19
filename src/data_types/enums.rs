fn enums() {
    // declaration
    enum GameState {
        Play,
        Paused(i32),
        Resume(i32, i32),
        Setting { particles: i64, refresh_rate: f32 }, // only the way to extract input data is through pattern matching
    }

    // instantiating
    GameState::Play;

    let state = GameState::Setting {
        particles: 30,
        refresh_rate: 60.0f32,
    };

    let state: GameState = GameState::Paused(34);

    // extracting values via pattern matching
    let state = GameState::Paused(34);

    match state {
        // matching should be done to all the enum elements
        GameState::Play => println!("play"),
        GameState::Paused(time) => println!("puased {} min", time),
        GameState::Resume(begin, i32) => println!("resume at {} min end at any min", begin),
        GameState::Setting {
            particles: i64,
            refresh_rate: f32,
        } => println!("Settings"),
    }

    // If let
    // Used to match enums
    // Doesn't need to be implemented for all elements
    enum Coin {
        Penny,
        Pound(char), // S - Silver G - Gold
        Cent,
    }

    let coin: Coin = Coin::Pound('G');

    if let Coin::Pound(material) = coin {
        println!("Coin is {} Pound", material);
    } else if let Coin::Penny = coin {
        println!("Coin is penny");
    } else {
        println!("Not a Pound or Penny");
    }

    // note: Most of the time enums are used with pattern matching
}
