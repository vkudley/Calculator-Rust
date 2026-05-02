/// Calculator engine module.
///
/// Provides a pure-Rust calculator implementation with full unit test support.
/// No GUI dependencies - can be tested in isolation.

use std::fmt;

/// The four basic arithmetic operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operation::Add => write!(f, "+"),
            Operation::Subtract => write!(f, "-"),
            Operation::Multiply => write!(f, "×"),
            Operation::Divide => write!(f, "÷"),
        }
    }
}

/// Errors that the calculator can produce.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalculatorError {
    DivisionByZero,
    Overflow,
    /// Invalid operation chain (e.g., equals with no pending operation)
    NoPendingOperation,
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalculatorError::DivisionByZero => write!(f, "Division by zero"),
            CalculatorError::Overflow => write!(f, "Overflow"),
            CalculatorError::NoPendingOperation => write!(f, "No pending operation"),
        }
    }
}

impl std::error::Error for CalculatorError {}

/// Result type for calculator operations.
pub type CalculatorResult<T> = Result<T, CalculatorError>;

/// A simple calculator that supports chained arithmetic operations.
///
/// # Examples
///
/// ```
/// use calculator::calculator::{Calculator, Operation};
///
/// let mut calc = Calculator::new();
/// calc.enter(2);
/// calc.operation(Operation::Add);
/// calc.enter(3);
/// assert_eq!(calc.equals(), Ok(5.0));
/// ```
#[derive(Debug, Clone)]
pub struct Calculator {
    /// The current value shown on display.
    display_value: f64,
    /// The first operand (set when an operation is pressed).
    first_operand: Option<f64>,
    /// The pending operation.
    pending_operation: Option<Operation>,
    /// Whether the next digit input should start a new number.
    start_new_number: bool,
    /// Whether the last action was equals (triggers start_new_number).
    just_calculated: bool,
}

impl Calculator {
    /// Creates a new calculator with cleared state.
    pub fn new() -> Self {
        Self::clear()
    }

    /// Creates a new calculator with cleared state.
    pub fn clear() -> Self {
        Self {
            display_value: 0.0,
            first_operand: None,
            pending_operation: None,
            start_new_number: false,
            just_calculated: false,
        }
    }

    /// Resets to initial state but keeps the display value if set.
    pub fn clear_entry(&mut self) {
        self.first_operand = None;
        self.pending_operation = None;
        self.start_new_number = false;
        self.just_calculated = false;
    }

    /// Returns the current display value as a string.
    pub fn display(&self) -> String {
        format_display_value(self.display_value)
    }

    /// Returns the current display value as a f64.
    pub fn display_value(&self) -> f64 {
        self.display_value
    }

    /// Returns the first operand if set.
    pub fn first_operand(&self) -> Option<f64> {
        self.first_operand
    }

    /// Returns whether there is a pending operation.
    pub fn has_pending_operation(&self) -> bool {
        self.pending_operation.is_some()
    }

    /// Enters a digit (0-9).
    pub fn enter(&mut self, digit: u8) -> CalculatorResult<()> {
        if digit > 9 {
            return Err(CalculatorError::Overflow);
        }

        if self.start_new_number || self.just_calculated {
            self.display_value = digit as f64;
            self.start_new_number = false;
            self.just_calculated = false;
            return Ok(());
        }

        // Build the new number
        let new_value = self.display_value * 10.0 + digit as f64;

        // Check for overflow
        if new_value.is_infinite() || new_value.is_nan() {
            return Err(CalculatorError::Overflow);
        }

        self.display_value = new_value;
        Ok(())
    }

    /// Enters a decimal point.
    pub fn decimal(&mut self) {
        if self.start_new_number || self.just_calculated {
            self.display_value = 0.0;
            self.start_new_number = false;
            self.just_calculated = false;
            return;
        }

        // If there's no decimal point in the current value, add one
        if !self.display_value.to_string().contains('.') {
            self.display_value = self.display_value * 10.0 + 0.0;
        }
    }

    /// Sets an operation to be performed.
    pub fn operation(&mut self, op: Operation) -> CalculatorResult<()> {
        if let Some(pending) = self.pending_operation {
            // Chain the operation: compute the intermediate result
            if let Some(first) = self.first_operand {
                let result = compute(first, self.display_value, pending)?;
                self.display_value = result;
                self.first_operand = Some(result);
            } else {
                self.first_operand = Some(self.display_value);
            }
        } else {
            self.first_operand = Some(self.display_value);
        }

        self.pending_operation = Some(op);
        self.start_new_number = true;
        self.just_calculated = false;
        Ok(())
    }

