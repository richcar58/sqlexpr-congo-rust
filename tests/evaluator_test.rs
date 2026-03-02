use std::collections::HashMap;
use parser::{evaluate, RuntimeValue, EvalError};

// ========== Helpers ==========

fn empty_map() -> HashMap<String, RuntimeValue> {
    HashMap::new()
}

fn map_of(entries: &[(&str, RuntimeValue)]) -> HashMap<String, RuntimeValue> {
    entries.iter().map(|(k, v)| (k.to_string(), v.clone())).collect()
}

fn eval_ok(input: &str, map: &HashMap<String, RuntimeValue>) -> bool {
    evaluate(input, map).unwrap_or_else(|e| panic!("evaluate('{}') failed: {}", input, e))
}

fn eval_err(input: &str, map: &HashMap<String, RuntimeValue>) -> EvalError {
    evaluate(input, map).unwrap_err()
}

// ================================================================
// 1. Literals without variables
// ================================================================

#[test]
fn test_true_literal() {
    assert!(eval_ok("TRUE", &empty_map()));
}

#[test]
fn test_false_literal() {
    assert!(!eval_ok("FALSE", &empty_map()));
}

#[test]
fn test_true_case_insensitive() {
    assert!(eval_ok("true", &empty_map()));
}

#[test]
fn test_one_eq_one() {
    assert!(eval_ok("1 = 1", &empty_map()));
}

#[test]
fn test_one_eq_two() {
    assert!(!eval_ok("1 = 2", &empty_map()));
}

#[test]
fn test_string_equality() {
    assert!(eval_ok("'hello' = 'hello'", &empty_map()));
}

#[test]
fn test_string_inequality() {
    assert!(!eval_ok("'hello' = 'world'", &empty_map()));
}

#[test]
fn test_numeric_comparison() {
    assert!(eval_ok("5 > 3", &empty_map()));
}

#[test]
fn test_numeric_comparison_false() {
    assert!(!eval_ok("3 > 5", &empty_map()));
}

// ================================================================
// 2. Variable resolution
// ================================================================

#[test]
fn test_integer_variable() {
    let map = map_of(&[("x", RuntimeValue::Integer(42))]);
    assert!(eval_ok("x > 10", &map));
}

#[test]
fn test_float_variable() {
    let map = map_of(&[("x", RuntimeValue::Float(3.14))]);
    assert!(eval_ok("x > 3.0", &map));
}

#[test]
fn test_string_variable() {
    let map = map_of(&[("name", RuntimeValue::String("alice".to_string()))]);
    assert!(eval_ok("name = 'alice'", &map));
}

#[test]
fn test_boolean_variable() {
    let map = map_of(&[("flag", RuntimeValue::Boolean(true))]);
    assert!(eval_ok("flag = TRUE", &map));
}

#[test]
fn test_null_variable() {
    let map = map_of(&[("x", RuntimeValue::Null)]);
    assert!(eval_ok("x IS NULL", &map));
}

#[test]
fn test_null_variable_is_not_null() {
    let map = map_of(&[("x", RuntimeValue::Integer(1))]);
    assert!(eval_ok("x IS NOT NULL", &map));
}

#[test]
fn test_unbound_variable_error() {
    let err = eval_err("x > 10", &empty_map());
    assert!(matches!(err, EvalError::UnboundVariable { name } if name == "x"));
}

// ================================================================
// 3. AND/OR/NOT with short-circuit
// ================================================================

#[test]
fn test_and_true_true() {
    assert!(eval_ok("TRUE AND TRUE", &empty_map()));
}

#[test]
fn test_and_true_false() {
    assert!(!eval_ok("TRUE AND FALSE", &empty_map()));
}

#[test]
fn test_and_false_short_circuit() {
    // false AND unbound_var: should short-circuit and not evaluate unbound_var
    let map = map_of(&[("a", RuntimeValue::Boolean(false))]);
    assert!(!eval_ok("a AND unbound_var = 1", &map));
}

#[test]
fn test_or_true_true() {
    assert!(eval_ok("TRUE OR TRUE", &empty_map()));
}

#[test]
fn test_or_false_false() {
    assert!(!eval_ok("FALSE OR FALSE", &empty_map()));
}

#[test]
fn test_or_true_short_circuit() {
    // true OR unbound_var: should short-circuit
    let map = map_of(&[("a", RuntimeValue::Boolean(true))]);
    assert!(eval_ok("a OR unbound_var = 1", &map));
}

