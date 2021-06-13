// Structs - Used to create custom data types

//Struct Example
/* (struct Color{
    red: u8,
    green: u8,
    blue: u8
}
*/

struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct peson
    fn new(first: &str, last: &str) -> Person {
        Person{
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    fn get_full_name(&self) -> String{
        format!("{} {}",self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, new_last_name: &str){
        self.last_name = new_last_name.to_string();
    }
}

// Tuple structs
// struct Color(u8,u8,u8);

pub fn run () {
    /* let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    }; 

    c.green = 200; 

    println!("Color: {} {} {}", c.red, c.green, c.blue); 

    let mut c = Color(255,200,0);
    c.1 = 199; 

    println!("Color: {} {} {}", c.0, c.1, c.2); */

    let mut p = Person::new("elio","prifti");
    p.set_last_name("PRIFTI");
    println!("Person : {}",p.get_full_name());

}