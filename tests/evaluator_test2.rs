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

// ============================================================================
// MIXED NUMERIC FORMAT TESTS (DECIMAL / HEX / OCTAL)
// ============================================================================

// --- Equality comparisons with mixed formats ---

#[test]
fn test_mixed_hex_equals_decimal() {
    let map = HashMap::new();
    assert_eq!(evaluate("0xFF = 255", &map).unwrap(), true);
    assert_eq!(evaluate("0x0A = 10", &map).unwrap(), true);
    assert_eq!(evaluate("0x00 = 0", &map).unwrap(), true);
    assert_eq!(evaluate("0x01 = 1", &map).unwrap(), true);
    assert_eq!(evaluate("0x10 = 16", &map).unwrap(), true);
    assert_eq!(evaluate("0x64 = 100", &map).unwrap(), true);
}

#[test]
fn test_mixed_octal_equals_decimal() {
    let map = HashMap::new();
    assert_eq!(evaluate("010 = 8", &map).unwrap(), true);
    assert_eq!(evaluate("017 = 15", &map).unwrap(), true);
    assert_eq!(evaluate("077 = 63", &map).unwrap(), true);
    assert_eq!(evaluate("0144 = 100", &map).unwrap(), true);
    assert_eq!(evaluate("012 = 10", &map).unwrap(), true);
}

#[test]
fn test_mixed_hex_equals_octal() {
    let map = HashMap::new();
    // 0x0A = 10, 012 = 10
    assert_eq!(evaluate("0x0A = 012", &map).unwrap(), true);
    // 0xFF = 255, 0377 = 255
    assert_eq!(evaluate("0xFF = 0377", &map).unwrap(), true);
    // 0x10 = 16, 020 = 16
    assert_eq!(evaluate("0x10 = 020", &map).unwrap(), true);
    // 0x08 = 8, 010 = 8
    assert_eq!(evaluate("0x08 = 010", &map).unwrap(), true);
}

#[test]
fn test_mixed_not_equal_formats() {
    let map = HashMap::new();
    assert_eq!(evaluate("0xFF <> 254", &map).unwrap(), true);
    assert_eq!(evaluate("010 <> 9", &map).unwrap(), true);
    assert_eq!(evaluate("0x0A <> 012", &map).unwrap(), false); // both are 10
    assert_eq!(evaluate("0xFF != 0376", &map).unwrap(), true); // 255 != 254
}

// --- Equality with variables ---

#[test]
fn test_mixed_variable_equals_hex() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(255));
    assert_eq!(evaluate("x = 0xFF", &map).unwrap(), true);
    assert_eq!(evaluate("x <> 0xFE", &map).unwrap(), true);
}

#[test]
fn test_mixed_variable_equals_octal() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(8));
    assert_eq!(evaluate("x = 010", &map).unwrap(), true);
    assert_eq!(evaluate("x <> 011", &map).unwrap(), true);
}

#[test]
fn test_mixed_float_variable_with_hex() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Float(10.0));
    // Integer/Float cross-comparison in equality
    assert_eq!(evaluate("x = 0x0A", &map).unwrap(), true);
}

#[test]
fn test_mixed_float_variable_with_octal() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Float(8.0));
    assert_eq!(evaluate("x = 010", &map).unwrap(), true);
}

// --- Ordering comparisons with mixed formats ---

#[test]
fn test_mixed_hex_greater_than_decimal() {
    let map = HashMap::new();
    assert_eq!(evaluate("0xFF > 254", &map).unwrap(), true);
    assert_eq!(evaluate("0xFF > 255", &map).unwrap(), false);
    assert_eq!(evaluate("0x0A > 9", &map).unwrap(), true);
}

#[test]
fn test_mixed_octal_greater_than_decimal() {
    let map = HashMap::new();
    assert_eq!(evaluate("010 > 7", &map).unwrap(), true);
    assert_eq!(evaluate("010 > 8", &map).unwrap(), false);
    assert_eq!(evaluate("077 > 62", &map).unwrap(), true);
}

#[test]
fn test_mixed_hex_less_than_octal() {
    let map = HashMap::new();
    // 0x07 = 7, 010 = 8
    assert_eq!(evaluate("0x07 < 010", &map).unwrap(), true);
    // 0x09 = 9, 010 = 8
    assert_eq!(evaluate("0x09 < 010", &map).unwrap(), false);
}

