// Enums
// =====

/*
    Enums defines types that have fixed set of type variants

    They are like containers for multiple struct types
*/

fn enums() {
    enum GameState {
        Play,             // unit variant
        Paused(i32, i32), // tuple variant
        Resume,
        Exit { save: bool, location: String }, // named field variant
    }

    let mut _state = GameState::Play;

    _state = GameState::Paused(10, 25);

    _state = GameState::Exit {
        save: true,
        location: String::from("home"),
    };

    // Destructuring via pattern matching
    // All possible variants should be matched
    match &_state {
        GameState::Play => println!("play"),
        GameState::Paused(begin, _) => println!("paused at {} min", begin),
        GameState::Resume => println!("resume"),
        GameState::Exit { save, location } => {
            println!("exit with save: {} and location: {}", save, location)
        }
    }

    // Destructuring via if let
    // Unnecessary variants can be ignored
    if let GameState::Paused(begin, _) = _state {
        println!("paused at {} min", begin);
    } else if let GameState::Exit { save, location } = _state {
        println!("exit with save: {} and location: {}", save, location);
    } else {
        println!("other state");
    }
}
