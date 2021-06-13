/*
Tuples group together values if different types
Max 12 elements
*/

pub fn run () {
    let person: (&str, &str, i32) = ("Elio", "Prifti", 38);

    println!("FirstName: {}, Lastname: {}, Age: {}", person.0,person.1,person.2);
}