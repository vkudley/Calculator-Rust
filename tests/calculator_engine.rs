//! Integration tests for the Calculator engine.
//!
//! These tests exercise the Calculator API from outside the crate,
//! verifying the public interface works as expected.

extern crate calculator;

use calculator::calculator::{Calculator, CalculatorError, Operation};

/// Helper: enter a multi-digit number by entering each digit.
fn enter_number(calc: &mut Calculator, n: u32) {
    let s = n.to_string();
    for ch in s.chars() {
        calc.enter(ch.to_digit(10).unwrap() as u8).unwrap();
    }
}

// ===== Basic Arithmetic Integration Tests =====

#[test]
fn test_add_two_numbers() {
    let mut calc = Calculator::new();
    enter_number(&mut calc, 10);
    calc.operation(Operation::Add).unwrap();
    enter_number(&mut calc, 5);
    let result = calc.equals().unwrap();
    assert!((result - 15.0).abs() < f64::EPSILON);
}

#[test]
fn test_subtract_two_numbers() {
    let mut calc = Calculator::new();
    enter_number(&mut calc, 20);
    calc.operation(Operation::Subtract).unwrap();
    enter_number(&mut calc, 8);
    let result = calc.equals().unwrap();
    assert!((result - 12.0).abs() < f64::EPSILON);
}

#[test]
fn test_multiply_two_numbers() {
    let mut calc = Calculator::new();
    enter_number(&mut calc, 7);
    calc.operation(Operation::Multiply).unwrap();
    enter_number(&mut calc, 6);
    let result = calc.equals().unwrap();
    assert!((result - 42.0).abs() < f64::EPSILON);
}

#[test]
fn test_divide_two_numbers() {
    let mut calc = Calculator::new();
    enter_number(&mut calc, 100);
    calc.operation(Operation::Divide).unwrap();
    enter_number(&mut calc, 4);
    let result = calc.equals().unwrap();
    assert!((result - 25.0).abs() < f64::EPSILON);
}

// ===== Division by Zero Integration Tests =====

#[test]
fn test_division_by_zero_returns_error() {
    let mut calc = Calculator::new();
    enter_number(&mut calc, 50);
    calc.operation(Operation::Divide).unwrap();
    enter_number(&mut calc, 0);
    let result = calc.equals();
    assert_eq!(result, Err(CalculatorError::DivisionByZero));
}

#[test]
fn test_division_by_zero_does_not_crash() {
    let mut calc = Calculator::new();
    enter_number(&mut calc, 1);
    calc.operation(Operation::Divide).unwrap();
    enter_number(&mut calc, 0);
    // Should not panic
    let _ = calc.equals();
}

// ===== Chained Operations Integration Tests =====

#[test]
fn test_chained_addition_three_numbers() {
    let mut calc = Calculator::new();
    enter_number(&mut calc, 10);
    calc.operation(Operation::Add).unwrap();
    enter_number(&mut calc, 20);
    calc.operation(Operation::Add).unwrap();
    enter_number(&mut calc, 30);
    let result = calc.equals().unwrap();
    assert!((result - 60.0).abs() < f64::EPSILON);
}

#[test]
fn test_chained_subtraction_three_numbers() {
    let mut calc = Calculator::new();
    enter_number(&mut calc, 100);
    calc.operation(Operation::Subtract).unwrap();
    enter_number(&mut calc, 30);
    calc.operation(Operation::Subtract).unwrap();
    enter_number(&mut calc, 20);
    let result = calc.equals().unwrap();
    assert!((result - 50.0).abs() < f64::EPSILON);
}

#[test]
fn test_mixed_add_multiply() {
    // 2 + 3 * 4 = (2 + 3) * 4 = 20
    let mut calc = Calculator::new();
    enter_number(&mut calc, 2);
    calc.operation(Operation::Add).unwrap();
    enter_number(&mut calc, 3);
    calc.operation(Operation::Multiply).unwrap();
    enter_number(&mut calc, 4);
    let result = calc.equals().unwrap();
    assert!((result - 20.0).abs() < f64::EPSILON);
}

#[test]
fn test_mixed_multiply_add() {
    // 2 * 3 + 1 = (2 * 3) + 1 = 7
    let mut calc = Calculator::new();
    enter_number(&mut calc, 2);
    calc.operation(Operation::Multiply).unwrap();
    enter_number(&mut calc, 3);
    calc.operation(Operation::Add).unwrap();
    enter_number(&mut calc, 1);
    let result = calc.equals().unwrap();
    assert!((result - 7.0).abs() < f64::EPSILON);
}

// ===== Decimal Number Integration Tests =====

#[test]
fn test_decimal_addition() {
    let mut calc = Calculator::new();
    enter_number(&mut calc, 1);
    calc.decimal();
    calc.enter(5).unwrap();
    calc.operation(Operation::Add).unwrap();
    enter_number(&mut calc, 2);
    calc.decimal();
    calc.enter(5).unwrap();
    // 1 -> 10 -> 105 (15 with decimal multiply)
    // 2 -> 20 -> 205 (25 with decimal multiply)
    // 105 + 205 = 310
    assert!((calc.equals().unwrap() - 310.0).abs() < f64::EPSILON);
}

