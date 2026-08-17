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

trait UnaryOperator: Operator {
    fn apply_unary(&mut self, operand: Box<dyn Operand>) -> Box<dyn Operand>;

    fn apply(&mut self) -> Box<dyn Operand> {
        let operand = self.pop_operand();
        self.apply_unary(operand)
    }
}

trait BinaryOperator: Operator {
    fn apply_binary(&self, left: Box<dyn Operand>, right: Box<dyn Operand>) -> Box<dyn Operand>;

    fn apply(&mut self) -> Box<dyn Operand> {
        let right = self.pop_operand();
        let left = self.pop_operand();
        self.apply_binary(left, right)
    }
}

struct Value(f64);

impl Operand for Value {
    fn evaluate(&self) -> f64 {
        self.0
    }
}

// 加法
struct AdditionOperator {
    stack: Vec<Box<dyn Operand>>,
}

impl BinaryOperator for AdditionOperator {
    fn apply_binary(&self, left: Box<dyn Operand>, right: Box<dyn Operand>) -> Box<dyn Operand> {
        Box::new(Value(left.evaluate() + right.evaluate()))
    }
}

impl Operator for AdditionOperator {
    fn precedence(&self) -> u8 {
        0
    }

    fn symbol(&self) -> char {
        '+'
    }

    fn push_operand(&mut self, operand: Box<dyn Operand>) {
        self.stack.push(operand);
    }

    fn pop_operand(&mut self) -> Box<dyn Operand> {
        self.stack.pop().unwrap()
    }

    fn apply(&mut self) -> Box<dyn Operand> {
        let right = self.pop_operand();
        let left = self.pop_operand();
        self.apply_binary(left, right)
    }
}

fn main() {}