#[test]
fn test_mixed_greater_equal_formats() {
    let map = HashMap::new();
    assert_eq!(evaluate("0xFF >= 255", &map).unwrap(), true);
    assert_eq!(evaluate("0xFF >= 256", &map).unwrap(), false);
    assert_eq!(evaluate("010 >= 010", &map).unwrap(), true);
    assert_eq!(evaluate("0x10 >= 020", &map).unwrap(), true); // both 16
}

#[test]
fn test_mixed_less_equal_formats() {
    let map = HashMap::new();
    assert_eq!(evaluate("0x0A <= 012", &map).unwrap(), true); // both 10
    assert_eq!(evaluate("0x0A <= 10", &map).unwrap(), true);
    assert_eq!(evaluate("010 <= 0x08", &map).unwrap(), true); // both 8
}

#[test]
fn test_mixed_ordering_variable_vs_hex() {
    let mut map = HashMap::new();
    map.insert("score".to_string(), RuntimeValue::Integer(200));
    assert_eq!(evaluate("score > 0xC7", &map).unwrap(), true); // 200 > 199
    assert_eq!(evaluate("score < 0xC9", &map).unwrap(), true); // 200 < 201
    assert_eq!(evaluate("score >= 0xC8", &map).unwrap(), true); // 200 >= 200
    assert_eq!(evaluate("score <= 0xC8", &map).unwrap(), true); // 200 <= 200
}

#[test]
fn test_mixed_ordering_variable_vs_octal() {
    let mut map = HashMap::new();
    map.insert("val".to_string(), RuntimeValue::Integer(63));
    assert_eq!(evaluate("val > 076", &map).unwrap(), true); // 63 > 62
    assert_eq!(evaluate("val < 0100", &map).unwrap(), true); // 63 < 64
    assert_eq!(evaluate("val >= 077", &map).unwrap(), true); // 63 >= 63
    assert_eq!(evaluate("val <= 077", &map).unwrap(), true); // 63 <= 63
}

// --- Arithmetic with mixed formats ---

#[test]
fn test_mixed_addition_hex_decimal() {
    let map = HashMap::new();
    // 0x0A + 5 = 15
    assert_eq!(evaluate("0x0A + 5 = 15", &map).unwrap(), true);
}

#[test]
fn test_mixed_addition_octal_decimal() {
    let map = HashMap::new();
    // 010 + 2 = 10
    assert_eq!(evaluate("010 + 2 = 10", &map).unwrap(), true);
}

#[test]
fn test_mixed_addition_hex_octal() {
    let map = HashMap::new();
    // 0x0A + 010 = 10 + 8 = 18
    assert_eq!(evaluate("0x0A + 010 = 18", &map).unwrap(), true);
}

#[test]
fn test_mixed_subtraction_hex_decimal() {
    let map = HashMap::new();
    // 0xFF - 200 = 55
    assert_eq!(evaluate("0xFF - 200 = 55", &map).unwrap(), true);
}

#[test]
fn test_mixed_subtraction_octal_hex() {
    let map = HashMap::new();
    // 077 - 0x3E = 63 - 62 = 1
    assert_eq!(evaluate("077 - 0x3E = 1", &map).unwrap(), true);
}

#[test]
fn test_mixed_multiplication_hex_decimal() {
    let map = HashMap::new();
    // 0x0A * 5 = 50
    assert_eq!(evaluate("0x0A * 5 = 50", &map).unwrap(), true);
}

#[test]
fn test_mixed_multiplication_octal_hex() {
    let map = HashMap::new();
    // 010 * 0x04 = 8 * 4 = 32
    assert_eq!(evaluate("010 * 0x04 = 32", &map).unwrap(), true);
}

#[test]
fn test_mixed_division_hex_decimal() {
    let map = HashMap::new();
    // 0x14 / 4 = 20 / 4 = 5
    assert_eq!(evaluate("0x14 / 4 = 5", &map).unwrap(), true);
}

#[test]
fn test_mixed_division_octal_decimal() {
    let map = HashMap::new();
    // 020 / 4 = 16 / 4 = 4
    assert_eq!(evaluate("020 / 4 = 4", &map).unwrap(), true);
}

#[test]
fn test_mixed_modulo_hex_decimal() {
    let map = HashMap::new();
    // 0xFF % 100 = 255 % 100 = 55
    assert_eq!(evaluate("0xFF % 100 = 55", &map).unwrap(), true);
}

