//Reference Pointers = Point to a resource in memory

pub fn run (){
    // Primitive arrayslet
    let array1 = [1,2,3];
    let array2 = array1;

    println!("Arrays: {:?}", (array1, array2));

    /* With non-primites (ex: Vector), if you assing another variable 
    to a piece of data, the first variable will no lonber
    hold that value. You wil need to use a reference (&) to point
    to the resource
    */
    

    // Vector
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    println!("Arrays: {:?}", (&vec1, vec2));
}