use std::io;
use std::io::Write;

fn do_calculation(num_1: f64, op: char, num_2: f64) -> Result<f64, &'static str> {
    let result = match op {
        '+' => num_1 + num_2,
        '-' => num_1 - num_2,
        '*' => num_1 * num_2,
        '/' => if num_2 != 0.0 { num_1 / num_2 } else { return Err("Division by zero") },
        _ => return Err("Invalid operator"),
    };

    Ok(result)
}

fn evaluate_rpn(expr: &str) -> Result<f64, &'static str> {
    let mut stack: Vec<f64> = Vec::new();

    for token in expr.split_whitespace() {
        match token {
            "+" | "-" | "*" | "/" => {
                if stack.len() < 2 {
                    return Err("Invalid expression");
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                let result = match token {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => if b != 0.0 { a / b } else { return Err("Division by zero") },
                    _ => unreachable!(),
                };
                stack.push(result);
            }
            num => match num.parse::<f64>() {
                Ok(n) => stack.push(n),
                Err(_) => return Err("Invalid number"),
            },
        }
    }

    if stack.len() != 1 {
        return Err("Invalid expression");
    }

    Ok(stack[0])
}

fn main() {
    loop {
        print!("Hello! Please select operational mode: \n");
        print!("\t1: Default Calculator \n");
        print!("\t2: RPN Calculator \n");
        print!("\tq: Quit \n");

        let mut choice = String::new();

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => default_calculator(),
            "2" => rpn_calculator(),
            "q" => {
                println!("Goodbye!");
                break;
            },
            _ => println!("Invalid choice"),
        }
    }
}

fn default_calculator() {
    let mut current = 0.0;

    println!("Basic Calculator Mode");
    println!("Current value: {}", current);
    println!("Available commands:");
    println!("\t$number: Set current value");
    println!("\t('+', '-', '/', '*') $number: Perform operation");
    println!("\tq: Return to main menu");

    loop {
        let mut input = String::new();

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "q" {
            println!("\n");
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        match parts.as_slice() {
            [num] => {
                match num.parse::<f64>() {
                    Ok(n) => {
                        current = n;
                        println!("Current value: {}", current);
                    }
                    Err(_) => println!("Invalid number"),
                }
            }
            [op, num] => {
                match num.parse::<f64>() {
                    Ok(n) => {
                        match do_calculation(current, op.chars().next().unwrap(), n) {
                            Ok(result) => {
                                println!("Result: {}", result);
                                current = result;
                            },
                            Err(e) => println!("Error: {}", e),
                        }
                    }
                    Err(_) => println!("Invalid number"),
                }
            }
            _ => println!("Invalid input"),
        }
    }
}

fn rpn_calculator() {
    println!("RPN Calculator Mode");
    println!("Enter expression in RPN format (e.g., '3 − 4 + 5' becomes '3 4 − 5 +')");
    println!("Enter 'q' to return to main menu");

    loop {
        let mut input = String::new();

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input == "q" {
            println!("\n");
            break;
        }

        match evaluate_rpn(input) {
            Ok(result) => println!("Result: {}", result),
            Err(e) => println!("Error: {}", e),
        }
    }
}