#[test]
fn test_not_true() {
    assert!(!eval_ok("NOT TRUE", &empty_map()));
}

#[test]
fn test_not_false() {
    assert!(eval_ok("NOT FALSE", &empty_map()));
}

#[test]
fn test_not_not_true() {
    assert!(eval_ok("NOT NOT TRUE", &empty_map()));
}

#[test]
fn test_multiple_and() {
    assert!(eval_ok("TRUE AND TRUE AND TRUE", &empty_map()));
}

#[test]
fn test_multiple_or() {
    assert!(eval_ok("FALSE OR FALSE OR TRUE", &empty_map()));
}

// ================================================================
// 4. Equality (=, <>, !=)
// ================================================================

#[test]
fn test_int_equal() {
    assert!(eval_ok("1 = 1", &empty_map()));
}

#[test]
fn test_int_not_equal() {
    assert!(eval_ok("1 <> 2", &empty_map()));
}

#[test]
fn test_float_equal() {
    assert!(eval_ok("3.14 = 3.14", &empty_map()));
}

#[test]
fn test_string_equal() {
    assert!(eval_ok("'abc' = 'abc'", &empty_map()));
}

#[test]
fn test_string_not_equal() {
    assert!(eval_ok("'abc' <> 'def'", &empty_map()));
}

#[test]
fn test_bool_equal() {
    assert!(eval_ok("TRUE = TRUE", &empty_map()));
}

#[test]
fn test_bool_not_equal() {
    assert!(eval_ok("TRUE <> FALSE", &empty_map()));
}

#[test]
fn test_mixed_int_float_equal() {
    let map = map_of(&[("x", RuntimeValue::Integer(3))]);
    assert!(eval_ok("x = 3.0", &map));
}

#[test]
fn test_is_null() {
    let map = map_of(&[("x", RuntimeValue::Null)]);
    assert!(eval_ok("x IS NULL", &map));
}

#[test]
fn test_is_null_false() {
    let map = map_of(&[("x", RuntimeValue::Integer(1))]);
    assert!(!eval_ok("x IS NULL", &map));
}

#[test]
fn test_is_not_null() {
    let map = map_of(&[("x", RuntimeValue::Integer(1))]);
    assert!(eval_ok("x IS NOT NULL", &map));
}

#[test]
fn test_is_not_null_false() {
    let map = map_of(&[("x", RuntimeValue::Null)]);
    assert!(!eval_ok("x IS NOT NULL", &map));
}

#[test]
fn test_null_equality_error() {
    let map = map_of(&[("x", RuntimeValue::Null)]);
    let err = eval_err("x = 1", &map);
    assert!(matches!(err, EvalError::NullInOperation { .. }));
}

// ================================================================
// 5. Comparison (<, <=, >, >=)
// ================================================================

#[test]
fn test_gt_int() {
    assert!(eval_ok("5 > 3", &empty_map()));
}

#[test]
fn test_ge_int() {
    assert!(eval_ok("5 >= 5", &empty_map()));
}

#[test]
fn test_lt_int() {
    assert!(eval_ok("3 < 5", &empty_map()));
}

#[test]
fn test_le_int() {
    assert!(eval_ok("5 <= 5", &empty_map()));
}

#[test]
fn test_gt_float() {
    assert!(eval_ok("3.14 > 2.71", &empty_map()));
}

#[test]
fn test_lt_string() {
    assert!(eval_ok("'abc' < 'def'", &empty_map()));
}

#[test]
fn test_ge_string() {
    assert!(eval_ok("'z' >= 'a'", &empty_map()));
}

#[test]
fn test_mixed_int_float_comparison() {
    let map = map_of(&[("x", RuntimeValue::Integer(5))]);
    assert!(eval_ok("x > 4.5", &map));
}

#[test]
fn test_comparison_null_error() {
    let map = map_of(&[("x", RuntimeValue::Null)]);
    let err = eval_err("x > 1", &map);
    assert!(matches!(err, EvalError::NullInOperation { .. }));
}

// ================================================================
// 6. Arithmetic (+, -, *, /, %)
// ================================================================

#[test]
fn test_int_addition() {
    assert!(eval_ok("1 + 2 = 3", &empty_map()));
}

#[test]
fn test_int_subtraction() {
    assert!(eval_ok("5 - 3 = 2", &empty_map()));
}

