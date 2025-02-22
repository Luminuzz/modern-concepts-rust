use rpn_calculator::RPNCalculator;
use std::io;

// Main function for executing the RPN calculator
fn main() {
    let mut calc = RPNCalculator::new();
    RPNCalculator::welcome_prompt();
    let mut input = String::new();

    // "Main loop", repeating logic for each input
    loop {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("Exiting RPN Calculator...");
            println!("Your infix calculation is: {}", calc.clone().reconstruct_expression_infix());
            println!("Your LaTeX calculation is: {}", calc.clone().reconstruct_expression_latex());

            match calc.get_result() {
                Some(value) => println!("The final result is: {}", value),
                None => println!("No result available."),
            }
            break;
        }

        calc.apply_operation(input);

        match input {
            "+" | "-" | "*" | "/" | "^" | "sqrt" | "log" | "abs" | "++" | "**" | "!" => {
                if let Some(value) = calc.get_result() {
                    println!("The current result is: {}", value);
                }
            }
            _ => {}
        }
    }
}