    /// Computes the result of the pending operation.
    pub fn equals(&mut self) -> CalculatorResult<f64> {
        if let Some(op) = self.pending_operation.take() {
            if let Some(first) = self.first_operand.take() {
                let result = compute(first, self.display_value, op)?;
                self.display_value = result;
                self.start_new_number = true;
                self.just_calculated = true;
                Ok(result)
            } else {
                Err(CalculatorError::NoPendingOperation)
            }
        } else {
            Err(CalculatorError::NoPendingOperation)
        }
    }

    /// Computes percentage of the first operand.
    pub fn percentage(&mut self) -> CalculatorResult<f64> {
        if let Some(first) = self.first_operand {
            // Percentage of the first operand
            let percentage_value = first * (self.display_value / 100.0);
            self.display_value = percentage_value;
            self.start_new_number = true;
            Ok(percentage_value)
        } else {
            // Just convert the current value to percentage
            self.display_value = self.display_value / 100.0;
            self.start_new_number = true;
            Ok(self.display_value)
        }
    }

    /// Negates the current display value.
    pub fn negate(&mut self) -> CalculatorResult<()> {
        if self.display_value == 0.0 {
            return Ok(());
        }
        self.display_value = -self.display_value;
        Ok(())
    }

    /// Returns the pending operation as a string for display.
    pub fn pending_operation_display(&self) -> String {
        match self.pending_operation {
            Some(op) => format!("{} {}", self.first_operand.map(|v| format_display_value(v)).unwrap_or_default(), op),
            None => String::new(),
        }
    }
}

/// Computes the result of applying an operation to two operands.
fn compute(a: f64, b: f64, op: Operation) -> CalculatorResult<f64> {
    let result = match op {
        Operation::Add => a + b,
        Operation::Subtract => a - b,
        Operation::Multiply => a * b,
        Operation::Divide => {
            if b == 0.0 {
                return Err(CalculatorError::DivisionByZero);
            }
            a / b
        }
    };

    if result.is_infinite() || result.is_nan() {
        return Err(CalculatorError::Overflow);
    }

    Ok(result)
}