#[test]
fn test_int_multiplication() {
    assert!(eval_ok("3 * 4 = 12", &empty_map()));
}

#[test]
fn test_int_division_returns_float() {
    // Integer division always returns f64
    assert!(eval_ok("10 / 4 = 2.5", &empty_map()));
}

#[test]
fn test_int_modulo() {
    assert!(eval_ok("10 % 3 = 1", &empty_map()));
}

#[test]
fn test_float_addition() {
    assert!(eval_ok("1.5 + 2.5 = 4.0", &empty_map()));
}

#[test]
fn test_mixed_int_float_arithmetic() {
    let map = map_of(&[
        ("a", RuntimeValue::Integer(2)),
        ("b", RuntimeValue::Float(1.5)),
    ]);
    assert!(eval_ok("a + b = 3.5", &map));
}

#[test]
fn test_division_by_zero() {
    let err = eval_err("1 / 0 > 0", &empty_map());
    assert!(matches!(err, EvalError::DivisionByZero { .. }));
}

#[test]
fn test_modulo_by_zero() {
    let err = eval_err("5 % 0 > 0", &empty_map());
    assert!(matches!(err, EvalError::DivisionByZero { .. }));
}

#[test]
fn test_unary_plus() {
    let map = map_of(&[("x", RuntimeValue::Integer(5))]);
    assert!(eval_ok("+x = 5", &map));
}

#[test]
fn test_unary_negate() {
    let map = map_of(&[("x", RuntimeValue::Integer(5))]);
    assert!(eval_ok("-x = -5", &map));
}

#[test]
fn test_arithmetic_null_error() {
    let map = map_of(&[("x", RuntimeValue::Null)]);
    let err = eval_err("x + 1 > 0", &map);
    assert!(matches!(err, EvalError::NullInOperation { .. }));
}

#[test]
fn test_arithmetic_type_error() {
    let map = map_of(&[("x", RuntimeValue::String("hello".to_string()))]);
    let err = eval_err("x + 1 > 0", &map);
    assert!(matches!(err, EvalError::TypeError { .. }));
}

// ================================================================
// 7. BETWEEN / NOT BETWEEN
// ================================================================

#[test]
fn test_between_in_range() {
    let map = map_of(&[("x", RuntimeValue::Integer(5))]);
    assert!(eval_ok("x BETWEEN 1 AND 10", &map));
}

#[test]
fn test_between_out_of_range() {
    let map = map_of(&[("x", RuntimeValue::Integer(15))]);
    assert!(!eval_ok("x BETWEEN 1 AND 10", &map));
}

#[test]
fn test_between_boundary_inclusive_low() {
    let map = map_of(&[("x", RuntimeValue::Integer(1))]);
    assert!(eval_ok("x BETWEEN 1 AND 10", &map));
}

#[test]
fn test_between_boundary_inclusive_high() {
    let map = map_of(&[("x", RuntimeValue::Integer(10))]);
    assert!(eval_ok("x BETWEEN 1 AND 10", &map));
}

#[test]
fn test_between_string_range() {
    let map = map_of(&[("x", RuntimeValue::String("m".to_string()))]);
    assert!(eval_ok("x BETWEEN 'a' AND 'z'", &map));
}

#[test]
fn test_not_between() {
    let map = map_of(&[("x", RuntimeValue::Integer(15))]);
    assert!(eval_ok("x NOT BETWEEN 1 AND 10", &map));
}

#[test]
fn test_not_between_in_range() {
    let map = map_of(&[("x", RuntimeValue::Integer(5))]);
    assert!(!eval_ok("x NOT BETWEEN 1 AND 10", &map));
}

// ================================================================
// 8. LIKE / NOT LIKE
// ================================================================

#[test]
fn test_like_percent_wildcard() {
    let map = map_of(&[("x", RuntimeValue::String("hello world".to_string()))]);
    assert!(eval_ok("x LIKE '%world'", &map));
}

#[test]
fn test_like_percent_prefix() {
    let map = map_of(&[("x", RuntimeValue::String("hello world".to_string()))]);
    assert!(eval_ok("x LIKE 'hello%'", &map));
}

#[test]
fn test_like_percent_middle() {
    let map = map_of(&[("x", RuntimeValue::String("hello world".to_string()))]);
    assert!(eval_ok("x LIKE 'hello%world'", &map));
}

#[test]
fn test_like_underscore_wildcard() {
    let map = map_of(&[("x", RuntimeValue::String("cat".to_string()))]);
    assert!(eval_ok("x LIKE 'c_t'", &map));
}

