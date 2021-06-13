/* Primitive str = "Immutable fixed-length string somewhere in memory"
 String = Growable, heap-allocatd data structure = Use when you need to modify or own
*/

pub fn run () {
    // str by defaul
    let hello = "Hello";

    let mut name = String::from("Elio");

    println!("{}, {}, name length : {}", hello, name, name.len());

    // push char
    name.push(' ');
    name.push('w');


    //push string
    name.push_str(" Prifti");



    // Capacity in bytes
    println!("Capacity is:  {}", name.capacity());

    // Check if empty
    println!("Is empty: {}", name.is_empty());

    // Contains
    println!("Contains 'e': {}", hello.contains("e"));

    //Replace
    println!("Replace: {}", name.replace("o","0"));

    // Loop through string by whitespace
    for word in name.split_whitespace(){
        println!("{}",word)
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    println!("{}", name);

}