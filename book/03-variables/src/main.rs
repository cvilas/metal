fn main() {
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    let tup = ("char", 5, 6.8);
    let heart_eyed_cat = '😻';
    println!("{} {} {} {} {}", heart_eyed_cat, tup.0, tup.1, tup.2, y);
    
    //if
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
    
    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // for
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
    
    // range
     for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

}