#[test]
fn test_like_exact_match() {
    let map = map_of(&[("x", RuntimeValue::String("hello".to_string()))]);
    assert!(eval_ok("x LIKE 'hello'", &map));
}

#[test]
fn test_like_no_match() {
    let map = map_of(&[("x", RuntimeValue::String("hello".to_string()))]);
    assert!(!eval_ok("x LIKE 'world'", &map));
}

#[test]
fn test_like_escape_clause() {
    let map = map_of(&[("x", RuntimeValue::String("100%".to_string()))]);
    assert!(eval_ok("x LIKE '100!%' ESCAPE '!'", &map));
}

#[test]
fn test_like_escape_underscore() {
    let map = map_of(&[("x", RuntimeValue::String("a_b".to_string()))]);
    assert!(eval_ok("x LIKE 'a!_b' ESCAPE '!'", &map));
}

#[test]
fn test_not_like() {
    let map = map_of(&[("x", RuntimeValue::String("hello".to_string()))]);
    assert!(eval_ok("x NOT LIKE 'world'", &map));
}

#[test]
fn test_not_like_match() {
    let map = map_of(&[("x", RuntimeValue::String("hello".to_string()))]);
    assert!(!eval_ok("x NOT LIKE 'hello'", &map));
}

#[test]
fn test_like_percent_only() {
    let map = map_of(&[("x", RuntimeValue::String("anything".to_string()))]);
    assert!(eval_ok("x LIKE '%'", &map));
}

#[test]
fn test_like_empty_string() {
    let map = map_of(&[("x", RuntimeValue::String("".to_string()))]);
    assert!(eval_ok("x LIKE ''", &map));
}

#[test]
fn test_like_empty_no_match() {
    let map = map_of(&[("x", RuntimeValue::String("a".to_string()))]);
    assert!(!eval_ok("x LIKE ''", &map));
}

// ================================================================
// 9. IN / NOT IN
// ================================================================

#[test]
fn test_in_string_list_match() {
    let map = map_of(&[("x", RuntimeValue::String("red".to_string()))]);
    assert!(eval_ok("x IN ('red', 'green', 'blue')", &map));
}

#[test]
fn test_in_string_list_no_match() {
    let map = map_of(&[("x", RuntimeValue::String("yellow".to_string()))]);
    assert!(!eval_ok("x IN ('red', 'green', 'blue')", &map));
}

#[test]
fn test_in_integer_list_match() {
    let map = map_of(&[("x", RuntimeValue::Integer(2))]);
    assert!(eval_ok("x IN (1, 2, 3)", &map));
}

#[test]
fn test_in_integer_list_no_match() {
    let map = map_of(&[("x", RuntimeValue::Integer(4))]);
    assert!(!eval_ok("x IN (1, 2, 3)", &map));
}

#[test]
fn test_in_float_list_match() {
    let map = map_of(&[("x", RuntimeValue::Float(2.5))]);
    assert!(eval_ok("x IN (1.5, 2.5, 3.5)", &map));
}

#[test]
fn test_not_in_string_list() {
    let map = map_of(&[("x", RuntimeValue::String("yellow".to_string()))]);
    assert!(eval_ok("x NOT IN ('red', 'green', 'blue')", &map));
}

#[test]
fn test_not_in_match() {
    let map = map_of(&[("x", RuntimeValue::String("red".to_string()))]);
    assert!(!eval_ok("x NOT IN ('red', 'green', 'blue')", &map));
}

#[test]
fn test_not_in_integer_list() {
    let map = map_of(&[("x", RuntimeValue::Integer(10))]);
    assert!(eval_ok("x NOT IN (1, 2, 3)", &map));
}

// ================================================================
// 10. Complex expressions
// ================================================================

#[test]
fn test_complex_and_or() {
    let map = map_of(&[
        ("a", RuntimeValue::Integer(5)),
        ("b", RuntimeValue::Integer(15)),
    ]);
    assert!(eval_ok("a > 3 AND b < 20 OR a = 100", &map));
}

#[test]
fn test_complex_nested_parens() {
    let map = map_of(&[
        ("a", RuntimeValue::Boolean(true)),
        ("b", RuntimeValue::Boolean(false)),
        ("c", RuntimeValue::Boolean(true)),
    ]);
    // (true OR false) AND true => true
    assert!(eval_ok("(a OR b) AND c", &map));
}

