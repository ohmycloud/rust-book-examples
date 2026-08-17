trait OperationTrait {
    fn calculate(&self, left: f64, right: f64) -> f64;
    fn precedence(&self) -> u8;
}

struct Add;
struct Multiply;

impl OperationTrait for Add {
    fn calculate(&self, left: f64, right: f64) -> f64 {
        left + right
    }

    fn precedence(&self) -> u8 {
        1
    }
}

impl OperationTrait for Multiply {
    fn calculate(&self, left: f64, right: f64) -> f64 {
        left * right
    }

    fn precedence(&self) -> u8 {
        2
    }
}

struct Operation<T: OperationTrait> {
    symbol: String,
    op_type: T,
}

impl<T: OperationTrait> Operation<T> {
    fn new(symbol: String, op_type: T) -> Self {
        Operation { symbol, op_type }
    }
    fn evaluate(&self, left: f64, right: f64) -> f64 {
        self.op_type.calculate(left, right)
    }
}

fn main() {
    let add_op = Operation::new("+".to_string(), Add);
    let mul_op = Operation::new("*".to_string(), Multiply);

    println!("5 {} 3 = {}", add_op.symbol, add_op.evaluate(5.0, 3.0));
    println!("5 {} 3 = {}", mul_op.symbol, mul_op.evaluate(5.0, 3.0));

    // Attemp to create a vector of different operations
    // let operations: Vec<Operation<_>> = vec![add_op, mul_op];
}