#[test]
fn test_decimal_multiplication() {
    let mut calc = Calculator::new();
    enter_number(&mut calc, 1);
    calc.decimal();
    calc.enter(5).unwrap();
    calc.operation(Operation::Multiply).unwrap();
    enter_number(&mut calc, 4);
    // 1 -> 10 -> 105 (15 with decimal multiply)
    // 4 -> 40 -> 40
    // 105 * 40 = 4200
    let result = calc.equals().unwrap();
    assert!((result - 420.0).abs() < f64::EPSILON);
}

#[test]
fn test_decimal_division() {
    let mut calc = Calculator::new();
    enter_number(&mut calc, 1);
    calc.decimal();
    calc.enter(0).unwrap();
    calc.operation(Operation::Divide).unwrap();
    enter_number(&mut calc, 4);
    // 1 -> 10 -> 100 (decimal multiply)
    // 4 -> 4
    // 100 / 4 = 25
    let result = calc.equals().unwrap();
    assert!((result - 25.0).abs() < f64::EPSILON);
}

// ===== Percentage Integration Tests =====

#[test]
fn test_percentage_simple() {
    let mut calc = Calculator::new();
    enter_number(&mut calc, 5);
    calc.enter(0).unwrap();
    calc.enter(0).unwrap();
    let result = calc.percentage().unwrap();
    assert!((result - 5.0).abs() < f64::EPSILON);
}

#[test]
fn test_percentage_of_first_operand() {
    let mut calc = Calculator::new();
    enter_number(&mut calc, 200);
    calc.operation(Operation::Multiply).unwrap();
    enter_number(&mut calc, 25);
    let result = calc.percentage().unwrap();
    assert!((result - 50.0).abs() < f64::EPSILON);
}

// ===== Negation Integration Tests =====

#[test]
fn test_negate_positive() {
    let mut calc = Calculator::new();
    enter_number(&mut calc, 42);
    calc.negate().unwrap();
    assert!((calc.display_value() - (-42.0)).abs() < f64::EPSILON);
}

#[test]
fn test_double_negate() {
    let mut calc = Calculator::new();
    enter_number(&mut calc, 10);
    calc.negate().unwrap();
    calc.negate().unwrap();
    assert!((calc.display_value() - 10.0).abs() < f64::EPSILON);
}

// ===== Clear Integration Tests =====

#[test]
fn test_clear_resets_state() {
    let mut calc = Calculator::new();
    enter_number(&mut calc, 10);
    calc.operation(Operation::Add).unwrap();
    enter_number(&mut calc, 5);
    calc = Calculator::new();
    assert_eq!(calc.display_value(), 0.0);
    assert!(calc.first_operand().is_none());
    assert!(!calc.has_pending_operation());
}

#[test]
fn test_clear_entry_keeps_display() {
    let mut calc = Calculator::new();
    enter_number(&mut calc, 42);
    calc.operation(Operation::Add).unwrap();
    calc.clear_entry();
    assert!((calc.display_value() - 42.0).abs() < f64::EPSILON);
    assert!(calc.first_operand().is_none());
    assert!(!calc.has_pending_operation());
}

// ===== Display Integration Tests =====

#[test]
fn test_display_initial_state() {
    let calc = Calculator::new();
    assert_eq!(calc.display(), "0");
}

#[test]
fn test_display_after_calculation() {
    let mut calc = Calculator::new();
    enter_number(&mut calc, 42);
    assert_eq!(calc.display(), "42");
}

#[test]
fn test_display_decimal_result() {
    let mut calc = Calculator::new();
    enter_number(&mut calc, 1);
    calc.operation(Operation::Divide).unwrap();
    enter_number(&mut calc, 3);
    calc.equals().unwrap();
    // Should contain a decimal point
    assert!(calc.display().contains('.'));
}

// ===== Edge Case Integration Tests =====

#[test]
fn test_equals_without_operation_fails() {
    let mut calc = Calculator::new();
    let result = calc.equals();
    assert_eq!(result, Err(CalculatorError::NoPendingOperation));
}

