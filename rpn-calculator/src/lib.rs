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
    use super::RPNCalculator;

    // Test für Addition
    #[test]
    fn test_addition() {
        // Erstelle einen neuen Rechner
        let mut calc = RPNCalculator::new();

        // Füge die Zahlen und die Addition-Operation hinzu
        calc.apply_operation("3"); // 3
        calc.apply_operation("5"); // 5
        calc.apply_operation("+"); // 3 + 5

        // Überprüfe das Ergebnis: Erwartet ist 8
        assert_eq!(calc.get_result(), Some(8.0));
    }

    // Test für Subtraktion
    #[test]
    fn test_subtraction() {
        let mut calc = RPNCalculator::new();

        calc.apply_operation("7"); // 7
        calc.apply_operation("4"); // 4
        calc.apply_operation("-"); // 7 - 4

        assert_eq!(calc.get_result(), Some(3.0)); // Erwartet: 3
    }

    // Test für Multiplikation
    #[test]
    fn test_multiplication() {
        let mut calc = RPNCalculator::new();

        calc.apply_operation("6"); // 6
        calc.apply_operation("2"); // 2
        calc.apply_operation("*"); // 6 * 2

        assert_eq!(calc.get_result(), Some(12.0)); // Erwartet: 12
    }

    // Test für Division
    #[test]
    fn test_division() {
        let mut calc = RPNCalculator::new();

        calc.apply_operation("10"); // 10
        calc.apply_operation("2"); // 2
        calc.apply_operation("/"); // 10 / 2

        assert_eq!(calc.get_result(), Some(5.0)); // Erwartet: 5
    }

    // Test für Exponentiation
    #[test]
    fn test_exponentiation() {
        let mut calc = RPNCalculator::new();

        calc.apply_operation("2"); // 2
        calc.apply_operation("3"); // 3
        calc.apply_operation("^"); // 2 ^ 3

        assert_eq!(calc.get_result(), Some(8.0)); // Erwartet: 8
    }

    // Test für Quadratwurzel
    #[test]
    fn test_sqrt() {
        let mut calc = RPNCalculator::new();

        calc.apply_operation("9"); // 9
        calc.apply_operation("sqrt"); // sqrt(9)

        assert_eq!(calc.get_result(), Some(3.0)); // Erwartet: 3
    }

    // Test für Logarithmus
    #[test]
    fn test_log() {
        let mut calc = RPNCalculator::new();

        calc.apply_operation("100"); // 100
        calc.apply_operation("log"); // log(100)

        assert_eq!(calc.get_result(), Some(2.0)); // Erwartet: 2
    }

    // Test für Betrag
    #[test]
    fn test_abs() {
        let mut calc = RPNCalculator::new();

        calc.apply_operation("-5"); // -5
        calc.apply_operation("abs"); // abs(-5)

        assert_eq!(calc.get_result(), Some(5.0)); // Erwartet: 5
    }

    // Test für Fakultät
    #[test]
    fn test_factorial() {
        let mut calc = RPNCalculator::new();

        calc.apply_operation("5"); // 5
        calc.apply_operation("!"); // 5!

        assert_eq!(calc.get_result(), Some(120.0)); // Erwartet: 120
    }

    // Test für Addition über den gesamten Stack
    #[test]
    fn test_full_stack_addition() {
        let mut calc = RPNCalculator::new();

        calc.apply_operation("1"); // 1
        calc.apply_operation("2"); // 2
        calc.apply_operation("3"); // 3
        calc.apply_operation("++"); // 1 + 2 + 3

        assert_eq!(calc.get_result(), Some(6.0)); // Erwartet: 6
    }

    // Test für Multiplikation über den gesamten Stack
    #[test]
    fn test_full_stack_multiplication() {
        let mut calc = RPNCalculator::new();

        calc.apply_operation("2"); // 2
        calc.apply_operation("3"); // 3
        calc.apply_operation("4"); // 4
        calc.apply_operation("**"); // 2 * 3 * 4

        assert_eq!(calc.get_result(), Some(24.0)); // Erwartet: 24
    }

    // Test für ungültige Eingaben
    #[test]
    fn test_invalid_input() {
        let mut calc = RPNCalculator::new();

        calc.apply_operation("abc"); // ungültige Eingabe

        // Die Eingabe wird nicht verarbeitet und der Stack bleibt unverändert
        assert_eq!(calc.get_result(), None); // Kein Ergebnis
    }

    // Test für Division durch Null
    #[test]
    fn test_division_by_zero() {
        let mut calc = RPNCalculator::new();

        calc.apply_operation("10"); // 10
        calc.apply_operation("0"); // 0
        calc.apply_operation("/"); // 10 / 0

        // Sollte "inf" oder eine ähnliche Fehlerbehandlung erzeugen
        assert_eq!(calc.get_result(), Some(f64::INFINITY)); // Erwartet: unendlich
    }


    // Test für Infix-Notation Rekonstruktion
    #[test]
    fn test_infix_reconstruction() {
        let mut calc = RPNCalculator::new();

        calc.apply_operation("3"); // 3
        calc.apply_operation("5"); // 5
        calc.apply_operation("+"); // 3 + 5
        calc.apply_operation("2"); // 2
        calc.apply_operation("*"); // (3 + 5) * 2

        let infix = calc.reconstruct_expression_infix();

        // Erwartet: ((3 + 5) * 2)
        assert_eq!(infix, "((3 + 5) * 2)");
    }

    // Test für LaTeX-Notation Rekonstruktion
    #[test]
    fn test_latex_reconstruction() {
        let mut calc = RPNCalculator::new();

        calc.apply_operation("3"); // 3
        calc.apply_operation("5"); // 5
        calc.apply_operation("+"); // 3 + 5
        calc.apply_operation("2"); // 2
        calc.apply_operation("*"); // (3 + 5) * 2

        let latex = calc.reconstruct_expression_latex();

        // Erwartet: {{3 + 5} \\cdot 2}
        assert_eq!(latex, "{{3 + 5} \\cdot 2}");
    }

    // Test für komplexe Berechnungen
    #[test]
    fn test_simple_complex_calculation() {
        let mut calc = RPNCalculator::new();

        // Eingabe: 3 4 + 2 * 5 +
        calc.apply_operation("3"); // 3
        calc.apply_operation("4"); // 4
        calc.apply_operation("+"); // 3 + 4 = 7
        calc.apply_operation("2"); // 2
        calc.apply_operation("*"); // 7 * 2 = 14
        calc.apply_operation("5"); // 5
        calc.apply_operation("+"); // 14 + 5 = 19

        assert_eq!(calc.get_result(), Some(19.0)); // Erwartet: 19
    }
}
