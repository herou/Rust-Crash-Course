pub fn run (){
    greeting("Hello", "Elio");
    println!("Add function result: {} ", add(5,6));
}

fn greeting(greet: &str, name: &str) {
    println!("{}, nice to meet you {}!", greet, name);
}

fn add(value1: i32, value2: i32) -> i32 {
    return value1 + value2;
}