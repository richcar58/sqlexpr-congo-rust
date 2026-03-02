// Comprehensive tests for SQL Expression Evaluator
use std::collections::HashMap;
use parser::{evaluate, RuntimeValue, EvalError};

// ============================================================================
// LITERAL TESTS
// ============================================================================

#[test]
fn test_eval_boolean_literal_true() {
    let result = evaluate("TRUE", &HashMap::new());
    if let Err(e) = &result {
        eprintln!("Parse error: {}", e);
    }
    assert_eq!(result.unwrap(), true);
}

#[test]
fn test_eval_boolean_literal_false() {
    let result = evaluate("FALSE", &HashMap::new());
    if let Err(e) = &result {
        eprintln!("Parse error: {}", e);
    }
    assert_eq!(result.unwrap(), false);
}

// ============================================================================
// VARIABLE BINDING TESTS
// ============================================================================

#[test]
fn test_eval_boolean_variable() {
    let mut bindings = HashMap::new();
    bindings.insert("active".to_string(), RuntimeValue::Boolean(true));

    let result = evaluate("active", &bindings);
    if let Err(e) = &result {
        eprintln!("Parse error: {}", e);
    }
    assert_eq!(result.unwrap(), true);
}

#[test]
fn test_unbound_variable_error() {
    let result = evaluate("missing", &HashMap::new());
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), EvalError::UnboundVariable { .. }));
}

#[test]
fn test_wrong_type_boolean_variable() {
    let mut bindings = HashMap::new();
    bindings.insert("x".to_string(), RuntimeValue::Integer(42));

    let result = evaluate("x", &bindings);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), EvalError::TypeError { .. }));
}

// ============================================================================
// ARITHMETIC TESTS
// ============================================================================

#[test]
fn test_arithmetic_addition_integers() {
    let mut bindings = HashMap::new();
    bindings.insert("a".to_string(), RuntimeValue::Integer(10));
    bindings.insert("b".to_string(), RuntimeValue::Integer(20));

    let result = evaluate("a + b = 30", &bindings);
    if let Err(e) = &result {
        eprintln!("Parse error: {}", e);
    }
    assert_eq!(result.unwrap(), true);
}

#[test]
fn test_arithmetic_subtraction() {
    let mut bindings = HashMap::new();
    bindings.insert("a".to_string(), RuntimeValue::Integer(50));
    bindings.insert("b".to_string(), RuntimeValue::Integer(20));

    let result = evaluate("a - b = 30", &bindings);
    if let Err(e) = &result {
        eprintln!("Parse error: {}", e);
    }
    assert_eq!(result.unwrap(), true);
}

#[test]
fn test_arithmetic_multiplication() {
    let mut bindings = HashMap::new();
    bindings.insert("a".to_string(), RuntimeValue::Integer(5));
    bindings.insert("b".to_string(), RuntimeValue::Integer(6));

    let result = evaluate("a * b = 30", &bindings);
    if let Err(e) = &result {
        eprintln!("Parse error: {}", e);
    }
    assert_eq!(result.unwrap(), true);
}

#[test]
fn test_arithmetic_mixed_types_coercion() {
    let mut bindings = HashMap::new();
    bindings.insert("a".to_string(), RuntimeValue::Integer(10));
    bindings.insert("b".to_string(), RuntimeValue::Float(5.5));

    let result = evaluate("a + b = 15.5", &bindings);
    if let Err(e) = &result {
        eprintln!("Parse error: {}", e);
    }
    assert_eq!(result.unwrap(), true);
}

#[test]
fn test_division_returns_float() {
    let mut bindings = HashMap::new();
    bindings.insert("a".to_string(), RuntimeValue::Integer(10));
    bindings.insert("b".to_string(), RuntimeValue::Integer(4));

    let result = evaluate("a / b = 2.5", &bindings);
    if let Err(e) = &result {
        eprintln!("Parse error: {}", e);
    }
    assert_eq!(result.unwrap(), true);
}

#[test]
fn test_division_by_zero_error() {
    let mut bindings = HashMap::new();
    bindings.insert("a".to_string(), RuntimeValue::Integer(10));

    let result = evaluate("a / 0 = 5", &bindings);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), EvalError::DivisionByZero { .. }));
}

#[test]
fn test_modulo_operation() {
    let mut bindings = HashMap::new();
    bindings.insert("a".to_string(), RuntimeValue::Integer(10));
    bindings.insert("b".to_string(), RuntimeValue::Integer(3));

    let result = evaluate("a % b = 1", &bindings);
    if let Err(e) = &result {
        eprintln!("Parse error: {}", e);
    }
    assert_eq!(result.unwrap(), true);
}

#[test]
fn test_arithmetic_null_error() {
    let mut bindings = HashMap::new();
    bindings.insert("a".to_string(), RuntimeValue::Integer(10));
    bindings.insert("b".to_string(), RuntimeValue::Null);

    let result = evaluate("a + b = 10", &bindings);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), EvalError::NullInOperation { .. }));
}

#[test]
fn test_unary_minus() {
    let mut bindings = HashMap::new();
    bindings.insert("a".to_string(), RuntimeValue::Integer(10));

    let result = evaluate("-a = -10", &bindings);
    if let Err(e) = &result {
        eprintln!("Parse error: {}", e);
    }
    assert_eq!(result.unwrap(), true);
}

