struct Value(f64);

impl Value {
    fn evaluate(&self) -> f64 {
        self.0
    }
}

enum Operator {
    Addition { lhs: Value, rhs: Value },
    Subtraction { lhs: Value, rhs: Value },
    Multiplication { lhs: Value, rhs: Value },
    Division { lhs: Value, rhs: Value },
    Negation { operand: Value },
}

impl Operator {
    fn apply(&self) -> Value {
        let inner = match self {
            Operator::Addition { lhs, rhs } => lhs.evaluate() + rhs.evaluate(),
            Operator::Subtraction { lhs, rhs } => lhs.evaluate() - rhs.evaluate(),
            Operator::Multiplication { lhs, rhs } => lhs.evaluate() * rhs.evaluate(),
            Operator::Division { lhs, rhs } => lhs.evaluate() / rhs.evaluate(),
            Operator::Negation { operand } => -operand.evaluate(),
        };
        Value(inner)
    }

    fn precedence(&self) -> u8 {
        match self {
            Operator::Addition { lhs, rhs } => todo!(),
            Operator::Subtraction { lhs, rhs } => todo!(),
            Operator::Multiplication { lhs, rhs } => todo!(),
            Operator::Division { lhs, rhs } => todo!(),
            Operator::Negation { operand } => todo!(),
        }
    }
    fn symbol(&self) -> char {
        match self {
            Operator::Addition { .. } => '+',
            Operator::Subtraction { .. } => '-',
            Operator::Multiplication { .. } => '*',
            Operator::Division { .. } => '/',
            Operator::Negation { .. } => '-',
        }
    }
}

fn main() {
    let addition = Operator::Addition {
        lhs: Value(2.0),
        rhs: Value(3.0),
    };
    let subtraction = Operator::Subtraction {
        lhs: Value(5.0),
        rhs: Value(1.0),
    };
    let negation = Operator::Negation {
        operand: Value(-7.0),
    };

    println!("Addition result: {}", addition.apply().evaluate());
    println!("Subtraction result: {}", subtraction.apply().evaluate());
    println!("Negation result: {}", negation.apply().evaluate());
}
