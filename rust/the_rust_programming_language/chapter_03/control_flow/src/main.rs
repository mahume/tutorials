fn main() {
    let number = 6;

    // Each condition is called an arm
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Can assign the result of an if expression to a variable
    // But, the type of the variable must be the same type for all arms
    let condition = true;
    let value_from_if_expression = if condition {
        5
    } else {
        6
    };
    println!("The value of value_from_if_expression is: {value_from_if_expression}");

}
