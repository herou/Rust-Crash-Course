pub fn run(){
    let name = "Elio";
    let mut age = 37;
    println!("My name is {} and I am {}", name, age);
    age = 38;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 1;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (name, last_name, age) = ("Elio", "Prifti", 38);
    println!("Name: {}, Last Name: {}, Age: {}", name,last_name,age);


}