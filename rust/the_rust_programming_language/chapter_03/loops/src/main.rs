fn main() {
    // Loop
    let mut counter = 0;
    // Return the value of the last expression into a variable
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // While
    // Conditional loop
    let mut while_counter = 10;
    while while_counter != 0 {
        println!("{while_counter}!");
        while_counter -= 1;
    }
    println!("LIFTOFF!!!");

    // For
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // For loop
    for element in a.iter() {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
}