#[test]
fn test_unary_plus() {
    let mut bindings = HashMap::new();
    bindings.insert("a".to_string(), RuntimeValue::Integer(10));

    let result = evaluate("+a = 10", &bindings);
    if let Err(e) = &result {
        eprintln!("Parse error: {}", e);
    }
    assert_eq!(result.unwrap(), true);
}

#[test]
fn test_precedence_no_parens() {
    let mut bindings = HashMap::new();
    bindings.insert("a".to_string(), RuntimeValue::Integer(14));
    bindings.insert("b".to_string(), RuntimeValue::Integer(6));

    let result = evaluate("2 + 3 * 4 = a", &bindings);
    if let Err(e) = &result {
        eprintln!("Parse error: {}", e);
    }
    assert_eq!(result.unwrap(), true);
}

#[test]
fn test_precedence_with_parens() {
    let mut bindings = HashMap::new();
    bindings.insert("a".to_string(), RuntimeValue::Integer(20));
    bindings.insert("b".to_string(), RuntimeValue::Integer(6));

    let result = evaluate("(2 + 3) * 4 = a", &bindings);
    if let Err(e) = &result {
        eprintln!("Parse error: {}", e);
    }
    assert_eq!(result.unwrap(), true);
}

// ============================================================================
// COMPARISON TESTS
// ============================================================================

#[test]
fn test_comparison_integers() {
    let mut bindings = HashMap::new();
    bindings.insert("age".to_string(), RuntimeValue::Integer(25));

    assert_eq!(evaluate("age > 18", &bindings).unwrap(), true);
    assert_eq!(evaluate("age < 30", &bindings).unwrap(), true);
    assert_eq!(evaluate("age >= 25", &bindings).unwrap(), true);
    assert_eq!(evaluate("age <= 25", &bindings).unwrap(), true);
    assert_eq!(evaluate("age = 25", &bindings).unwrap(), true);
    assert_eq!(evaluate("age <> 30", &bindings).unwrap(), true);
}

#[test]
fn test_comparison_floats() {
    let mut bindings = HashMap::new();
    bindings.insert("price".to_string(), RuntimeValue::Float(19.99));

    assert_eq!(evaluate("price > 10.0", &bindings).unwrap(), true);
    assert_eq!(evaluate("price < 20.0", &bindings).unwrap(), true);
}

#[test]
fn test_comparison_strings() {
    let mut bindings = HashMap::new();
    bindings.insert("name".to_string(), RuntimeValue::String("John".to_string()));

    assert_eq!(evaluate("name > 'Alice'", &bindings).unwrap(), true);
    assert_eq!(evaluate("name < 'Zoe'", &bindings).unwrap(), true);
    assert_eq!(evaluate("name = 'John'", &bindings).unwrap(), true);
}

#[test]
fn test_comparison_type_mismatch() {
    let mut bindings = HashMap::new();
    bindings.insert("age".to_string(), RuntimeValue::Integer(25));
    bindings.insert("name".to_string(), RuntimeValue::String("John".to_string()));

    let result = evaluate("age > name", &bindings);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), EvalError::TypeError { .. }));
}

#[test]
fn test_equality_boolean() {
    let mut bindings = HashMap::new();
    bindings.insert("flag1".to_string(), RuntimeValue::Boolean(true));
    bindings.insert("flag2".to_string(), RuntimeValue::Boolean(true));

    assert_eq!(evaluate("flag1 = flag2", &bindings).unwrap(), true);
}

// ============================================================================
// LIKE TESTS
// ============================================================================

#[test]
fn test_like_wildcard_percent() {
    let mut bindings = HashMap::new();
    bindings.insert("email".to_string(),
        RuntimeValue::String("user@example.com".to_string()));

    assert_eq!(evaluate("email LIKE '%@example.com'", &bindings).unwrap(), true);
    assert_eq!(evaluate("email LIKE 'user%'", &bindings).unwrap(), true);
    assert_eq!(evaluate("email LIKE '%example%'", &bindings).unwrap(), true);
    assert_eq!(evaluate("email LIKE '%nobody%'", &bindings).unwrap(), false);
}

#[test]
fn test_like_wildcard_underscore() {
    let mut bindings = HashMap::new();
    bindings.insert("code".to_string(), RuntimeValue::String("A1B".to_string()));

    assert_eq!(evaluate("code LIKE 'A_B'", &bindings).unwrap(), true);
    assert_eq!(evaluate("code LIKE 'A__'", &bindings).unwrap(), true);  // A__ matches A1B (A + any 2 chars)
    assert_eq!(evaluate("code LIKE '___'", &bindings).unwrap(), true);
    assert_eq!(evaluate("code LIKE 'A____'", &bindings).unwrap(), false);  // Need 5 chars total
}

#[test]
fn test_like_with_escape() {
    let mut bindings = HashMap::new();
    bindings.insert("text".to_string(),
        RuntimeValue::String("50%".to_string()));

    let result = evaluate("text LIKE '50\\%' ESCAPE '\\'", &bindings);
    if let Err(e) = &result {
        eprintln!("Parse error: {}", e);
    }
    assert_eq!(result.unwrap(), true);
}

#[test]
fn test_not_like() {
    let mut bindings = HashMap::new();
    bindings.insert("name".to_string(), RuntimeValue::String("John".to_string()));

    assert_eq!(evaluate("name NOT LIKE '%test%'", &bindings).unwrap(), true);
}

// ============================================================================
// BETWEEN TESTS
// ============================================================================

