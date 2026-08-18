use std::cell::{Ref, RefCell};

struct CalculationResult {
    expression: String,
    result: f64,
}

struct Calculator {
    history: RefCell<Vec<CalculationResult>>,
    current_expression: RefCell<Option<String>>,
}

trait HistoryViewer {
    fn view_history(&self) -> Ref<Vec<CalculationResult>>;
    fn get_last_result(&self) -> Option<f64>;
}

trait HistoryManager {
    fn add_to_history(&self, expression: String, result: f64);
    fn clear_history(&self);
}

impl Calculator {
    fn new() -> Self {
        Self {
            history: RefCell::new(Vec::new()),
            current_expression: RefCell::new(None),
        }
    }
}

impl HistoryViewer for Calculator {
    fn view_history(&self) -> Ref<'_, Vec<CalculationResult>> {
        self.history.borrow()
    }

    fn get_last_result(&self) -> Option<f64> {
        self.history.borrow().last().map(|r| r.result)
    }
}

impl HistoryManager for Calculator {
    fn add_to_history(&self, expression: String, result: f64) {
        self.history
            .borrow_mut()
            .push(CalculationResult { expression, result });
    }

    fn clear_history(&self) {
        self.history.borrow_mut().clear();
    }
}

fn main() {}