#[test]
fn test_mixed_modulo_octal_hex() {
    let map = HashMap::new();
    // 077 % 0x0A = 63 % 10 = 3
    assert_eq!(evaluate("077 % 0x0A = 3", &map).unwrap(), true);
}

#[test]
fn test_mixed_arithmetic_with_variable() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(10));
    // x + 0xFF = 10 + 255 = 265
    assert_eq!(evaluate("x + 0xFF = 265", &map).unwrap(), true);
    // x * 010 = 10 * 8 = 80
    assert_eq!(evaluate("x * 010 = 80", &map).unwrap(), true);
    // x - 0x05 = 10 - 5 = 5
    assert_eq!(evaluate("x - 0x05 = 5", &map).unwrap(), true);
}

#[test]
fn test_mixed_arithmetic_all_three_formats() {
    let map = HashMap::new();
    // 0x0A + 010 + 2 = 10 + 8 + 2 = 20
    assert_eq!(evaluate("0x0A + 010 + 2 = 20", &map).unwrap(), true);
    // 0xFF - 077 - 100 = 255 - 63 - 100 = 92
    assert_eq!(evaluate("0xFF - 077 - 100 = 92", &map).unwrap(), true);
}

#[test]
fn test_mixed_complex_arithmetic_expression() {
    let mut map = HashMap::new();
    map.insert("base".to_string(), RuntimeValue::Integer(100));
    // base + 0x0A * 010 = 100 + 10 * 8 = 100 + 80 = 180 (multiplication has higher precedence)
    assert_eq!(evaluate("(base + 0x0A * 010) = 180", &map).unwrap(), true);
}

// --- BETWEEN with mixed formats ---

#[test]
fn test_between_hex_bounds() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(128));
    // x BETWEEN 0x00 AND 0xFF => 128 between 0 and 255
    assert_eq!(evaluate("x BETWEEN 0x00 AND 0xFF", &map).unwrap(), true);
}

#[test]
fn test_between_octal_bounds() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(10));
    // x BETWEEN 010 AND 077 => 10 between 8 and 63
    assert_eq!(evaluate("x BETWEEN 010 AND 077", &map).unwrap(), true);
}

#[test]
fn test_between_hex_low_decimal_high() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(50));
    // x BETWEEN 0x0A AND 100 => 50 between 10 and 100
    assert_eq!(evaluate("x BETWEEN 0x0A AND 100", &map).unwrap(), true);
}

#[test]
fn test_between_decimal_low_hex_high() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(200));
    // x BETWEEN 10 AND 0xFF => 200 between 10 and 255
    assert_eq!(evaluate("x BETWEEN 10 AND 0xFF", &map).unwrap(), true);
}

#[test]
fn test_between_octal_low_hex_high() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(20));
    // x BETWEEN 010 AND 0xFF => 20 between 8 and 255
    assert_eq!(evaluate("x BETWEEN 010 AND 0xFF", &map).unwrap(), true);
}

#[test]
fn test_between_hex_low_octal_high() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(30));
    // x BETWEEN 0x0A AND 077 => 30 between 10 and 63
    assert_eq!(evaluate("x BETWEEN 0x0A AND 077", &map).unwrap(), true);
}

#[test]
fn test_between_false_with_mixed_formats() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(5));
    // x BETWEEN 0x0A AND 0xFF => 5 NOT between 10 and 255
    assert_eq!(evaluate("x BETWEEN 0x0A AND 0xFF", &map).unwrap(), false);
}

#[test]
fn test_between_boundary_with_hex() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(10));
    // Boundary: x = low bound
    assert_eq!(evaluate("x BETWEEN 0x0A AND 0xFF", &map).unwrap(), true);
    map.insert("x".to_string(), RuntimeValue::Integer(255));
    // Boundary: x = high bound
    assert_eq!(evaluate("x BETWEEN 0x0A AND 0xFF", &map).unwrap(), true);
}

#[test]
fn test_not_between_hex_bounds() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(300));
    assert_eq!(evaluate("x NOT BETWEEN 0x00 AND 0xFF", &map).unwrap(), true);
    map.insert("x".to_string(), RuntimeValue::Integer(128));
    assert_eq!(evaluate("x NOT BETWEEN 0x00 AND 0xFF", &map).unwrap(), false);
}

#[test]
fn test_not_between_octal_bounds() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(100));
    // x NOT BETWEEN 010 AND 077 => 100 NOT between 8 and 63
    assert_eq!(evaluate("x NOT BETWEEN 010 AND 077", &map).unwrap(), true);
}

