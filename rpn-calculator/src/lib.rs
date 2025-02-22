// Library for handling user inputs
//use std::io;

// Class declaration, clone trait
#[derive(Clone)]
pub struct RPNCalculator {
    stack: Vec<f64>,
    history_stack: Vec<String>,
}

impl RPNCalculator {
    // Constructor, initializing the vectors
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            history_stack: Vec::new(),
        }
    }

    // Applying operation to the stack
    pub fn apply_operation(&mut self, token: &str) {
        self.history_stack.push(token.to_owned());
        match token {
            "+" | "-" | "*" | "/" | "^" => {
                self.arithmetical_operation_handling(token);
            }
            "sqrt" | "log" | "abs" => {
                self.log_abs_sqrt_operation_handling(token);
            }
            "!" => {
                self.factorial_operation_handling();
            }
            "++" => {
                self.full_stack_addition_handling();
            }
            "**" => {
                self.full_stack_multiplication_handling();
            }
            _ => {
                match token.parse::<f64>() {
                    Ok(_) => self.new_number_handling(token),
                    Err(_) => {
                        println!("Invalid input '{}'", token);
                        self.history_stack.pop();
                    }
            }
        }
        }
    }

    pub fn arithmetical_operation_handling(&mut self, token: &str) {
        let b = self.stack.pop().unwrap();
        let a = self.stack.pop().unwrap();
        let result = match token {
            "+" => a + b,
            "-" => a - b,
            "*" => a * b,
            "/" => a / b,
            "^" => a.powf(b),
            _ => unreachable!(),
        };
        self.stack.push(result);
    }

    pub fn log_abs_sqrt_operation_handling(&mut self, token: &str) {
        let a: f64 = self.stack.pop().unwrap();
        let result = match token {
            "sqrt" => a.sqrt(),
            "log" => a.log10(),
            "abs" => a.abs(),
            _ => unreachable!(),
        };

        self.stack.push(result);
    }

    pub fn factorial_operation_handling(&mut self) {
        let a = self.stack.pop().unwrap();
        let result = (1..=a as u64).product::<u64>() as f64;

        self.stack.push(result);
    }

    pub fn full_stack_addition_handling(&mut self) {
        let result: f64 = self.stack.iter().sum();

        self.stack.clear();
        self.stack.push(result);
    }

    pub fn full_stack_multiplication_handling(&mut self) {
        let result: f64 = self.stack.iter().product();

        self.stack.clear();
        self.stack.push(result);
    }

    pub fn new_number_handling(&mut self, token: &str) {
        if let Ok(num) = token.parse::<f64>() {
            self.stack.push(num);
        }
    }

    pub fn welcome_prompt() {
        println!("-----------------------------");
        println!("Welcome to the RPN calculator, please input your equation with 'enter' between!");
        println!("Type 'exit' to quit.");
        println!("-----------------------------");
    }

    pub fn get_result(&self) -> Option<f64> {
        if self.stack.len() == 1{
            self.stack.last().cloned()
        }else{
            None
        }

    }

    pub fn reconstruct_expression_infix(&mut self) -> String {
        if let Some(token) = self.history_stack.pop() {
            match token.as_str() {
                "+" | "-" | "*" | "/" | "^" => {
                    let right = self.reconstruct_expression_infix();
                    let left = self.reconstruct_expression_infix();
                    format!("({} {} {})", left, token, right)
                }
                "++" => {
                    let mut terms = Vec::new();
                    while !self.history_stack.is_empty() {
                        terms.push(self.reconstruct_expression_infix());
                    }
                    format!("({})", terms.join(" + "))
                }
                "**" => {
                    let mut terms = Vec::new();
                    while !self.history_stack.is_empty() {
                        terms.push(self.reconstruct_expression_infix());
                    }
                    format!("({})", terms.join(" * "))
                }
                "!" => {
                    let operand = self.reconstruct_expression_infix();
                    format!("({}!)", operand)
                }
                "abs" => {
                    let operand = self.reconstruct_expression_infix();
                    format!("abs({})", operand)
                }
                "sqrt" => {
                    let operand = self.reconstruct_expression_infix();
                    format!("sqrt({})", operand)
                }
                "log" => {
                    let operand = self.reconstruct_expression_infix();
                    format!("log10({})", operand)
                }
                _ => token, // numbers
            }
        } else {
            String::new() // empty stack
        }
    }

    pub fn reconstruct_expression_latex(&mut self) -> String {
        if let Some(token) = self.history_stack.pop() {
            match token.as_str() {
                "+" | "-" => {
                    let right = self.reconstruct_expression_latex();
                    let left = self.reconstruct_expression_latex();
                    format!("{{{} {} {}}}", left, token, right)
                }
                "*" => {
                    let right = self.reconstruct_expression_latex();
                    let left = self.reconstruct_expression_latex();
                    format!("{{{} \\cdot {}}}", left, right)
                }
                "/" => {
                    let right = self.reconstruct_expression_latex();
                    let left = self.reconstruct_expression_latex();
                    format!("{{\\frac{{{}}}{{{}}}}}", left, right)
                }
                "^" => {
                    let right = self.reconstruct_expression_latex();
                    let left = self.reconstruct_expression_latex();
                    format!("{{{}^{{{}}}}}", left, right)
                }
                "++" => {
                    let mut terms = Vec::new();
                    while !self.history_stack.is_empty() {
                        terms.push(self.reconstruct_expression_latex());
                    }
                    format!("{{{}}}", terms.join(" + "))
                }
                "**" => {
                    let mut terms = Vec::new();
                    while !self.history_stack.is_empty() {
                        terms.push(self.reconstruct_expression_latex());
                    }
                    format!("{{{}}}", terms.join(" \\cdot "))
                }
                "!" => {
                    let operand = self.reconstruct_expression_latex();
                    format!("{{{}}}!", operand)
                }
                "abs" => {
                    let operand = self.reconstruct_expression_latex();
                    format!(r"\left| {{{}}} \right|", operand)
                }
                "sqrt" => {
                    let operand = self.reconstruct_expression_latex();
                    format!(r"\sqrt{{{}}}", operand)
                }
                "log" => {
                    let operand = self.reconstruct_expression_latex();
                    format!(r"\log_{{10}} {{{}}}", operand)
                }
                _ => token, // numbers
            }
        } else {
            String::new() // if the stack is empty
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_number() {
        let mut calc = RPNCalculator::new();
        calc.apply_operation("42");
        // Da nur eine Zahl auf dem Stack liegt, sollte get_result() Some(42.0) liefern.
        assert_eq!(calc.get_result(), Some(42.0));
    }

    #[test]
    fn test_arithmetical_addition() {
        let mut calc = RPNCalculator::new();
        calc.apply_operation("3");
        calc.apply_operation("7");
        calc.apply_operation("+");
        assert_eq!(calc.get_result(), Some(10.0));
    }

    #[test]
    fn test_arithmetical_subtraction() {
        let mut calc = RPNCalculator::new();
        calc.apply_operation("10");
        calc.apply_operation("4");
        calc.apply_operation("-");
        assert_eq!(calc.get_result(), Some(6.0));
    }

    #[test]
    fn test_arithmetical_multiplication() {
        let mut calc = RPNCalculator::new();
        calc.apply_operation("5");
        calc.apply_operation("6");
        calc.apply_operation("*");
        assert_eq!(calc.get_result(), Some(30.0));
    }

    #[test]
    fn test_arithmetical_division() {
        let mut calc = RPNCalculator::new();
        calc.apply_operation("20");
        calc.apply_operation("4");
        calc.apply_operation("/");
        assert_eq!(calc.get_result(), Some(5.0));
    }

    #[test]
    fn test_arithmetical_exponentiation() {
        let mut calc = RPNCalculator::new();
        calc.apply_operation("2");
        calc.apply_operation("5");
        calc.apply_operation("^");
        assert_eq!(calc.get_result(), Some(32.0));
    }

    #[test]
    fn test_log_operation() {
        let mut calc = RPNCalculator::new();
        calc.apply_operation("1000");
        calc.apply_operation("log");
        // log10(1000) = 3
        assert_eq!(calc.get_result(), Some(3.0));
    }

    #[test]
    fn test_sqrt_operation() {
        let mut calc = RPNCalculator::new();
        calc.apply_operation("9");
        calc.apply_operation("sqrt");
        assert_eq!(calc.get_result(), Some(3.0));
    }

    #[test]
    fn test_abs_operation() {
        let mut calc = RPNCalculator::new();
        calc.apply_operation("-8");
        calc.apply_operation("abs");
        assert_eq!(calc.get_result(), Some(8.0));
    }

    #[test]
    fn test_factorial_operation() {
        let mut calc = RPNCalculator::new();
        calc.apply_operation("5");
        calc.apply_operation("!");
        // 5! = 120
        assert_eq!(calc.get_result(), Some(120.0));
    }

    #[test]
    fn test_factorial_zero() {
        let mut calc = RPNCalculator::new();
        calc.apply_operation("0");
        calc.apply_operation("!");
        // 0! gilt hier als Produkt eines leeren Intervalls, was per Definition 1 ergibt.
        assert_eq!(calc.get_result(), Some(1.0));
    }

    #[test]
    fn test_full_stack_addition() {
        let mut calc = RPNCalculator::new();
        calc.apply_operation("2");
        calc.apply_operation("4");
        calc.apply_operation("6");
        calc.apply_operation("++");
        // 2 + 4 + 6 = 12
        assert_eq!(calc.get_result(), Some(12.0));
    }

    #[test]
    fn test_full_stack_multiplication() {
        let mut calc = RPNCalculator::new();
        calc.apply_operation("2");
        calc.apply_operation("3");
        calc.apply_operation("4");
        calc.apply_operation("**");
        // 2 * 3 * 4 = 24
        assert_eq!(calc.get_result(), Some(24.0));
    }

    #[test]
    fn test_reconstruct_expression_infix() {
        let mut calc = RPNCalculator::new();
        calc.apply_operation("3");
        calc.apply_operation("4");
        calc.apply_operation("+");
        // Da reconstruct_expression_infix die history_stack von hinten rekonstruiert,
        // sollte das Ergebnis "(3 + 4)" sein.
        let infix_expr = calc.clone().reconstruct_expression_infix();
        assert_eq!(infix_expr, "(3 + 4)");
    }

    #[test]
    fn test_reconstruct_expression_latex() {
        let mut calc = RPNCalculator::new();
        calc.apply_operation("5");
        calc.apply_operation("2");
        calc.apply_operation("/");
        // Erwartetes LaTeX: "{\frac{5}{2}}"
        let latex_expr = calc.clone().reconstruct_expression_latex();
        assert_eq!(latex_expr, "{\\frac{5}{2}}");
    }

    #[test]
    fn test_invalid_input() {
        let mut calc = RPNCalculator::new();
        calc.apply_operation("7");
        // Ungültiger Token "foo": Es sollte eine Fehlermeldung ausgegeben und der Token wieder aus der history_stack entfernt werden.
        calc.apply_operation("foo");
        // Der Stack muss somit unverändert bleiben.
        assert_eq!(calc.get_result(), Some(7.0));
    }

    #[test]
    fn test_negative_numbers() {
        let mut calc = RPNCalculator::new();
        calc.apply_operation("-10");
        calc.apply_operation("-5");
        calc.apply_operation("+");
        // -10 + (-5) = -15
        assert_eq!(calc.get_result(), Some(-15.0));
    }
}
