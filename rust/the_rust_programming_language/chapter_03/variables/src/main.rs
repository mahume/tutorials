use std::io;

fn main() {
    // IMMUTABLE VS MUTABLE
    // Change the value of a variable
    let mut mutable_variable = 5;
    println!("The value of 'mutable_variable' is: {mutable_variable}");
    mutable_variable = 6;
    println!("The value of mutable_variable is: {mutable_variable}");

    // SHADOWING
    // Overwrite a variable's value with a new one
    // This allows transforming the variable's value without making it mutable
    let shadowed_variable = 5;
    let shadowed_variable = shadowed_variable + 1;
    let shadowed_variable = shadowed_variable * 2;
    println!("The value of 'shadowed_variable' is: {shadowed_variable}");
    // We can also change the type of the variable when shadowing
    let string_to_int = "   ";
    let string_to_int = string_to_int.len();
    println!("The value of 'string_to_int' is: {string_to_int}");

    // PRIMITIVE TYPES
    // Scalar Types
    // Integers: Signed & Unsigned
    // Floating-Point: f32 & f64
    // Boolean: true & false
    // Character: 'a' & 'α'

    // Compound Types
    // Tuples
    let tuple: (u8, f32, i32) = (100, 6.4, -1);
    // Destructuring a tuple
    let (integer, floating_point, signed_integer) = tuple;
    println!("The value of 'integer' is: {integer}");

    // Indexing a tuple
    let one_hundred = tuple.0;
    let six_point_four = tuple.1;
    let negative_one = tuple.2;

    // Arrays
    // Allocated on the stack
    // :[same_type; fixed_length]
    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    enter_array_index();
}

fn enter_array_index() {
    let range = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Not a number! Please try again!");

    let element = range[index];

    println!("The value of the element at index {index} is: {element}");
}
