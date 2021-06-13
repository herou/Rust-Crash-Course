pub fn run (){
    // Print to console
    println!("Hello from the print.rs file");

    // Basic Formatting 
    println!("{} plus {} is equal to : {}", "One" , 2, 3);

    // Positional Aruments
    println!("{0} + {1} + {2} = {3}", "Zero","One","Two",3);

    // Named Arguments
    println!(
        "{name} likes to play {sport}",
         name = "Elio", sport = "Soccer"
        );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10 , 10 ,10 );

    // Placeholder for debug trait
    println!("{:?}", ("elio", "prifti", true));
}