#[test]
fn test_between_integers() {
    let mut bindings = HashMap::new();
    bindings.insert("age".to_string(), RuntimeValue::Integer(25));

    assert_eq!(evaluate("age BETWEEN 18 AND 30", &bindings).unwrap(), true);
    assert_eq!(evaluate("age BETWEEN 30 AND 40", &bindings).unwrap(), false);
}

#[test]
fn test_between_inclusive() {
    let mut bindings = HashMap::new();
    bindings.insert("x".to_string(), RuntimeValue::Integer(5));

    assert_eq!(evaluate("x BETWEEN 5 AND 10", &bindings).unwrap(), true);
    assert_eq!(evaluate("x BETWEEN 1 AND 5", &bindings).unwrap(), true);
}

#[test]
fn test_not_between() {
    let mut bindings = HashMap::new();
    bindings.insert("age".to_string(), RuntimeValue::Integer(15));

    assert_eq!(evaluate("age NOT BETWEEN 18 AND 65", &bindings).unwrap(), true);
}

#[test]
fn test_between_strings() {
    let mut bindings = HashMap::new();
    bindings.insert("name".to_string(), RuntimeValue::String("John".to_string()));

    assert_eq!(evaluate("name BETWEEN 'Alice' AND 'Zoe'", &bindings).unwrap(), true);
}

// ============================================================================
// IN TESTS
// ============================================================================

#[test]
fn test_in_integers() {
    let mut bindings = HashMap::new();
    bindings.insert("status".to_string(), RuntimeValue::Integer(2));

    assert_eq!(evaluate("status IN (1, 2, 3)", &bindings).unwrap(), true);
    assert_eq!(evaluate("status IN (4, 5, 6)", &bindings).unwrap(), false);
}

#[test]
fn test_in_strings() {
    let mut bindings = HashMap::new();
    bindings.insert("state".to_string(),
        RuntimeValue::String("active".to_string()));

    let result = evaluate("state IN ('active', 'pending')", &bindings);
    if let Err(e) = &result {
        eprintln!("Parse error: {}", e);
    }
    assert_eq!(result.unwrap(), true);
}

#[test]
fn test_not_in() {
    let mut bindings = HashMap::new();
    bindings.insert("x".to_string(), RuntimeValue::Integer(5));

    assert_eq!(evaluate("x NOT IN (1, 2, 3)", &bindings).unwrap(), true);
}

#[test]
fn test_in_mixed_numeric_types() {
    let mut bindings = HashMap::new();
    bindings.insert("x".to_string(), RuntimeValue::Float(5.0));

    assert_eq!(evaluate("x IN (1, 2, 5)", &bindings).unwrap(), true);
}

// Automatic numeric type coercion in IN operator (e.g., Integer to Float)
#[test]
fn test_error_in_operator_with_compatible_numeric_types() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Float(8.0));

    assert_eq!(evaluate("x IN (6, 7, 8)", &map).unwrap(), true);
}

// Automatic numeric type coercion in IN operator (e.g., Integer to Float)
#[test]
fn test_error_in_operator_with_compatible_numeric_types_2() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(8));

    assert_eq!(evaluate("x IN (6.1, .2, 8.0)", &map).unwrap(), true);
}

// ============================================================================
// IS NULL TESTS
// ============================================================================

#[test]
fn test_is_null() {
    let mut bindings = HashMap::new();
    bindings.insert("value".to_string(), RuntimeValue::Null);

    assert_eq!(evaluate("value IS NULL", &bindings).unwrap(), true);
}

#[test]
fn test_is_not_null() {
    let mut bindings = HashMap::new();
    bindings.insert("value".to_string(), RuntimeValue::Integer(42));

    assert_eq!(evaluate("value IS NOT NULL", &bindings).unwrap(), true);
}

#[test]
fn test_is_null_with_non_null() {
    let mut bindings = HashMap::new();
    bindings.insert("value".to_string(), RuntimeValue::Integer(42));

    assert_eq!(evaluate("value IS NULL", &bindings).unwrap(), false);
}

// ============================================================================
// BOOLEAN LOGIC TESTS
// ============================================================================

#[test]
fn test_and_operator() {
    let mut bindings = HashMap::new();
    bindings.insert("a".to_string(), RuntimeValue::Boolean(true));
    bindings.insert("b".to_string(), RuntimeValue::Boolean(true));

    assert_eq!(evaluate("a AND b", &bindings).unwrap(), true);

    bindings.insert("b".to_string(), RuntimeValue::Boolean(false));
    assert_eq!(evaluate("a AND b", &bindings).unwrap(), false);
}

#[test]
fn test_or_operator() {
    let mut bindings = HashMap::new();
    bindings.insert("a".to_string(), RuntimeValue::Boolean(false));
    bindings.insert("b".to_string(), RuntimeValue::Boolean(true));

    assert_eq!(evaluate("a OR b", &bindings).unwrap(), true);

    bindings.insert("b".to_string(), RuntimeValue::Boolean(false));
    assert_eq!(evaluate("a OR b", &bindings).unwrap(), false);
}

#[test]
fn test_not_operator() {
    let mut bindings = HashMap::new();
    bindings.insert("active".to_string(), RuntimeValue::Boolean(false));

    assert_eq!(evaluate("NOT active", &bindings).unwrap(), true);
}