#[test]
fn test_not_between_mixed_formats() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(5));
    // x NOT BETWEEN 0x0A AND 077 => 5 NOT between 10 and 63
    assert_eq!(evaluate("x NOT BETWEEN 0x0A AND 077", &map).unwrap(), true);
}

// --- IN with mixed formats ---

#[test]
fn test_in_all_hex_elements() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(255));
    assert_eq!(evaluate("x IN (0x0A, 0xFF, 0x10)", &map).unwrap(), true);
    map.insert("x".to_string(), RuntimeValue::Integer(99));
    assert_eq!(evaluate("x IN (0x0A, 0xFF, 0x10)", &map).unwrap(), false);
}

#[test]
fn test_in_all_octal_elements() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(8));
    assert_eq!(evaluate("x IN (010, 017, 077)", &map).unwrap(), true);
    map.insert("x".to_string(), RuntimeValue::Integer(9));
    assert_eq!(evaluate("x IN (010, 017, 077)", &map).unwrap(), false);
}

#[test]
fn test_in_mixed_hex_and_decimal() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(16));
    // Mix hex and decimal in IN list
    assert_eq!(evaluate("x IN (0x0A, 16, 0xFF)", &map).unwrap(), true);
    assert_eq!(evaluate("x IN (0x0A, 15, 0xFF)", &map).unwrap(), false);
}

#[test]
fn test_in_mixed_octal_and_decimal() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(63));
    assert_eq!(evaluate("x IN (010, 63, 0144)", &map).unwrap(), true);
}

#[test]
fn test_in_mixed_hex_and_octal() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(10));
    // 0x0A = 10, 010 = 8, 0x10 = 16
    assert_eq!(evaluate("x IN (0x0A, 010, 0x10)", &map).unwrap(), true);
    map.insert("x".to_string(), RuntimeValue::Integer(8));
    assert_eq!(evaluate("x IN (0x0A, 010, 0x10)", &map).unwrap(), true);
}

#[test]
fn test_in_all_three_formats() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(100));
    // 0x64 = 100, 010 = 8, 42 = 42
    assert_eq!(evaluate("x IN (0x64, 010, 42)", &map).unwrap(), true);
    map.insert("x".to_string(), RuntimeValue::Integer(8));
    assert_eq!(evaluate("x IN (0x64, 010, 42)", &map).unwrap(), true);
    map.insert("x".to_string(), RuntimeValue::Integer(42));
    assert_eq!(evaluate("x IN (0x64, 010, 42)", &map).unwrap(), true);
    map.insert("x".to_string(), RuntimeValue::Integer(99));
    assert_eq!(evaluate("x IN (0x64, 010, 42)", &map).unwrap(), false);
}

#[test]
fn test_not_in_hex_elements() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(99));
    assert_eq!(evaluate("x NOT IN (0x0A, 0xFF, 0x10)", &map).unwrap(), true);
    map.insert("x".to_string(), RuntimeValue::Integer(255));
    assert_eq!(evaluate("x NOT IN (0x0A, 0xFF, 0x10)", &map).unwrap(), false);
}

#[test]
fn test_not_in_octal_elements() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(9));
    assert_eq!(evaluate("x NOT IN (010, 017, 077)", &map).unwrap(), true);
    map.insert("x".to_string(), RuntimeValue::Integer(15));
    assert_eq!(evaluate("x NOT IN (010, 017, 077)", &map).unwrap(), false);
}

#[test]
fn test_not_in_mixed_formats() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(50));
    // 0x0A = 10, 010 = 8, 42 = 42
    assert_eq!(evaluate("x NOT IN (0x0A, 010, 42)", &map).unwrap(), true);
    map.insert("x".to_string(), RuntimeValue::Integer(10));
    assert_eq!(evaluate("x NOT IN (0x0A, 010, 42)", &map).unwrap(), false);
}

// --- Unary operators on hex/octal ---

#[test]
fn test_unary_minus_hex() {
    let map = HashMap::new();
    // -0x0A = -10
    assert_eq!(evaluate("-0x0A = -10", &map).unwrap(), true);
    assert_eq!(evaluate("-0xFF = -255", &map).unwrap(), true);
}

#[test]
fn test_unary_minus_octal() {
    let map = HashMap::new();
    // -010 = -8
    assert_eq!(evaluate("-010 = -8", &map).unwrap(), true);
    assert_eq!(evaluate("-077 = -63", &map).unwrap(), true);
}