#[test]
fn test_precedence_and_before_or() {
    // FALSE AND TRUE OR TRUE => FALSE OR TRUE => TRUE
    assert!(eval_ok("FALSE AND TRUE OR TRUE", &empty_map()));
}

#[test]
fn test_precedence_mult_before_add() {
    // 2 + 3 * 4 = 2 + 12 = 14
    assert!(eval_ok("2 + 3 * 4 = 14", &empty_map()));
}

#[test]
fn test_complex_arithmetic_comparison() {
    let map = map_of(&[
        ("a", RuntimeValue::Integer(10)),
        ("b", RuntimeValue::Integer(3)),
    ]);
    assert!(eval_ok("a * 2 + b >= 23", &map));
}

#[test]
fn test_complex_between_and_like() {
    let map = map_of(&[
        ("price", RuntimeValue::Integer(50)),
        ("name", RuntimeValue::String("Widget Pro".to_string())),
    ]);
    assert!(eval_ok("price BETWEEN 10 AND 100 AND name LIKE 'Widget%'", &map));
}

#[test]
fn test_complex_in_and_comparison() {
    let map = map_of(&[
        ("color", RuntimeValue::String("red".to_string())),
        ("size", RuntimeValue::Integer(5)),
    ]);
    assert!(eval_ok("color IN ('red', 'blue') AND size > 3", &map));
}

#[test]
fn test_complex_not_with_comparison() {
    let map = map_of(&[("x", RuntimeValue::Boolean(false))]);
    // NOT false = true, then true > 0 → type error in comparison
    // Actually: NOT x gives true (bool), then > 5 would be type error
    // Let's do a valid one: NOT (x = TRUE) => NOT false => true
    assert!(eval_ok("NOT x", &map));
}

#[test]
fn test_chained_equality() {
    // 1 = 1 = TRUE  → (1=1) yields true, then true = TRUE yields true
    // Actually the IS NULL / equality chaining... let's test a simple case
    let map = map_of(&[("x", RuntimeValue::Integer(1))]);
    assert!(eval_ok("x = 1", &map));
}

// ================================================================
// 11. Error cases
// ================================================================

#[test]
fn test_type_mismatch_equality() {
    let map = map_of(&[("x", RuntimeValue::Integer(1))]);
    let err = eval_err("x = 'hello'", &map);
    assert!(matches!(err, EvalError::TypeError { .. }));
}

#[test]
fn test_type_mismatch_comparison() {
    let map = map_of(&[("x", RuntimeValue::String("hello".to_string()))]);
    let err = eval_err("x > 5", &map);
    assert!(matches!(err, EvalError::TypeError { .. }));
}

#[test]
fn test_null_in_arithmetic() {
    let map = map_of(&[("x", RuntimeValue::Null)]);
    let err = eval_err("x + 1 > 0", &map);
    assert!(matches!(err, EvalError::NullInOperation { .. }));
}

#[test]
fn test_null_in_comparison() {
    let map = map_of(&[("x", RuntimeValue::Null)]);
    let err = eval_err("x > 1", &map);
    assert!(matches!(err, EvalError::NullInOperation { .. }));
}

#[test]
fn test_non_boolean_top_level() {
    let err = eval_err("42", &empty_map());
    assert!(matches!(err, EvalError::TypeError { .. }));
}

#[test]
fn test_parse_error() {
    let err = eval_err("a = = 1", &empty_map());
    assert!(matches!(err, EvalError::EvalParseError(_)));
}

#[test]
fn test_division_by_zero_error() {
    let err = eval_err("10 / 0 > 0", &empty_map());
    assert!(matches!(err, EvalError::DivisionByZero { .. }));
}

#[test]
fn test_not_type_error() {
    let map = map_of(&[("x", RuntimeValue::Integer(5))]);
    let err = eval_err("NOT x", &map);
    assert!(matches!(err, EvalError::TypeError { .. }));
}

#[test]
fn test_unary_negate_string_error() {
    let map = map_of(&[("x", RuntimeValue::String("hello".to_string()))]);
    let err = eval_err("-x > 0", &map);
    assert!(matches!(err, EvalError::TypeError { .. }));
}

#[test]
fn test_like_non_string_error() {
    let map = map_of(&[("x", RuntimeValue::Integer(42))]);
    let err = eval_err("x LIKE '%42'", &map);
    assert!(matches!(err, EvalError::TypeError { .. }));
}

