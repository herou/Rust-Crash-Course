// Loops untill conditions is met

pub fn run () {
    let mut count = 0;

    /*loop{
        count += 1;
        println!("Values: {}",count);

        if count == 20 {
            break;
        }
    }*/

  
    while count <= 100{
        if count % 2 == 0{
            println!("Even: {}",count);
        } else{
            println!("Odd: {}",count);
        }

        count += 1;
    }
}