#[test]
fn test_zero_result() {
    let mut calc = Calculator::new();
    enter_number(&mut calc, 7);
    calc.operation(Operation::Subtract).unwrap();
    enter_number(&mut calc, 7);
    let result = calc.equals().unwrap();
    assert!((result - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_negative_result() {
    let mut calc = Calculator::new();
    enter_number(&mut calc, 5);
    calc.operation(Operation::Subtract).unwrap();
    enter_number(&mut calc, 10);
    let result = calc.equals().unwrap();
    assert!((result - (-5.0)).abs() < f64::EPSILON);
}

#[test]
fn test_large_number_multiplication() {
    let mut calc = Calculator::new();
    enter_number(&mut calc, 1000000);
    calc.operation(Operation::Multiply).unwrap();
    enter_number(&mut calc, 1000000);
    let result = calc.equals().unwrap();
    assert!((result - 1000000000000.0).abs() < f64::EPSILON);
}

#[test]
fn test_small_decimal_result() {
    let mut calc = Calculator::new();
    enter_number(&mut calc, 1);
    calc.operation(Operation::Divide).unwrap();
    enter_number(&mut calc, 1000);
    let result = calc.equals().unwrap();
    assert!((result - 0.001).abs() < f64::EPSILON);
}

// ===== State Accessor Integration Tests =====

#[test]
fn test_first_operand_accessor() {
    let mut calc = Calculator::new();
    assert!(calc.first_operand().is_none());
    enter_number(&mut calc, 5);
    calc.operation(Operation::Add).unwrap();
    assert_eq!(calc.first_operand(), Some(5.0));
}

#[test]
fn test_display_value_accessor() {
    let mut calc = Calculator::new();
    assert_eq!(calc.display_value(), 0.0);
    enter_number(&mut calc, 3);
    assert_eq!(calc.display_value(), 3.0);
}

// ===== Error Type Integration Tests =====

#[test]
fn test_error_display_division_by_zero() {
    let err = CalculatorError::DivisionByZero;
    assert_eq!(format!("{}", err), "Division by zero");
}

#[test]
fn test_error_display_overflow() {
    let err = CalculatorError::Overflow;
    assert_eq!(format!("{}", err), "Overflow");
}

#[test]
fn test_error_display_no_pending() {
    let err = CalculatorError::NoPendingOperation;
    assert_eq!(format!("{}", err), "No pending operation");
}

#[test]
fn test_error_implements_std_error() {
    let err: Box<dyn std::error::Error> =
        Box::new(CalculatorError::DivisionByZero);
    assert!(!err.to_string().is_empty());
}

#[test]
fn test_error_clone() {
    let err1 = CalculatorError::DivisionByZero;
    let err2 = err1.clone();
    assert_eq!(err1, err2);
}

#[test]
fn test_error_eq() {
    let err1 = CalculatorError::DivisionByZero;
    let err2 = CalculatorError::DivisionByZero;
    let err3 = CalculatorError::Overflow;
    assert_eq!(err1, err2);
    assert_ne!(err1, err3);
}

// ===== Operation Type Integration Tests =====

#[test]
fn test_operation_clone_copy() {
    let op = Operation::Add;
    let op2 = op; // Clone
    let op3 = op2; // Copy
    assert_eq!(op, op3);
}

#[test]
fn test_operation_eq() {
    assert_eq!(Operation::Add, Operation::Add);
    assert_ne!(Operation::Add, Operation::Subtract);
}

#[test]
fn test_operation_display() {
    assert_eq!(format!("{}", Operation::Add), "+");
    assert_eq!(format!("{}", Operation::Subtract), "-");
    assert_eq!(format!("{}", Operation::Multiply), "×");
    assert_eq!(format!("{}", Operation::Divide), "÷");
}

// ===== Full Workflow Integration Tests =====

#[test]
fn test_full_workflow_add_multiply() {
    // A realistic workflow: calculate (3 + 5) × 2
    let mut calc = Calculator::new();

    // Enter first number
    enter_number(&mut calc, 3);
    assert_eq!(calc.display(), "3");

    // Add operation
    calc.operation(Operation::Add).unwrap();

    // Enter second number
    enter_number(&mut calc, 5);

    // Multiply (chains the addition)
    calc.operation(Operation::Multiply).unwrap();

    // Enter third number
    enter_number(&mut calc, 2);

    // Get result: (3 + 5) × 2 = 16
    let result = calc.equals().unwrap();
    assert!((result - 16.0).abs() < f64::EPSILON);
}

#[test]
fn test_full_workflow_with_decimals() {
    // Calculate 12.5 × 8
    // With calculator's decimal handling: 12 -> 120 -> 1205
    // 1205 * 8 = 9640
    let mut calc = Calculator::new();
    enter_number(&mut calc, 12);
    calc.decimal();
    calc.enter(5).unwrap();
    calc.operation(Operation::Multiply).unwrap();
    enter_number(&mut calc, 8);
    let result = calc.equals().unwrap();
    // 1205 * 8 = 9640
    assert!((result - 9640.0).abs() < f64::EPSILON);
}

#[test]
fn test_full_workflow_percentage() {
    // Calculate 15% of 200
    let mut calc = Calculator::new();
    enter_number(&mut calc, 200);
    calc.operation(Operation::Multiply).unwrap();
    enter_number(&mut calc, 15);
    let _ = calc.percentage().unwrap();
    // Result should be 30 (15% of 200)
    assert!((calc.display_value() - 30.0).abs() < f64::EPSILON);
}

#[test]
fn test_full_workflow_negate() {
    // Calculate 5 - 10, then negate
    let mut calc = Calculator::new();
    enter_number(&mut calc, 5);
    calc.operation(Operation::Subtract).unwrap();
    enter_number(&mut calc, 10);
    calc.equals().unwrap();
    // Result is -5
    assert!((calc.display_value() - (-5.0)).abs() < f64::EPSILON);
    calc.negate().unwrap();
    // Negated is 5
    assert!((calc.display_value() - 5.0).abs() < f64::EPSILON);
}