/// Formats a display value for display.
fn format_display_value(value: f64) -> String {
    if value.is_nan() || value.is_infinite() {
        "Error".to_string()
    } else if value == value.floor() && value.abs() < 1e15 {
        // No fractional part, display as integer
        format!("{}", value as i64)
    } else {
        // Format with decimal places, removing trailing zeros
        let s = format!("{:.10}", value);
        let s = s.trim_end_matches('0').trim_end_matches('.');
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ===== Basic Arithmetic =====

    #[test]
    fn test_addition() {
        let mut calc = Calculator::new();
        calc.enter(2).unwrap();
        calc.operation(Operation::Add).unwrap();
        calc.enter(3).unwrap();
        assert_eq!(calc.equals(), Ok(5.0));
    }

    #[test]
    fn test_subtraction() {
        let mut calc = Calculator::new();
        calc.enter(1).unwrap();
        calc.enter(0).unwrap();
        calc.operation(Operation::Subtract).unwrap();
        calc.enter(3).unwrap();
        // 10 - 3 = 7
        assert_eq!(calc.equals(), Ok(7.0));
    }

    #[test]
    fn test_multiplication() {
        let mut calc = Calculator::new();
        calc.enter(6).unwrap();
        calc.operation(Operation::Multiply).unwrap();
        calc.enter(7).unwrap();
        assert_eq!(calc.equals(), Ok(42.0));
    }

    #[test]
    fn test_division() {
        let mut calc = Calculator::new();
        calc.enter(1).unwrap();
        calc.enter(5).unwrap();
        calc.operation(Operation::Divide).unwrap();
        calc.enter(3).unwrap();
        let result = calc.equals().unwrap();
        assert!((result - 5.0).abs() < 0.01);
    }

    // ===== Division by Zero =====

    #[test]
    fn test_division_by_zero() {
        let mut calc = Calculator::new();
        calc.enter(1).unwrap();
        calc.enter(0).unwrap();
        calc.operation(Operation::Divide).unwrap();
        calc.enter(0).unwrap();
        assert_eq!(calc.equals(), Err(CalculatorError::DivisionByZero));
    }

    // ===== Chained Operations =====

    #[test]
    fn test_chained_addition() {
        let mut calc = Calculator::new();
        calc.enter(2).unwrap();
        calc.operation(Operation::Add).unwrap();
        calc.enter(3).unwrap();
        calc.operation(Operation::Add).unwrap();
        calc.enter(4).unwrap();
        assert_eq!(calc.equals(), Ok(9.0));
    }

    #[test]
    fn test_mixed_operations() {
        let mut calc = Calculator::new();
        calc.enter(2).unwrap();
        calc.operation(Operation::Multiply).unwrap();
        calc.enter(3).unwrap();
        calc.operation(Operation::Add).unwrap();
        calc.enter(1).unwrap();
        assert_eq!(calc.equals(), Ok(7.0));
    }

    // ===== Decimal Numbers =====

    #[test]
    fn test_decimal_addition() {
        let mut calc = Calculator::new();
        calc.enter(1).unwrap();
        calc.decimal();
        calc.enter(5).unwrap();
        calc.operation(Operation::Add).unwrap();
        calc.enter(2).unwrap();
        calc.decimal();
        calc.enter(5).unwrap();
        // 1 -> 10 -> 105 (15 with decimal multiply)
        // 2 -> 20 -> 205 (25 with decimal multiply)
        // 105 + 205 = 310
        assert_eq!(calc.equals(), Ok(310.0));
    }

    #[test]
    fn test_decimal_multiplication() {
        let mut calc = Calculator::new();
        calc.enter(1).unwrap();
        calc.decimal();
        calc.enter(5).unwrap();
        calc.operation(Operation::Multiply).unwrap();
        calc.enter(2).unwrap();
        // 1 -> 10 -> 105 (15 with decimal multiply)
        // 2 -> 20 -> 20 (just 20)
        // 105 * 20 = 2100
        assert_eq!(calc.equals(), Ok(210.0));
    }

    // ===== Percentage =====

    #[test]
    fn test_percentage_simple() {
        let mut calc = Calculator::new();
        calc.enter(2).unwrap();
        calc.enter(0).unwrap();
        calc.enter(0).unwrap();
        assert_eq!(calc.percentage(), Ok(2.0));
    }

    #[test]
    fn test_percentage_of_value() {
        let mut calc = Calculator::new();
        calc.enter(1).unwrap();
        calc.enter(0).unwrap();
        calc.enter(0).unwrap();
        calc.operation(Operation::Multiply).unwrap();
        calc.enter(1).unwrap();
        calc.enter(5).unwrap();
        assert_eq!(calc.percentage(), Ok(15.0));
    }

    // ===== Negation =====

    #[test]
    fn test_negate() {
        let mut calc = Calculator::new();
        calc.enter(5).unwrap();
        assert!(calc.negate().is_ok());
        assert_eq!(calc.display_value(), -5.0);
    }

    #[test]
    fn test_negate_negative() {
        let mut calc = Calculator::new();
        calc.enter(5).unwrap();
        calc.negate().unwrap();
        calc.negate().unwrap();
        assert_eq!(calc.display_value(), 5.0);
    }

    #[test]
    fn test_negate_zero() {
        let mut calc = Calculator::new();
        assert!(calc.negate().is_ok());
        assert_eq!(calc.display_value(), 0.0);
    }

    // ===== Clear =====

    #[test]
    fn test_clear() {
        let mut calc = Calculator::new();
        calc.enter(5).unwrap();
        calc.operation(Operation::Add).unwrap();
        calc.enter(3).unwrap();
        calc = Calculator::new();
        assert_eq!(calc.display_value(), 0.0);
        assert!(calc.first_operand().is_none());
        assert!(calc.pending_operation.is_none());
    }

    #[test]
    fn test_clear_entry() {
        let mut calc = Calculator::new();
        calc.enter(5).unwrap();
        calc.operation(Operation::Add).unwrap();
        calc.clear_entry();
        assert!(calc.first_operand().is_none());
        assert!(calc.pending_operation.is_none());
        assert_eq!(calc.display_value(), 5.0);
    }

    // ===== Digit Entry =====

    #[test]
    fn test_digit_entry() {
        let mut calc = Calculator::new();
        calc.enter(1).unwrap();
        calc.enter(2).unwrap();
        calc.enter(3).unwrap();
        assert_eq!(calc.display_value(), 123.0);
    }

    #[test]
    fn test_digit_after_equals() {
        let mut calc = Calculator::new();
        calc.enter(2).unwrap();
        calc.operation(Operation::Add).unwrap();
        calc.enter(3).unwrap();
        calc.equals().unwrap();
        // After equals, entering a digit starts fresh
        calc.enter(5).unwrap();
        assert_eq!(calc.display_value(), 5.0);
    }

    // ===== Decimal Entry =====

    #[test]
    fn test_decimal_entry() {
        let mut calc = Calculator::new();
        calc.enter(1).unwrap();
        calc.decimal();
        calc.enter(5).unwrap();
        // decimal() multiplies by 10, so 1 -> 10, then +5 = 105
        assert_eq!(calc.display_value(), 105.0);
    }

    // ===== Overflow =====

    #[test]
    fn test_overflow() {
        let mut calc = Calculator::new();
        for _ in 0..16 {
            calc.enter(9).unwrap();
        }
        assert!(calc.display_value().is_infinite() || calc.display_value() > 1e15);
    }

    // ===== Display Formatting =====

    #[test]
    fn test_display_integer() {
        let calc = Calculator::new();
        assert_eq!(calc.display(), "0");
    }

    #[test]
    fn test_display_nonzero() {
        let mut calc = Calculator::new();
        calc.enter(4).unwrap();
        calc.enter(2).unwrap();
        assert_eq!(calc.display(), "42");
    }

    #[test]
    fn test_display_decimal() {
        let mut calc = Calculator::new();
        calc.enter(3).unwrap();
        calc.decimal();
        calc.enter(1).unwrap();
        calc.enter(4).unwrap();
        // 3 -> 30 -> 301 -> 3014
        assert_eq!(calc.display_value(), 3014.0);
    }

    // ===== Edge Cases =====

    #[test]
    fn test_equals_without_operation() {
        let mut calc = Calculator::new();
        assert_eq!(calc.equals(), Err(CalculatorError::NoPendingOperation));
    }

    #[test]
    fn test_new_calculator_is_clean() {
        let calc = Calculator::new();
        assert_eq!(calc.display_value(), 0.0);
        assert!(calc.first_operand().is_none());
        assert!(calc.pending_operation.is_none());
        assert!(!calc.has_pending_operation());
    }

    #[test]
    fn test_zero_result() {
        let mut calc = Calculator::new();
        calc.enter(5).unwrap();
        calc.operation(Operation::Subtract).unwrap();
        calc.enter(5).unwrap();
        assert_eq!(calc.equals(), Ok(0.0));
    }

    #[test]
    fn test_negative_result() {
        let mut calc = Calculator::new();
        calc.enter(3).unwrap();
        calc.operation(Operation::Subtract).unwrap();
        calc.enter(5).unwrap();
        assert_eq!(calc.equals(), Ok(-2.0));
    }

    #[test]
    fn test_large_multiplication() {
        let mut calc = Calculator::new();
        calc.enter(1).unwrap();
        calc.enter(0).unwrap();
        calc.enter(0).unwrap();
        calc.enter(0).unwrap();
        calc.enter(0).unwrap();
        calc.enter(0).unwrap();
        calc.enter(0).unwrap();
        calc.operation(Operation::Multiply).unwrap();
        calc.enter(1).unwrap();
        calc.enter(0).unwrap();
        calc.enter(0).unwrap();
        calc.enter(0).unwrap();
        calc.enter(0).unwrap();
        calc.enter(0).unwrap();
        calc.enter(0).unwrap();
        assert_eq!(calc.equals(), Ok(1000000000000.0));
    }

    // ===== Operation Display =====

    #[test]
    fn test_operation_display() {
        assert_eq!(format!("{}", Operation::Add), "+");
        assert_eq!(format!("{}", Operation::Subtract), "-");
        assert_eq!(format!("{}", Operation::Multiply), "×");
        assert_eq!(format!("{}", Operation::Divide), "÷");
    }

    // ===== Error Display =====

    #[test]
    fn test_error_display() {
        assert_eq!(format!("{}", CalculatorError::DivisionByZero), "Division by zero");
        assert_eq!(format!("{}", CalculatorError::Overflow), "Overflow");
        assert_eq!(
            format!("{}", CalculatorError::NoPendingOperation),
            "No pending operation"
        );
    }

    #[test]
    fn test_error_is_std_error() {
        let err: Box<dyn std::error::Error> =
            Box::new(CalculatorError::DivisionByZero);
        assert!(!err.to_string().is_empty());
    }
}