// Enums are types which have a few definitive values

enum State{
    // Variants
    Loading,
    Show,
    Error
}

fn change_state(state: State){
    match state {
        State::Loading => println!("Loading..."),
        State::Show => println!("Show"),
        State::Error => println!("Error")
    }
}

pub fn run () {
    let st1 = State::Loading;
    let st2 = State::Show;
    let st3 = State::Error;

    change_state(st1);
    change_state(st2);
    change_state(st3);
}