#[test]
fn test_unary_plus_hex() {
    let map = HashMap::new();
    assert_eq!(evaluate("+0x0A = 10", &map).unwrap(), true);
}

#[test]
fn test_unary_plus_octal() {
    let map = HashMap::new();
    assert_eq!(evaluate("+010 = 8", &map).unwrap(), true);
}

// --- IS NULL / IS NOT NULL with hex/octal in same expression ---

#[test]
fn test_is_null_and_hex_comparison() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Null);
    map.insert("y".to_string(), RuntimeValue::Integer(255));
    assert_eq!(evaluate("x IS NULL AND y = 0xFF", &map).unwrap(), true);
    assert_eq!(evaluate("x IS NULL AND y > 0xFE", &map).unwrap(), true);
}

#[test]
fn test_is_not_null_and_octal_comparison() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(42));
    map.insert("y".to_string(), RuntimeValue::Integer(8));
    assert_eq!(evaluate("x IS NOT NULL AND y = 010", &map).unwrap(), true);
}

#[test]
fn test_is_null_or_hex_match() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(16));
    assert_eq!(evaluate("x IS NULL OR x = 0x10", &map).unwrap(), true);
}

// --- Complex expressions combining multiple contexts ---

#[test]
fn test_mixed_complex_and_or_with_hex_octal() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), RuntimeValue::Integer(100));
    map.insert("b".to_string(), RuntimeValue::Integer(8));
    // a > 0x50 (80) AND b = 010 (8)
    assert_eq!(evaluate("a > 0x50 AND b = 010", &map).unwrap(), true);
}

#[test]
fn test_mixed_arithmetic_in_comparison_with_hex() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(10));
    // (x + 0x06) = 0x10 => (10 + 6) = 16
    assert_eq!(evaluate("(x + 0x06) = 0x10", &map).unwrap(), true);
}

#[test]
fn test_mixed_arithmetic_in_comparison_with_octal() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(4));
    // (x * 010) = 32 => (4 * 8) = 32
    assert_eq!(evaluate("(x * 010) = 32", &map).unwrap(), true);
}

#[test]
fn test_mixed_between_with_arithmetic() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(50));
    // (x + 0x0A) BETWEEN 010 AND 0xFF => 60 between 8 and 255
    assert_eq!(evaluate("(x + 0x0A) BETWEEN 010 AND 0xFF", &map).unwrap(), true);
}

#[test]
fn test_mixed_in_with_variable_arithmetic() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(6));
    // (x + 0x04) IN (0x0A, 010, 20) => 10 IN (10, 8, 20)
    assert_eq!(evaluate("(x + 0x04) IN (0x0A, 010, 20)", &map).unwrap(), true);
}

#[test]
fn test_mixed_complex_nested_expression() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(100));
    map.insert("y".to_string(), RuntimeValue::Integer(200));
    // (x > 0x32 AND y < 0xC9) OR x BETWEEN 010 AND 077
    // (100 > 50 AND 200 < 201) OR 100 BETWEEN 8 AND 63
    // (true AND true) OR false = true
    assert_eq!(
        evaluate("(x > 0x32 AND y < 0xC9) OR x BETWEEN 010 AND 077", &map).unwrap(),
        true
    );
}

#[test]
fn test_mixed_all_formats_in_single_expression() {
    let mut map = HashMap::new();
    map.insert("a".to_string(), RuntimeValue::Integer(10));
    map.insert("b".to_string(), RuntimeValue::Integer(8));
    map.insert("c".to_string(), RuntimeValue::Integer(255));
    // a = 0x0A AND b = 010 AND c = 255
    assert_eq!(
        evaluate("a = 0x0A AND b = 010 AND c = 255", &map).unwrap(),
        true
    );
}

#[test]
fn test_mixed_not_with_hex_comparison() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(100));
    // NOT (100 = 255) => NOT false => true
    assert_eq!(evaluate("NOT (x = 0xFF)", &map).unwrap(), true);
}

#[test]
fn test_mixed_not_with_octal_comparison() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(100));
    // NOT (100 = 63) => NOT false => true
    assert_eq!(evaluate("NOT (x = 077)", &map).unwrap(), true);
}