#[test]
fn test_operator_precedence() {
    let mut bindings = HashMap::new();
    bindings.insert("a".to_string(), RuntimeValue::Boolean(true));
    bindings.insert("b".to_string(), RuntimeValue::Boolean(false));
    bindings.insert("c".to_string(), RuntimeValue::Boolean(true));

    // a OR b AND c should be a OR (b AND c) = true OR false = true
    assert_eq!(evaluate("a OR b AND c", &bindings).unwrap(), true);
}

// ============================================================================
// COMPLEX INTEGRATION TESTS
// ============================================================================

#[test]
fn test_complex_expression_real_world() {
    let mut bindings = HashMap::new();
    bindings.insert("age".to_string(), RuntimeValue::Integer(25));
    bindings.insert("status".to_string(),
        RuntimeValue::String("active".to_string()));
    bindings.insert("score".to_string(), RuntimeValue::Float(85.5));

    let expr = "age >= 18 AND status = 'active' AND score > 80.0";
    assert_eq!(evaluate(expr, &bindings).unwrap(), true);
}

#[test]
fn test_complex_expression_with_arithmetic() {
    let mut bindings = HashMap::new();
    bindings.insert("revenue".to_string(), RuntimeValue::Float(1000.0));
    bindings.insert("cost".to_string(), RuntimeValue::Float(700.0));

    let expr = "(revenue - cost) / revenue * 100 >= 20";
    assert_eq!(evaluate(expr, &bindings).unwrap(), true);
}

#[test]
fn test_complex_nested_boolean() {
    let mut bindings = HashMap::new();
    bindings.insert("a".to_string(), RuntimeValue::Boolean(true));
    bindings.insert("b".to_string(), RuntimeValue::Boolean(false));
    bindings.insert("c".to_string(), RuntimeValue::Boolean(true));
    bindings.insert("d".to_string(), RuntimeValue::Boolean(false));

    let expr = "(a OR b) AND (c OR d)";
    assert_eq!(evaluate(expr, &bindings).unwrap(), true);
}

#[test]
fn test_complex_expression_crazy() {
    let mut bindings = HashMap::new();
    bindings.insert("age".to_string(), RuntimeValue::Integer(25));
    bindings.insert("status".to_string(), RuntimeValue::String("active".to_string()));
    bindings.insert("score".to_string(), RuntimeValue::Float(85.5));
    bindings.insert("like_bananas".to_string(), RuntimeValue::Boolean(false));
    bindings.insert("height".to_string(), RuntimeValue::Float(5.9));
    bindings.insert("fastest_speed".to_string(), RuntimeValue::Integer(145));
    bindings.insert("yesterday_score".to_string(), RuntimeValue::Float(8.5));


    let expr = "(age >= 30 OR (age < 30 AND yesterday_score < 10)) AND status = 'active' AND score > 80.0 AND NOT like_bananas AND (height BETWEEN 5.5 AND 6.5) AND (fastest_speed / 2 > 70)";
    assert_eq!(evaluate(expr, &bindings).unwrap(), true);
}

// ============================================================================
// NEGATIVE TESTS (ERROR CASES)
// ============================================================================

#[test]
fn test_error_null_in_arithmetic() {
    let mut bindings = HashMap::new();
    bindings.insert("x".to_string(), RuntimeValue::Null);

    let result = evaluate("x + 5 = 10", &bindings);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), EvalError::NullInOperation { .. }));
}

#[test]
fn test_error_null_in_comparison() {
    let mut bindings = HashMap::new();
    bindings.insert("x".to_string(), RuntimeValue::Null);

    let result = evaluate("x > 5", &bindings);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), EvalError::NullInOperation { .. }));
}

#[test]
fn test_error_type_mismatch_in_comparison() {
    let mut bindings = HashMap::new();
    bindings.insert("age".to_string(), RuntimeValue::Integer(25));
    bindings.insert("name".to_string(), RuntimeValue::String("John".to_string()));

    let result = evaluate("age = name", &bindings);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), EvalError::TypeError { .. }));
}

#[test]
fn test_error_boolean_in_arithmetic() {
    let mut bindings = HashMap::new();
    bindings.insert("flag".to_string(), RuntimeValue::Boolean(true));

    let result = evaluate("flag + 5 = 10", &bindings);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), EvalError::TypeError { .. }));
}

#[test]
fn test_error_string_in_arithmetic() {
    let mut bindings = HashMap::new();
    bindings.insert("name".to_string(), RuntimeValue::String("John".to_string()));

    let result = evaluate("name + 5 = 10", &bindings);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), EvalError::TypeError { .. }));
}

#[test]
fn test_error_like_on_non_string() {
    let mut bindings = HashMap::new();
    bindings.insert("x".to_string(), RuntimeValue::Integer(42));

    let result = evaluate("x LIKE '%test%'", &bindings);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), EvalError::TypeError { .. }));
}

#[test]
fn test_error_null_in_between() {
    let mut bindings = HashMap::new();
    bindings.insert("x".to_string(), RuntimeValue::Null);

    let result = evaluate("x BETWEEN 1 AND 10", &bindings);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), EvalError::NullInOperation { .. }));
}

#[test]
fn test_error_null_in_in() {
    let mut bindings = HashMap::new();
    bindings.insert("x".to_string(), RuntimeValue::Null);

    let result = evaluate("x IN (1, 2, 3)", &bindings);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), EvalError::NullInOperation { .. }));
}

// ============================================================================
// POSITIVE TESTS - Valid evaluations that should succeed
// ============================================================================

