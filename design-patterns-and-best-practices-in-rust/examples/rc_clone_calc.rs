use std::collections::HashMap;

struct CalculationResult {
    expression: String,
    result: f64,
}

struct Variable {
    name: String,
    value: f64,
}

struct Calculator {
    history: Vec<CalculationResult>,
    current_expression: Option<String>,
    variables: HashMap<String, Variable>,
}

enum Token<'a> {
    Number(f64),
    Variable(&'a Variable),
    Operator(char),
}

trait HistoryViewer {
    fn view_history(&self) -> &[CalculationResult];
    fn get_last_result(&self) -> Option<f64>;
}

trait HistoryManager {
    fn add_to_history(&mut self, expression: String, result: f64);
    fn clear_history(&mut self);
    fn evaluate(&mut self, tokens: Vec<Token>) -> f64;
}

impl Calculator {
    fn new() -> Self {
        Self {
            history: Vec::new(),
            current_expression: None,
            variables: HashMap::new(),
        }
    }

    fn tokenize<'a>(&'a self, expression: &str) -> Vec<Token<'a>> {
        let mut tokens = Vec::new();

        for part in expression.split_whitespace() {
            if let Some(var) = self.variables.get(part) {
                // just use a reference
                tokens.push(Token::Variable(var));
            }
            // ... rest of tokenization
        }
        tokens
    }

    fn apply_operator(&self, operator: char, left: f64, right: f64) -> Result<f64, String> {
        match operator {
            '+' => Ok(left + right),
            '-' => Ok(left - right),
            '*' => Ok(left * right),
            '/' => {
                if right == 0.0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(left / right)
                }
            }
            _ => Err("Invalid operator".to_string()),
        }
    }
}

impl HistoryViewer for Calculator {
    fn view_history(&self) -> &[CalculationResult] {
        &self.history
    }

    fn get_last_result(&self) -> Option<f64> {
        self.history.last().map(|r| r.result)
    }
}

impl HistoryManager for Calculator {
    fn add_to_history(&mut self, expression: String, result: f64) {
        self.history.push(CalculationResult { expression, result });
    }

    fn clear_history(&mut self) {
        self.history.clear();
    }

    fn evaluate(&mut self, mut tokens: Vec<Token>) -> f64 {
        while tokens.len() > 1 {
            // find next operator
            let op_pos = tokens
                .iter()
                .position(|t| matches!(t, Token::Operator(_)))
                .unwrap();

            // calculate result using references
            let result = match (&tokens[op_pos - 1], &tokens[op_pos], &tokens[op_pos + 1]) {
                (Token::Number(n1), Token::Operator(op), Token::Number(n2)) => {
                    self.apply_operator(*op, *n1, *n2)
                }
                (Token::Variable(v1), Token::Operator(op), Token::Number(n2)) => {
                    self.apply_operator(*op, v1.value, *n2)
                }
                (Token::Number(n1), Token::Operator(op), Token::Variable(v2)) => {
                    self.apply_operator(*op, *n1, v2.value)
                }
                (Token::Variable(v1), Token::Operator(op), Token::Variable(v2)) => {
                    self.apply_operator(*op, v1.value, v2.value)
                }
                _ => panic!("Invalid expression"),
            };

            // remove old tokens and insert result
            tokens.drain(op_pos - 1..=op_pos + 1);
            tokens.insert(op_pos - 1, Token::Number(result.unwrap()));
        }
        match tokens[0] {
            Token::Number(n) => n,
            _ => panic!("Invalid expression"),
        }
    }
}

fn main() {
    let mut calc = Calculator::new();
}
