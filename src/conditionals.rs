// Conditionals - Used to check the condition

pub fn run (){
    let age: u8 = 19;
    let check_id = false;

    // if/else
    if age >= 18  && check_id{
        println!("Get driver license!");
    } else if age >= 18 || check_id{
        println!("Become 18!")
    }else{
        println!("Nothing to do!");
    }

    // Shorthand If
    let isMature = if age >= 18 { true } else { false} ; 
    println!("Is Mature: {}",isMature);
}