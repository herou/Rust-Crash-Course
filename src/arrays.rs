/*Fixed size list where elements are the same data types*/

pub fn run (){
    let mut numbers: [i32; 5] = [1,2,3,4,5];

    println!("Numbers: {:?}", numbers);

    // We ca nRe-assign value but we CAN NOT add values
    numbers[1]= 20; 
    println!("Numbers: {:?}", numbers);

    // Get value based on index of array
    println!("First element: {}", numbers[0]);

    // Get length of array
    println!("Length of: {}", numbers.len());

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    
    println!("Slice: {:?}", slice);
}