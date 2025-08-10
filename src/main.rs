//use core::num;
use std::io::{self, Write};

fn read_line(mut number_01: String, mut number_02: String, mut operation: String) {

    print!("Enter first number: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut number_01)
        .expect("Faild to read input for first number");

    print!("Enter the operation( * , + /,- ): ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut operation)
        .expect("Faild to read input for operation");
    operation = operation.trim().to_string(); // remove all new lines `\n`

    print!("Enter the second Number :");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut number_02)
        .expect("Fail to read input for second number");
    
}



fn multiplication(number_01: i32, number_02: i32) -> i32 {
    number_01 * number_02
}
fn addition(number_01: i32, number_02: i32) -> i32 {
    number_01 + number_02
}
fn subraction(number_01: i32, number_02: i32) -> i32 {
    number_01 - number_02
}
fn devison(number_01: i32, number_02: i32) -> i32 {
    number_01 / number_02
}
fn main() {
    let mut number_02 = String::new();
    let mut number_01 = String::new();
    let mut operation: String = String::new();
    let mut answer: i32;

    //parsing the strings to i32
    let number_01: i32 = number_01.trim().parse().unwrap();
    let number_02: i32 = number_02.trim().parse().unwrap();

    //check the opereation
    if operation == "*" {
        println!("You have chosen mutilplication");
        answer = multiplication(number_01, number_02);
    } else if operation == "+" {
        println!("You have chossen addition ");
        answer = addition(number_01, number_02);
    } else if operation == "-" {
        print!("You have chossen subraction");
        answer = subraction(number_01, number_02);
    } else if operation == "/" {
        print!("You have chosen devision");
        answer = devison(number_01, number_02);
    } else {
        println!("You have chosen an operation that is not supported yet.");
        return;
    };

    println!("The answer is: {}", answer);
}