#[test]
fn test_mixed_chained_arithmetic_all_formats() {
    let map = HashMap::new();
    // 0xFF - 0x0A - 010 - 100 = 255 - 10 - 8 - 100 = 137
    assert_eq!(evaluate("0xFF - 0x0A - 010 - 100 = 137", &map).unwrap(), true);
}

#[test]
fn test_mixed_multiplicative_all_formats() {
    let map = HashMap::new();
    // 0x02 * 010 * 3 = 2 * 8 * 3 = 48
    assert_eq!(evaluate("0x02 * 010 * 3 = 48", &map).unwrap(), true);
}

#[test]
fn test_mixed_division_hex_by_octal() {
    let map = HashMap::new();
    // 0x10 / 010 = 16 / 8 = 2
    assert_eq!(evaluate("0x10 / 010 = 2", &map).unwrap(), true);
}

#[test]
fn test_mixed_modulo_all_formats() {
    let map = HashMap::new();
    // 0xFF % 010 = 255 % 8 = 7
    assert_eq!(evaluate("0xFF % 010 = 7", &map).unwrap(), true);
    // 0xFF % 100 = 255 % 100 = 55
    assert_eq!(evaluate("0xFF % 100 = 55", &map).unwrap(), true);
}

// --- Error cases with hex/octal ---

#[test]
fn test_error_null_in_arithmetic_with_hex() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Null);
    let result = evaluate("(x + 0xFF) > 0", &map);
    assert!(result.is_err());
    match result.unwrap_err() {
        EvalError::NullInOperation { operation, .. } => {
            assert_eq!(operation, "addition");
        }
        other => panic!("Expected NullInOperation, got: {:?}", other),
    }
}

#[test]
fn test_error_null_in_comparison_with_octal() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Null);
    let result = evaluate("x > 010", &map);
    assert!(result.is_err());
    match result.unwrap_err() {
        EvalError::NullInOperation { operation, .. } => {
            assert_eq!(operation, "GreaterThan");
        }
        other => panic!("Expected NullInOperation, got: {:?}", other),
    }
}

#[test]
fn test_error_null_in_equality_with_hex() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Null);
    let result = evaluate("x = 0xFF", &map);
    assert!(result.is_err());
    match result.unwrap_err() {
        EvalError::NullInOperation { operation, .. } => {
            assert_eq!(operation, "Equal");
        }
        other => panic!("Expected NullInOperation, got: {:?}", other),
    }
}

#[test]
fn test_error_null_in_between_with_hex_bounds() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Null);
    let result = evaluate("x BETWEEN 0x00 AND 0xFF", &map);
    assert!(result.is_err());
    match result.unwrap_err() {
        EvalError::NullInOperation { operation, .. } => {
            assert_eq!(operation, "BETWEEN");
        }
        other => panic!("Expected NullInOperation, got: {:?}", other),
    }
}

#[test]
fn test_error_string_in_hex_in_list() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::String("hello".to_string()));
    let result = evaluate("x IN (0x0A, 0xFF)", &map);
    assert!(result.is_err());
    match result.unwrap_err() {
        EvalError::TypeError { operation, expected, .. } => {
            assert_eq!(operation, "IN");
            assert_eq!(expected, "integer");
        }
        other => panic!("Expected TypeError, got: {:?}", other),
    }
}

#[test]
fn test_error_string_in_octal_in_list() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::String("hello".to_string()));
    let result = evaluate("x IN (010, 077)", &map);
    assert!(result.is_err());
    match result.unwrap_err() {
        EvalError::TypeError { operation, expected, .. } => {
            assert_eq!(operation, "IN");
            assert_eq!(expected, "integer");
        }
        other => panic!("Expected TypeError, got: {:?}", other),
    }
}

#[test]
fn test_error_division_by_zero_hex() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Integer(10));
    let result = evaluate("x / 0x00 > 0", &map);
    assert!(result.is_err());
    match result.unwrap_err() {
        EvalError::DivisionByZero { .. } => {}
        other => panic!("Expected DivisionByZero, got: {:?}", other),
    }
}

#[test]
fn test_error_unary_minus_null_with_hex_context() {
    let mut map = HashMap::new();
    map.insert("x".to_string(), RuntimeValue::Null);
    let result = evaluate("(-x) = 0x0A", &map);
    assert!(result.is_err());
    match result.unwrap_err() {
        EvalError::NullInOperation { operation, .. } => {
            assert_eq!(operation, "unary minus");
        }
        other => panic!("Expected NullInOperation, got: {:?}", other),
    }
}

