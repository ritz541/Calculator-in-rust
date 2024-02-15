use ::std::io;
fn main() {
    let input = user_input();
    let x = input.0;
    let y = input.1;

    operation(x, y);
}

fn user_input() -> (i32, i32) {
    println!("The first operand: ");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Falied to read input!");

    let first_number: i32 = input.trim().parse().expect("Enter a number!");

    let mut input = String::new();

    println!("Enter second operand: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Falied to read input!");

    let second_number: i32 = input.trim().parse().expect("Enter a number!");
    (first_number, second_number)
}

fn operation(x: i32, y: i32) {
    println!("1.Addition\n2.Subtraction\n3.Multiplication\n4.Division");
    println!("Enter your choice(1/2/3/4");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input!");

    let choice: isize = choice.trim().parse().expect("Enter a valid option");

    match choice {
        1 => println!("Addition of {} and {} is {}", x, y, addition(x, y)),
        2 => println!("Subtraction of {} and {} is {}", x, y, substraction(x, y)),
        3 => println!(
            "Multiplication of {} and {} is {}",
            x,
            y,
            multiplication(x, y)
        ),
        4 => {
            if y != 0 {
                println!("Division of {} and {} is {}", x, y, division(x, y));
            } else {
                println!("Cannot divide by zero");
            }
        }
        _ => println!("Invalid option: {}!", choice),
    }
}

fn addition(x: i32, y: i32) -> i32 {
    x + y
}

fn substraction(x: i32, y: i32) -> i32 {
    x - y
}

fn multiplication(x: i32, y: i32) -> i32 {
    x * y
}

fn division(x: i32, y: i32) -> f32 {
    let result = x as f32 / y as f32;
    result
}