#[test]
fn test_simple_integer_comparison() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(42));

    assert_eq!(evaluate("x > 10", &map).unwrap(), true);
    assert_eq!(evaluate("x < 100", &map).unwrap(), true);
    assert_eq!(evaluate("x = 42", &map).unwrap(), true);
    assert_eq!(evaluate("x != 10", &map).unwrap(), true);
    assert_eq!(evaluate("x <> 10", &map).unwrap(), true);
}

#[test]
fn test_float_comparison() {
    let mut map = HashMap::new();
    map.insert("price".to_string(), RuntimeValue::Float(19.99));

    assert_eq!(evaluate("price > 10.0", &map).unwrap(), true);
    assert_eq!(evaluate("price < 20.0", &map).unwrap(), true);
    assert_eq!(evaluate("price >= 19.99", &map).unwrap(), true);
    assert_eq!(evaluate("price <= 19.99", &map).unwrap(), true);
}

#[test]
fn test_mixed_numeric_comparison() {
    let mut map = HashMap::new();
    map.insert("int_val".to_string(), RuntimeValue::Integer(10));
    map.insert("float_val".to_string(), RuntimeValue::Float(10.5));

    assert_eq!(evaluate("int_val < float_val", &map).unwrap(), true);
    assert_eq!(evaluate("float_val > int_val", &map).unwrap(), true);
    assert_eq!(evaluate("int_val < 10.5", &map).unwrap(), true);
}

#[test]
fn test_string_comparison() {
    let mut map = HashMap::new();
    map.insert("name".to_string(), RuntimeValue::String("Alice".to_string()));

    assert_eq!(evaluate("name = 'Alice'", &map).unwrap(), true);
    assert_eq!(evaluate("name != 'Bob'", &map).unwrap(), true);
    assert_eq!(evaluate("name > 'Aaron'", &map).unwrap(), true);
    assert_eq!(evaluate("name < 'Zoe'", &map).unwrap(), true);
}

#[test]
fn test_boolean_variables() {
    let mut map = HashMap::new();
    map.insert("active".to_string(), RuntimeValue::Boolean(true));
    map.insert("deleted".to_string(), RuntimeValue::Boolean(false));

    assert_eq!(evaluate("active", &map).unwrap(), true);
    assert_eq!(evaluate("NOT deleted", &map).unwrap(), true);
    assert_eq!(evaluate("active AND NOT deleted", &map).unwrap(), true);
}

#[test]
fn test_boolean_literals() {
    let map = HashMap::new();

    assert_eq!(evaluate("TRUE", &map).unwrap(), true);
    assert_eq!(evaluate("FALSE", &map).unwrap(), false);
    assert_eq!(evaluate("NOT FALSE", &map).unwrap(), true);
    assert_eq!(evaluate("TRUE AND TRUE", &map).unwrap(), true);
    assert_eq!(evaluate("TRUE OR FALSE", &map).unwrap(), true);
}

#[test]
fn test_logical_and() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(15));
    map.insert("y".to_string(), RuntimeValue::Integer(25));

    assert_eq!(evaluate("x > 10 AND y < 30", &map).unwrap(), true);
    assert_eq!(evaluate("x > 10 AND y > 30", &map).unwrap(), false);
}

#[test]
fn test_logical_or() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(5));
    map.insert("y".to_string(), RuntimeValue::Integer(25));

    assert_eq!(evaluate("x > 10 OR y > 20", &map).unwrap(), true);
    assert_eq!(evaluate("x > 10 OR y < 20", &map).unwrap(), false);
}

#[test]
fn test_logical_not() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(15));

    assert_eq!(evaluate("NOT (x < 10)", &map).unwrap(), true);
    assert_eq!(evaluate("NOT (x > 10)", &map).unwrap(), false);
}

#[test]
fn test_complex_boolean_expression() {
    let mut map = HashMap::new();
    map.insert("age".to_string(), RuntimeValue::Integer(25));
    map.insert("status".to_string(), RuntimeValue::String("active".to_string()));
    map.insert("premium".to_string(), RuntimeValue::Boolean(true));

    assert_eq!(
        evaluate("age >= 18 AND status = 'active' AND premium", &map).unwrap(),
        true
    );
}

#[test]
fn test_arithmetic_in_comparison() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), RuntimeValue::Integer(10));
    map.insert("b".to_string(), RuntimeValue::Integer(5));

    assert_eq!(evaluate("(a + b) > 12", &map).unwrap(), true);
    assert_eq!(evaluate("(a - b) < 10", &map).unwrap(), true);
    assert_eq!(evaluate("(a * b) = 50", &map).unwrap(), true);
}

#[test]
fn test_division_always_returns_float() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(10));
    map.insert("y".to_string(), RuntimeValue::Integer(3));

    // Division of integers should return float
    assert_eq!(evaluate("(x / y) > 3.0", &map).unwrap(), true);
    assert_eq!(evaluate("(x / y) < 4.0", &map).unwrap(), true);
}

#[test]
fn test_modulo_operation_2() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(17));

    assert_eq!(evaluate("(x % 5) = 2", &map).unwrap(), true);
    assert_eq!(evaluate("(x % 3) = 2", &map).unwrap(), true);
}

#[test]
fn test_unary_minus_2() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(10));

    assert_eq!(evaluate("(-x) = -10", &map).unwrap(), true);
    assert_eq!(evaluate("(-x) < 0", &map).unwrap(), true);
}