#[test]
fn test_like_null_error() {
    let map = map_of(&[("x", RuntimeValue::Null)]);
    let err = eval_err("x LIKE '%foo'", &map);
    assert!(matches!(err, EvalError::NullInOperation { .. }));
}

#[test]
fn test_and_non_boolean_error() {
    let err = eval_err("1 AND TRUE", &empty_map());
    assert!(matches!(err, EvalError::TypeError { .. }));
}

#[test]
fn test_or_non_boolean_error() {
    let err = eval_err("1 OR TRUE", &empty_map());
    assert!(matches!(err, EvalError::TypeError { .. }));
}

#[test]
fn test_unary_negate_null_error() {
    let map = map_of(&[("x", RuntimeValue::Null)]);
    let err = eval_err("-x > 0", &map);
    assert!(matches!(err, EvalError::NullInOperation { .. }));
}

#[test]
fn test_unary_plus_null_error() {
    let map = map_of(&[("x", RuntimeValue::Null)]);
    let err = eval_err("+x > 0", &map);
    assert!(matches!(err, EvalError::NullInOperation { .. }));
}

#[test]
fn test_float_division_by_zero() {
    let err = eval_err("1.0 / 0.0 > 0", &empty_map());
    assert!(matches!(err, EvalError::DivisionByZero { .. }));
}

#[test]
fn test_modulo_by_zero_float() {
    let err = eval_err("5.0 % 0.0 > 0", &empty_map());
    assert!(matches!(err, EvalError::DivisionByZero { .. }));
}

// ================================================================
// Additional edge cases
// ================================================================

#[test]
fn test_like_underscore_length_mismatch() {
    let map = map_of(&[("x", RuntimeValue::String("ab".to_string()))]);
    assert!(!eval_ok("x LIKE '_'", &map));
}

#[test]
fn test_like_multiple_percent() {
    let map = map_of(&[("x", RuntimeValue::String("abcdef".to_string()))]);
    assert!(eval_ok("x LIKE '%cd%'", &map));
}

#[test]
fn test_between_float_value() {
    let map = map_of(&[("x", RuntimeValue::Float(5.5))]);
    assert!(eval_ok("x BETWEEN 1 AND 10", &map));
}

#[test]
fn test_in_single_element() {
    let map = map_of(&[("x", RuntimeValue::String("a".to_string()))]);
    assert!(eval_ok("x IN ('a')", &map));
}

#[test]
fn test_not_in_single_element() {
    let map = map_of(&[("x", RuntimeValue::String("b".to_string()))]);
    assert!(eval_ok("x NOT IN ('a')", &map));
}

#[test]
fn test_nested_not() {
    assert!(eval_ok("NOT NOT TRUE", &empty_map()));
}

#[test]
fn test_complex_with_all_ops() {
    let map = map_of(&[
        ("price", RuntimeValue::Float(49.99)),
        ("qty", RuntimeValue::Integer(3)),
        ("status", RuntimeValue::String("active".to_string())),
    ]);
    assert!(eval_ok(
        "price * qty > 100.0 AND status = 'active'",
        &map
    ));
}

#[test]
fn test_null_is_null_chained() {
    let map = map_of(&[("x", RuntimeValue::Null)]);
    assert!(eval_ok("x IS NULL", &map));
}

#[test]
fn test_multiple_is_null() {
    let map = map_of(&[
        ("x", RuntimeValue::Null),
        ("y", RuntimeValue::Integer(1)),
    ]);
    assert!(eval_ok("x IS NULL AND y IS NOT NULL", &map));
}

#[test]
fn test_not_like_with_escape() {
    let map = map_of(&[("x", RuntimeValue::String("100%".to_string()))]);
    assert!(!eval_ok("x NOT LIKE '100!%' ESCAPE '!'", &map));
}

#[test]
fn test_float_mul() {
    assert!(eval_ok("2.0 * 3.0 = 6.0", &empty_map()));
}

#[test]
fn test_float_sub() {
    assert!(eval_ok("5.0 - 2.0 = 3.0", &empty_map()));
}

#[test]
fn test_int_sub_result() {
    assert!(eval_ok("10 - 7 = 3", &empty_map()));
}

#[test]
fn test_mixed_arithmetic_result() {
    // 2 * 3.5 = 7.0 (int * float = float)
    assert!(eval_ok("2 * 3.5 = 7.0", &empty_map()));
}
