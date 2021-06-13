/*Vectors - Resizable arrays*/

pub fn run (){
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];

    println!("Numbers: {:?}", numbers);

    // We ca nRe-assign value but we CAN NOT add values
    numbers[1]= 20; 
    println!("Numbers: {:?}", numbers);

    // Add new values
    numbers.push(101);
    numbers.push(102);
    println!("Numbers: {:?}", numbers);


    //Pop out the last value
    numbers.pop();
    println!("Numbers: {:?}", numbers);

    // Get value based on index of array
    println!("First element: {}", numbers[0]);

    // Get length of vec
    println!("Length of: {}", numbers.len());

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    
    println!("Slice: {:?}", slice);

    // Loop vector
    for x in numbers.iter() {
        println!("Numbers: {}", x);
    }

      // Loop vector and mutate
      for x in numbers.iter_mut() {
           *x *=  2;
        println!("Numbers: {}", x);
    }
}