#[test]
fn test_is_null_2() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Null);
    map.insert("y".to_string(), RuntimeValue::Integer(42));

    assert_eq!(evaluate("x IS NULL", &map).unwrap(), true);
    assert_eq!(evaluate("y IS NULL", &map).unwrap(), false);
}

#[test]
fn test_is_not_null_2() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Null);
    map.insert("y".to_string(), RuntimeValue::Integer(42));

    assert_eq!(evaluate("x IS NOT NULL", &map).unwrap(), false);
    assert_eq!(evaluate("y IS NOT NULL", &map).unwrap(), true);
}

#[test]
fn test_like_basic() {
    let mut map = HashMap::new();
    map.insert("email".to_string(), RuntimeValue::String("user@example.com".to_string()));

    assert_eq!(evaluate("email LIKE '%@example.com'", &map).unwrap(), true);
    assert_eq!(evaluate("email LIKE 'user@%'", &map).unwrap(), true);
    assert_eq!(evaluate("email LIKE '%@%'", &map).unwrap(), true);
    assert_eq!(evaluate("email LIKE 'user@example.com'", &map).unwrap(), true);
}

#[test]
fn test_like_underscore() {
    let mut map = HashMap::new();
    map.insert("code".to_string(), RuntimeValue::String("ABC123".to_string()));

    assert_eq!(evaluate("code LIKE 'ABC___'", &map).unwrap(), true);
    assert_eq!(evaluate("code LIKE 'ABC__'", &map).unwrap(), false);
}

#[test]
fn test_like_escape() {
    let mut map = HashMap::new();
    map.insert("text".to_string(), RuntimeValue::String("50%".to_string()));

    assert_eq!(evaluate("text LIKE '50!%' ESCAPE '!'", &map).unwrap(), true);
}

#[test]
fn test_not_like_2() {
    let mut map = HashMap::new();
    map.insert("email".to_string(), RuntimeValue::String("user@gmail.com".to_string()));

    assert_eq!(evaluate("email NOT LIKE '%@example.com'", &map).unwrap(), true);
}

#[test]
fn test_between() {
    let mut map = HashMap::new();
    map.insert("age".to_string(), RuntimeValue::Integer(25));

    assert_eq!(evaluate("age BETWEEN 18 AND 65", &map).unwrap(), true);
    assert_eq!(evaluate("age BETWEEN 30 AND 40", &map).unwrap(), false);
}

#[test]
fn test_not_between_2() {
    let mut map = HashMap::new();
    map.insert("score".to_string(), RuntimeValue::Integer(95));

    assert_eq!(evaluate("score NOT BETWEEN 0 AND 59", &map).unwrap(), true);
    assert_eq!(evaluate("score NOT BETWEEN 90 AND 100", &map).unwrap(), false);
}

#[test]
fn test_in_operator() {
    let mut map = HashMap::new();
    map.insert("status".to_string(), RuntimeValue::String("active".to_string()));

    assert_eq!(evaluate("status IN ('active', 'pending')", &map).unwrap(), true);
    assert_eq!(evaluate("status IN ('inactive', 'deleted')", &map).unwrap(), false);
}

#[test]
fn test_not_in_operator() {
    let mut map = HashMap::new();
    map.insert("role".to_string(), RuntimeValue::String("user".to_string()));

    assert_eq!(evaluate("role NOT IN ('admin', 'moderator')", &map).unwrap(), true);
    assert_eq!(evaluate("role NOT IN ('user', 'guest')", &map).unwrap(), false);
}

#[test]
fn test_numeric_literals() {
    let map = HashMap::new();

    assert_eq!(evaluate("42 > 10", &map).unwrap(), true);
    assert_eq!(evaluate("3.14 < 4.0", &map).unwrap(), true);
    assert_eq!(evaluate("0xFF = 255", &map).unwrap(), true); // Hex
    assert_eq!(evaluate("010 = 8", &map).unwrap(), true);    // Octal
}

#[test]
fn test_parenthesized_expressions() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(5));
    map.insert("y".to_string(), RuntimeValue::Integer(10));

    assert_eq!(evaluate("(x + y) * 2 > 25", &map).unwrap(), true);
    assert_eq!(evaluate("((x > 0) AND (y > 0))", &map).unwrap(), true);
}

#[test]
fn test_short_circuit_and() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(5));

    // First condition is false, so second shouldn't cause unbound variable error
    // (though in this case both are evaluated since both vars are bound)
    assert_eq!(evaluate("x > 10 AND x < 20", &map).unwrap(), false);
}

#[test]
fn test_short_circuit_or() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(15));

    // First condition is true, so result is true regardless of second
    assert_eq!(evaluate("x > 10 OR x < 5", &map).unwrap(), true);
}

#[test]
fn test_comparison_two_valueexpr() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(10));
    map.insert("y".to_string(), RuntimeValue::Integer(5));
    map.insert("a".to_string(), RuntimeValue::Integer(4));
    map.insert("b".to_string(), RuntimeValue::Integer(3));

    // Compare two arithmetic expressions
    assert_eq!(evaluate("a + x + 6 = 12 + b + y", &map).unwrap(), true);
}

// ============================================================================
// NEGATIVE TESTS - Errors that should occur
// ============================================================================

#[test]
fn test_error_unbound_variable() {
    let map = HashMap::new();

    let result = evaluate("x > 10", &map);
    assert!(result.is_err());
    match result.unwrap_err() {
        EvalError::UnboundVariable { name } => assert_eq!(name, "x"),
        _ => panic!("Expected UnboundVariable error"),
    }
}

