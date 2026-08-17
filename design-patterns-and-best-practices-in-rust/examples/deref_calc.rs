trait Operand {
    fn evaluate(&self) -> f64;
}

trait Operator {
    fn precedence(&self) -> u8;
    fn symbol(&self) -> char;
    fn push_operand(&mut self, operand: Box<dyn Operand>);
    fn pop_operand(&mut self) -> Box<dyn Operand>;
    fn apply(&mut self) -> Box<dyn Operand>;
}

struct OperandStack(Vec<Box<dyn Operand>>);

impl OperandStack {
    fn new() -> Self {
        Self(Vec::new())
    }

    fn push_operand(&mut self, operand: Box<dyn Operand>) {
        self.0.push(operand);
    }

    fn pop_operand(&mut self) -> Box<dyn Operand> {
        self.0.pop().unwrap()
    }
}

struct AdditionOperator {
    pub stack: OperandStack,
}

impl AdditionOperator {
    fn new() -> Self {
        Self {
            stack: OperandStack::new(),
        }
    }
}

struct Value(f64);

impl Operand for Value {
    fn evaluate(&self) -> f64 {
        self.0
    }
}

fn main() {
    let value = Value(4.0);
    let mut addition_operator = AdditionOperator::new();
    addition_operator.stack.push_operand(Box::new(value));
    let popped_operator = addition_operator.stack.pop_operand();
    println!("{:?}", popped_operator.evaluate());
}