#[test]
fn test_error_null_in_arithmetic_2() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Null);
    map.insert("y".to_string(), RuntimeValue::Integer(10));

    let result = evaluate("(x + y) > 0", &map);
    assert!(result.is_err());
    match result.unwrap_err() {
        EvalError::NullInOperation { operation, .. } => {
            assert_eq!(operation, "addition");
        }
        _ => panic!("Expected NullInOperation error"),
    }
}

#[test]
fn test_error_null_in_comparison_2() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Null);

    let result = evaluate("x > 10", &map);
    assert!(result.is_err());
    match result.unwrap_err() {
        EvalError::NullInOperation { operation, .. } => {
            assert_eq!(operation, "GreaterThan");
        }
        _ => panic!("Expected NullInOperation error"),
    }
}

#[test]
fn test_error_null_in_equality() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Null);

    let result = evaluate("x = 10", &map);
    assert!(result.is_err());
    match result.unwrap_err() {
        EvalError::NullInOperation { operation, .. } => {
            assert_eq!(operation, "Equal");
        }
        _ => panic!("Expected NullInOperation error"),
    }
}

#[test]
fn test_error_null_in_like() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Null);

    let result = evaluate("x LIKE '%test%'", &map);
    assert!(result.is_err());
    match result.unwrap_err() {
        EvalError::NullInOperation { operation, .. } => {
            assert_eq!(operation, "LIKE");
        }
        _ => panic!("Expected NullInOperation error"),
    }
}

#[test]
fn test_error_null_in_between_2() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Null);

    let result = evaluate("x BETWEEN 10 AND 20", &map);
    assert!(result.is_err());
    match result.unwrap_err() {
        EvalError::NullInOperation { operation, .. } => {
            assert_eq!(operation, "BETWEEN");
        }
        _ => panic!("Expected NullInOperation error"),
    }
}

#[test]
fn test_error_null_in_in_operator() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Null);

    let result = evaluate("x IN ('a', 'b')", &map);
    assert!(result.is_err());
    match result.unwrap_err() {
        EvalError::NullInOperation { operation, .. } => {
            assert_eq!(operation, "IN");
        }
        _ => panic!("Expected NullInOperation error"),
    }
}

#[test]
fn test_error_division_by_zero() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(10));

    let result = evaluate("x / 0 > 0", &map);
    assert!(result.is_err());
    match result.unwrap_err() {
        EvalError::DivisionByZero { .. } => {}
        _ => panic!("Expected DivisionByZero error"),
    }
}

#[test]
fn test_error_modulo_by_zero() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(10));

    let result = evaluate("(x % 0) = 0", &map);
    assert!(result.is_err());
    match result.unwrap_err() {
        EvalError::DivisionByZero { .. } => {}
        _ => panic!("Expected DivisionByZero error"),
    }
}

#[test]
fn test_error_type_mismatch_in_arithmetic() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::String("hello".to_string()));

    let result = evaluate("(x + 10) > 0", &map);
    assert!(result.is_err());
    match result.unwrap_err() {
        EvalError::TypeError { operation, .. } => {
            assert_eq!(operation, "addition");
        }
        _ => panic!("Expected TypeError"),
    }
}

#[test]
fn test_error_type_mismatch_in_comparison_2() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::String("hello".to_string()));
    map.insert("y".to_string(), RuntimeValue::Integer(42));

    let result = evaluate("x > y", &map);
    assert!(result.is_err());
    match result.unwrap_err() {
        EvalError::TypeError { operation, .. } => {
            assert_eq!(operation, "GreaterThan");
        }
        _ => panic!("Expected TypeError"),
    }
}

#[test]
fn test_error_boolean_variable_with_wrong_type() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(42));

    let result = evaluate("x AND TRUE", &map);
    assert!(result.is_err());
    match result.unwrap_err() {
        EvalError::TypeError { operation, expected, .. } => {
            assert_eq!(operation, "AND");
            assert_eq!(expected, "boolean");
        }
        _ => panic!("Expected TypeError"),
    }
}

#[test]
fn test_error_like_with_non_string() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(42));

    let result = evaluate("x LIKE '%42%'", &map);
    assert!(result.is_err());
    match result.unwrap_err() {
        EvalError::TypeError { operation, expected, .. } => {
            assert_eq!(operation, "LIKE");
            assert_eq!(expected, "string");
        }
        _ => panic!("Expected TypeError"),
    }
}

#[test]
fn test_error_in_operator_with_non_string() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(42));

    let result = evaluate("x IN ('a', 'b')", &map);
    assert!(result.is_err());
    match result.unwrap_err() {
        EvalError::TypeError { operation, expected, .. } => {
            assert_eq!(operation, "IN");
            assert_eq!(expected, "string");
        }
        _ => panic!("Expected TypeError"),
    }
}

// String value cannot be used in numeric IN operator
#[test]
fn test_error_in_operator_with_non_numeric() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::String("banana".to_string()));

    let result = evaluate("x IN (6, 7, 8)", &map);
    assert!(result.is_err());
    match result.unwrap_err() {
        EvalError::TypeError { operation, expected, .. } => {
            assert_eq!(operation, "IN");
            assert_eq!(expected, "integer");
        }
        _ => panic!("Expected TypeError"),
    }
}

// Mixed numeric types (Integer and Float) are not allowed in IN operator list
// NOTE: This is now detected at parse time, not evaluation time
#[test]
fn test_error_in_operator_with_mixed_numeric_types() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(6));

    let result = evaluate("x IN (6, 7.0, 8)", &map);
    assert!(result.is_err());
    match result.unwrap_err() {
        EvalError::EvalParseError(msg) => {
            assert!(msg.contains("IN list elements must all be the same type"),
                "Expected parse error about mixed types, got: {}", msg);
        }
        other => panic!("Expected EvalParseError, got: {:?}", other),
    }
}

#[test]
fn test_error_unary_minus_on_string() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::String("hello".to_string()));

    let result = evaluate("(-x) = 'test'", &map);
    assert!(result.is_err());
    match result.unwrap_err() {
        EvalError::TypeError { operation, .. } => {
            assert_eq!(operation, "unary minus");
        }
        _ => panic!("Expected TypeError"),
    }
}

#[test]
fn test_error_unary_minus_on_null() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Null);

    let result = evaluate("(-x) = 0", &map);
    assert!(result.is_err());
    match result.unwrap_err() {
        EvalError::NullInOperation { operation, .. } => {
            assert_eq!(operation, "unary minus");
        }
        _ => panic!("Expected NullInOperation error"),
    }
}

// ============================================================================
// EDGE CASES AND BOUNDARY CONDITIONS
// ============================================================================

#[test]
fn test_empty_string_like() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::String("".to_string()));

    assert_eq!(evaluate("x LIKE ''", &map).unwrap(), true);
    assert_eq!(evaluate("x LIKE '%'", &map).unwrap(), true);
    assert_eq!(evaluate("x LIKE '_'", &map).unwrap(), false);
}

#[test]
fn test_large_integer() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(i64::MAX));

    assert_eq!(evaluate("x > 0", &map).unwrap(), true);
}

#[test]
fn test_negative_integer() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(-42));

    assert_eq!(evaluate("x < 0", &map).unwrap(), true);
    assert_eq!(evaluate("x = -42", &map).unwrap(), true);
}

#[test]
fn test_float_precision() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Float(0.1 + 0.2));

    // Floating point comparison
    assert_eq!(evaluate("x > 0.29", &map).unwrap(), true);
    assert_eq!(evaluate("x < 0.31", &map).unwrap(), true);
}

#[test]
fn test_complex_arithmetic_precedence() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), RuntimeValue::Integer(2));
    map.insert("b".to_string(), RuntimeValue::Integer(3));
    map.insert("c".to_string(), RuntimeValue::Integer(4));

    // 2 + 3 * 4 = 2 + 12 = 14
    assert_eq!(evaluate("(a + b * c) = 14", &map).unwrap(), true);
    // (2 + 3) * 4 = 5 * 4 = 20
    assert_eq!(evaluate("((a + b) * c) = 20", &map).unwrap(), true);
}

#[test]
fn test_deeply_nested_expression() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(5));

    assert_eq!(
        evaluate("((((x > 0) AND (x < 10)) OR (x = 100)) AND NOT (x = 3))", &map).unwrap(),
        true
    );
}

#[test]
fn test_multiple_and_or_precedence() {
    let map = HashMap::new();

    // OR has lower precedence than AND
    // FALSE AND TRUE OR TRUE = (FALSE AND TRUE) OR TRUE = FALSE OR TRUE = TRUE
    assert_eq!(evaluate("FALSE AND TRUE OR TRUE", &map).unwrap(), true);
    // TRUE OR FALSE AND FALSE = TRUE OR (FALSE AND FALSE) = TRUE OR FALSE = TRUE
    assert_eq!(evaluate("TRUE OR FALSE AND FALSE", &map).unwrap(), true);
}

#[test]
fn test_chained_comparisons_via_and() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(15));

    // Simulate x BETWEEN 10 AND 20 using AND
    assert_eq!(evaluate("x >= 10 AND x <= 20", &map).unwrap(), true);
}

#[test]
fn test_string_case_sensitivity() {
    let mut map = HashMap::new();
    map.insert("name".to_string(), RuntimeValue::String("Alice".to_string()));

    assert_eq!(evaluate("name = 'Alice'", &map).unwrap(), true);
    assert_eq!(evaluate("name = 'alice'", &map).unwrap(), false);
}

#[test]
fn test_like_with_multiple_wildcards() {
    let mut map = HashMap::new();
    map.insert("path".to_string(), RuntimeValue::String("/usr/local/bin".to_string()));

    assert_eq!(evaluate("path LIKE '%/%/%'", &map).unwrap(), true);
    assert_eq!(evaluate("path LIKE '/usr/%/bin'", &map).unwrap(), true);
}

#[test]
fn test_division_float_result() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), RuntimeValue::Integer(7));
    map.insert("b".to_string(), RuntimeValue::Integer(2));

    // 7 / 2 should be 3.5 (float)
    assert_eq!(evaluate("(a / b) > 3.4", &map).unwrap(), true);
    assert_eq!(evaluate("(a / b) < 3.6", &map).unwrap(), true);
}

#[test]
fn test_boolean_literal_in_value_context() {
    let map = HashMap::new();

    // TRUE/FALSE as string values in comparisons
    assert_eq!(evaluate("'TRUE' = 'TRUE'", &map).unwrap(), true);
    assert_eq!(evaluate("'FALSE' != 'TRUE'", &map).unwrap(), true);
}

