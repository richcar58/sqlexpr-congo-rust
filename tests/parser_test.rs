use parser::{
    AstNode, Parser, Arena, NodeId,
    EqualityOp, ComparisonOp, UnaryOp, AddOp, MultExprOp,
};

// ========== Helpers ==========

/// Parse input successfully, returning (Parser, root NodeId).
fn parse_ok(input: &str) -> (Parser, NodeId) {
    let mut p = Parser::new(input.to_string()).expect("Parser::new failed");
    let root = p.parse().expect(&format!("parse failed for: {}", input));
    println!("{}", p.arena().pretty_print(root, 0, p.input()));
    (p, root)
}

/// Parse input expecting failure, returning the error message.
fn parse_err(input: &str) -> String {
    let result = Parser::new(input.to_string()).and_then(|mut p| {
        p.parse()?;
        Ok(p)
    });
    match result {
        Ok(_) => panic!("expected parse error for: {}", input),
        Err(err) => {
            println!("Error for \"{}\": {}", input, err);
            err.message
        }
    }
}

/// Walk through pass-through (single-child) nodes to reach the semantic node.
fn skip(arena: &Arena, mut id: NodeId) -> NodeId {
    loop {
        let children = get_children(arena, id);
        let dominated = match arena.get_node(id) {
            AstNode::JmsSelector(_) => children.len() == 1,
            AstNode::OrExpression(_) => children.len() == 1,
            AstNode::AndExpression(_) => children.len() == 1,
            AstNode::EqualityExpression(n) => children.len() == 1 && n.operators.is_empty(),
            AstNode::ComparisonExpression(n) => children.len() == 1 && n.operators.is_empty(),
            AstNode::AddExpression(n) => children.len() == 1 && n.operators.is_empty(),
            AstNode::MultExpr(n) => children.len() == 1 && n.operators.is_empty(),
            AstNode::UnaryExpr(n) => children.len() == 1 && n.operator.is_none(),
            _ => false,
        };
        if dominated {
            id = children[0];
        } else {
            return id;
        }
    }
}

fn get_children(arena: &Arena, id: NodeId) -> Vec<NodeId> {
    match arena.get_node(id) {
        AstNode::JmsSelector(n) => n.children.clone(),
        AstNode::OrExpression(n) => n.children.clone(),
        AstNode::AndExpression(n) => n.children.clone(),
        AstNode::EqualityExpression(n) => n.children.clone(),
        AstNode::ComparisonExpression(n) => n.children.clone(),
        AstNode::AddExpression(n) => n.children.clone(),
        AstNode::MultExpr(n) => n.children.clone(),
        AstNode::UnaryExpr(n) => n.children.clone(),
        AstNode::PrimaryExpr(n) => n.children.clone(),
        AstNode::Literal(n) => n.children.clone(),
        AstNode::StringLiteral(n) => n.children.clone(),
        AstNode::Variable(n) => n.children.clone(),
    }
}

/// Get the token image for a leaf node, descending through wrappers.
fn leaf_image(arena: &Arena, id: NodeId) -> String {
    let id = skip(arena, id);
    match arena.get_node(id) {
        AstNode::Variable(n) => arena.get_token(n.begin_token).image.clone(),
        AstNode::Literal(n) => arena.get_token(n.begin_token).image.clone(),
        AstNode::StringLiteral(n) => arena.get_token(n.begin_token).image.clone(),
        AstNode::PrimaryExpr(n) => {
            if n.children.is_empty() {
                arena.get_token(n.begin_token).image.clone()
            } else {
                leaf_image(arena, n.children[0])
            }
        }
        other => panic!("leaf_image called on non-leaf: {:?}", other),
    }
}

/// Descend through PrimaryExpr wrapper(s) to get the inner leaf image.
fn primary_value(arena: &Arena, id: NodeId) -> String {
    let id = skip(arena, id);
    match arena.get_node(id) {
        AstNode::PrimaryExpr(n) => {
            if n.children.is_empty() {
                arena.get_token(n.begin_token).image.clone()
            } else {
                leaf_image(arena, n.children[0])
            }
        }
        _ => leaf_image(arena, id),
    }
}

// ================================================================
// POSITIVE TESTS: Literals
// ================================================================

#[test]
fn test_integer_literal() {
    let (p, root) = parse_ok("42");
    let n = skip(p.arena(), root);
    assert!(matches!(p.arena().get_node(n), AstNode::PrimaryExpr(_)));
    assert_eq!(primary_value(p.arena(), n), "42");
}

#[test]
fn test_decimal_literal() {
    let (p, root) = parse_ok("3.14");
    let n = skip(p.arena(), root);
    assert_eq!(primary_value(p.arena(), n), "3.14");
}

#[test]
fn test_string_literal() {
    let (p, root) = parse_ok("'hello'");
    let n = skip(p.arena(), root);
    assert!(matches!(p.arena().get_node(n), AstNode::PrimaryExpr(_)));
    assert_eq!(primary_value(p.arena(), n), "'hello'");
}

#[test]
fn test_empty_string_literal() {
    let (p, root) = parse_ok("''");
    let n = skip(p.arena(), root);
    assert_eq!(primary_value(p.arena(), n), "''");
}

#[test]
fn test_true_literal() {
    let (p, root) = parse_ok("TRUE");
    let n = skip(p.arena(), root);
    assert_eq!(primary_value(p.arena(), n), "TRUE");
}

#[test]
fn test_false_literal() {
    let (p, root) = parse_ok("FALSE");
    assert_eq!(primary_value(p.arena(), root), "FALSE");
}

#[test]
fn test_null_literal() {
    let (p, root) = parse_ok("NULL");
    assert_eq!(primary_value(p.arena(), root), "NULL");
}

#[test]
fn test_true_case_insensitive() {
    let (p, root) = parse_ok("true");
    assert_eq!(primary_value(p.arena(), root), "true");
}

#[test]
fn test_false_mixed_case() {
    let (p, root) = parse_ok("False");
    assert_eq!(primary_value(p.arena(), root), "False");
}

// ================================================================
// POSITIVE TESTS: Variables
// ================================================================

#[test]
fn test_simple_variable() {
    let (p, root) = parse_ok("x");
    let n = skip(p.arena(), root);
    assert!(matches!(p.arena().get_node(n), AstNode::PrimaryExpr(_)));
    assert_eq!(primary_value(p.arena(), n), "x");
}

#[test]
fn test_underscore_variable() {
    let (p, root) = parse_ok("_foo");
    assert_eq!(primary_value(p.arena(), root), "_foo");
}

#[test]
fn test_variable_with_digits() {
    let (p, root) = parse_ok("col1");
    assert_eq!(primary_value(p.arena(), root), "col1");
}

#[test]
fn test_long_variable() {
    let (p, root) = parse_ok("my_long_variable_name_123");
    assert_eq!(primary_value(p.arena(), root), "my_long_variable_name_123");
}

// ================================================================
// POSITIVE TESTS: Equality operators
// ================================================================

#[test]
fn test_equal() {
    let (p, root) = parse_ok("a = 1");
    let n = skip(p.arena(), root);
    if let AstNode::EqualityExpression(eq) = p.arena().get_node(n) {
        assert_eq!(eq.operators, vec![EqualityOp::Equal]);
        assert_eq!(eq.children.len(), 2);
        assert_eq!(primary_value(p.arena(), eq.children[0]), "a");
        assert_eq!(primary_value(p.arena(), eq.children[1]), "1");
    } else {
        panic!("expected EqualityExpression");
    }
}

#[test]
fn test_not_equal() {
    let (p, root) = parse_ok("x <> 5");
    let n = skip(p.arena(), root);
    if let AstNode::EqualityExpression(eq) = p.arena().get_node(n) {
        assert_eq!(eq.operators, vec![EqualityOp::NotEqual]);
        assert_eq!(eq.children.len(), 2);
    } else {
        panic!("expected EqualityExpression");
    }
}

#[test]
fn test_not_equal_2() {
    let (p, root) = parse_ok("x != 5");
    let n = skip(p.arena(), root);
    if let AstNode::EqualityExpression(eq) = p.arena().get_node(n) {
        assert_eq!(eq.operators, vec![EqualityOp::NotEqual]);
        assert_eq!(eq.children.len(), 2);
    } else {
        panic!("expected EqualityExpression");
    }
}

#[test]
fn test_is_null() {
    let (p, root) = parse_ok("x IS NULL");
    let n = skip(p.arena(), root);
    if let AstNode::EqualityExpression(eq) = p.arena().get_node(n) {
        assert_eq!(eq.operators, vec![EqualityOp::IsNull]);
        assert_eq!(eq.children.len(), 1);
    } else {
        panic!("expected EqualityExpression");
    }
}

#[test]
fn test_is_not_null() {
    let (p, root) = parse_ok("y IS NOT NULL");
    let n = skip(p.arena(), root);
    if let AstNode::EqualityExpression(eq) = p.arena().get_node(n) {
        assert_eq!(eq.operators, vec![EqualityOp::IsNotNull]);
        assert_eq!(eq.children.len(), 1);
    } else {
        panic!("expected EqualityExpression");
    }
}

#[test]
fn test_is_null_case_insensitive() {
    let (p, root) = parse_ok("x is null");
    let n = skip(p.arena(), root);
    if let AstNode::EqualityExpression(eq) = p.arena().get_node(n) {
        assert_eq!(eq.operators, vec![EqualityOp::IsNull]);
    } else {
        panic!("expected EqualityExpression");
    }
}

#[test]
fn test_chained_equality() {
    let (p, root) = parse_ok("a = 1 = 2");
    let n = skip(p.arena(), root);
    if let AstNode::EqualityExpression(eq) = p.arena().get_node(n) {
        assert_eq!(eq.operators, vec![EqualityOp::Equal, EqualityOp::Equal]);
        assert_eq!(eq.children.len(), 3);
    } else {
        panic!("expected EqualityExpression");
    }
}

// ================================================================
// POSITIVE TESTS: Comparison operators
// ================================================================

#[test]
fn test_greater_than() {
    let (p, root) = parse_ok("a > 5");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::GreaterThan]);
        assert_eq!(cmp.children.len(), 2);
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_greater_than_equal() {
    let (p, root) = parse_ok("a >= 10");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::GreaterThanEqual]);
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_less_than() {
    let (p, root) = parse_ok("b < 3");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::LessThan]);
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_less_than_equal() {
    let (p, root) = parse_ok("b <= 99");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::LessThanEqual]);
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_like() {
    let (p, root) = parse_ok("name LIKE '%foo'");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::Like]);
        assert_eq!(cmp.children.len(), 2);
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_like_case_insensitive() {
    let (p, root) = parse_ok("name like '%bar'");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::Like]);
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_like_with_escape() {
    let (p, root) = parse_ok("name LIKE '%x' ESCAPE '\\'");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::LikeEscape]);
        assert_eq!(cmp.children.len(), 3); // variable, pattern, escape char
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_not_like() {
    let (p, root) = parse_ok("x NOT LIKE '%test'");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::NotLike]);
        assert_eq!(cmp.children.len(), 2);
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_not_like_with_escape() {
    let (p, root) = parse_ok("x NOT LIKE '%a' ESCAPE '!'");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::NotLikeEscape]);
        assert_eq!(cmp.children.len(), 3);
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_between() {
    let (p, root) = parse_ok("x BETWEEN 1 AND 10");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::Between]);
        assert_eq!(cmp.children.len(), 3); // value, low, high
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_not_between() {
    let (p, root) = parse_ok("y NOT BETWEEN 5 AND 20");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::NotBetween]);
        assert_eq!(cmp.children.len(), 3);
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_between_case_insensitive() {
    let (p, root) = parse_ok("x between 1 and 10");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::Between]);
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_in_single() {
    let (p, root) = parse_ok("x IN ('a')");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::In]);
        assert_eq!(cmp.children.len(), 2); // variable + 1 string
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_in_multiple() {
    let (p, root) = parse_ok("color IN ('red', 'green', 'blue')");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::In]);
        assert_eq!(cmp.children.len(), 4); // variable + 3 strings
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_not_in_single() {
    let (p, root) = parse_ok("x NOT IN ('z')");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::NotIn]);
        assert_eq!(cmp.children.len(), 2);
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_not_in_multiple() {
    let (p, root) = parse_ok("x NOT IN ('a', 'b')");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::NotIn]);
        assert_eq!(cmp.children.len(), 3);
    } else {
        panic!("expected ComparisonExpression");
    }
}

// ================================================================
// POSITIVE TESTS: Arithmetic operators
// ================================================================

#[test]
fn test_addition() {
    let (p, root) = parse_ok("a + b");
    let n = skip(p.arena(), root);
    if let AstNode::AddExpression(add) = p.arena().get_node(n) {
        assert_eq!(add.operators, vec![AddOp::Plus]);
        assert_eq!(add.children.len(), 2);
    } else {
        panic!("expected AddExpression, got {:?}", p.arena().get_node(n));
    }
}

#[test]
fn test_subtraction() {
    let (p, root) = parse_ok("a - b");
    let n = skip(p.arena(), root);
    if let AstNode::AddExpression(add) = p.arena().get_node(n) {
        assert_eq!(add.operators, vec![AddOp::Minus]);
    } else {
        panic!("expected AddExpression");
    }
}

#[test]
fn test_chained_add_sub() {
    let (p, root) = parse_ok("a + b - c + d");
    let n = skip(p.arena(), root);
    if let AstNode::AddExpression(add) = p.arena().get_node(n) {
        assert_eq!(add.operators, vec![AddOp::Plus, AddOp::Minus, AddOp::Plus]);
        assert_eq!(add.children.len(), 4);
    } else {
        panic!("expected AddExpression");
    }
}

#[test]
fn test_multiplication() {
    let (p, root) = parse_ok("a * b");
    let n = skip(p.arena(), root);
    if let AstNode::MultExpr(mul) = p.arena().get_node(n) {
        assert_eq!(mul.operators, vec![MultExprOp::Star]);
    } else {
        panic!("expected MultExpr");
    }
}

#[test]
fn test_division() {
    let (p, root) = parse_ok("a / b");
    let n = skip(p.arena(), root);
    if let AstNode::MultExpr(mul) = p.arena().get_node(n) {
        assert_eq!(mul.operators, vec![MultExprOp::Slash]);
    } else {
        panic!("expected MultExpr");
    }
}

#[test]
fn test_modulo() {
    let (p, root) = parse_ok("a % b");
    let n = skip(p.arena(), root);
    if let AstNode::MultExpr(mul) = p.arena().get_node(n) {
        assert_eq!(mul.operators, vec![MultExprOp::Percent]);
    } else {
        panic!("expected MultExpr");
    }
}

#[test]
fn test_chained_mult_div_mod() {
    let (p, root) = parse_ok("a * b / c % d");
    let n = skip(p.arena(), root);
    if let AstNode::MultExpr(mul) = p.arena().get_node(n) {
        assert_eq!(mul.operators, vec![MultExprOp::Star, MultExprOp::Slash, MultExprOp::Percent]);
        assert_eq!(mul.children.len(), 4);
    } else {
        panic!("expected MultExpr");
    }
}

// ================================================================
// POSITIVE TESTS: Unary operators
// ================================================================

#[test]
fn test_unary_plus() {
    let (p, root) = parse_ok("+x");
    let n = skip(p.arena(), root);
    if let AstNode::UnaryExpr(u) = p.arena().get_node(n) {
        assert_eq!(u.operator, Some(UnaryOp::Plus));
        assert_eq!(u.children.len(), 1);
    } else {
        panic!("expected UnaryExpr");
    }
}

#[test]
fn test_unary_negate() {
    let (p, root) = parse_ok("-x");
    let n = skip(p.arena(), root);
    if let AstNode::UnaryExpr(u) = p.arena().get_node(n) {
        assert_eq!(u.operator, Some(UnaryOp::Negate));
    } else {
        panic!("expected UnaryExpr");
    }
}

#[test]
fn test_unary_not() {
    let (p, root) = parse_ok("NOT x");
    let n = skip(p.arena(), root);
    if let AstNode::UnaryExpr(u) = p.arena().get_node(n) {
        assert_eq!(u.operator, Some(UnaryOp::Not));
    } else {
        panic!("expected UnaryExpr");
    }
}

#[test]
fn test_unary_not_case_insensitive() {
    let (p, root) = parse_ok("not x");
    let n = skip(p.arena(), root);
    if let AstNode::UnaryExpr(u) = p.arena().get_node(n) {
        assert_eq!(u.operator, Some(UnaryOp::Not));
    } else {
        panic!("expected UnaryExpr");
    }
}

#[test]
fn test_double_negate() {
    let (p, root) = parse_ok("--x");
    let n = skip(p.arena(), root);
    if let AstNode::UnaryExpr(u) = p.arena().get_node(n) {
        assert_eq!(u.operator, Some(UnaryOp::Negate));
        let inner = skip(p.arena(), u.children[0]);
        if let AstNode::UnaryExpr(u2) = p.arena().get_node(inner) {
            assert_eq!(u2.operator, Some(UnaryOp::Negate));
        } else {
            panic!("expected inner UnaryExpr");
        }
    } else {
        panic!("expected UnaryExpr");
    }
}

#[test]
fn test_not_not() {
    let (p, root) = parse_ok("NOT NOT x");
    let n = skip(p.arena(), root);
    if let AstNode::UnaryExpr(u) = p.arena().get_node(n) {
        assert_eq!(u.operator, Some(UnaryOp::Not));
        let inner = skip(p.arena(), u.children[0]);
        if let AstNode::UnaryExpr(u2) = p.arena().get_node(inner) {
            assert_eq!(u2.operator, Some(UnaryOp::Not));
        } else {
            panic!("expected inner UnaryExpr");
        }
    } else {
        panic!("expected UnaryExpr");
    }
}

// ================================================================
// POSITIVE TESTS: Logical operators (AND, OR)
// ================================================================

#[test]
fn test_and() {
    let (p, root) = parse_ok("a = 1 AND b = 2");
    let n = skip(p.arena(), root);
    if let AstNode::AndExpression(and) = p.arena().get_node(n) {
        assert_eq!(and.children.len(), 2);
    } else {
        panic!("expected AndExpression");
    }
}

#[test]
fn test_or() {
    let (p, root) = parse_ok("a = 1 OR b = 2");
    let n = skip(p.arena(), root);
    if let AstNode::OrExpression(or) = p.arena().get_node(n) {
        assert_eq!(or.children.len(), 2);
    } else {
        panic!("expected OrExpression");
    }
}

#[test]
fn test_multiple_and() {
    let (p, root) = parse_ok("a = 1 AND b = 2 AND c = 3");
    let n = skip(p.arena(), root);
    if let AstNode::AndExpression(and) = p.arena().get_node(n) {
        assert_eq!(and.children.len(), 3);
    } else {
        panic!("expected AndExpression");
    }
}

#[test]
fn test_multiple_or() {
    let (p, root) = parse_ok("a = 1 OR b = 2 OR c = 3");
    let n = skip(p.arena(), root);
    if let AstNode::OrExpression(or) = p.arena().get_node(n) {
        assert_eq!(or.children.len(), 3);
    } else {
        panic!("expected OrExpression");
    }
}

#[test]
fn test_and_case_insensitive() {
    let (p, root) = parse_ok("a = 1 and b = 2");
    let n = skip(p.arena(), root);
    assert!(matches!(p.arena().get_node(n), AstNode::AndExpression(_)));
}

#[test]
fn test_or_case_insensitive() {
    let (p, root) = parse_ok("a = 1 or b = 2");
    let n = skip(p.arena(), root);
    assert!(matches!(p.arena().get_node(n), AstNode::OrExpression(_)));
}

// ================================================================
// POSITIVE TESTS: Parenthesized expressions
// ================================================================

#[test]
fn test_parenthesized_variable() {
    let (p, root) = parse_ok("(x)");
    let n = skip(p.arena(), root);
    // Parenthesized expression: PrimaryExpr wrapping inner content
    assert!(matches!(p.arena().get_node(n), AstNode::PrimaryExpr(_)));
    // The inner chain is PrimaryExpr -> OrExpression(passthrough) -> ... -> Variable("x")
    if let AstNode::PrimaryExpr(pe) = p.arena().get_node(n) {
        assert_eq!(pe.children.len(), 1);
        assert_eq!(leaf_image(p.arena(), pe.children[0]), "x");
    }
}

#[test]
fn test_parenthesized_or() {
    let (p, root) = parse_ok("(a OR b) AND c = 1");
    let n = skip(p.arena(), root);
    if let AstNode::AndExpression(and) = p.arena().get_node(n) {
        assert_eq!(and.children.len(), 2);
        // First child should be a PrimaryExpr wrapping an OrExpression
        let first = skip(p.arena(), and.children[0]);
        assert!(matches!(p.arena().get_node(first), AstNode::PrimaryExpr(_)));
    } else {
        panic!("expected AndExpression");
    }
}

#[test]
fn test_nested_parentheses() {
    let (p, root) = parse_ok("((x))");
    // Nested parens: PrimaryExpr -> Or(pt) -> ... -> PrimaryExpr -> Or(pt) -> ... -> Variable("x")
    assert_eq!(leaf_image(p.arena(), root), "x");
}

#[test]
fn test_deeply_nested_parentheses() {
    let (p, root) = parse_ok("(((a = 1)))");
    let n = skip(p.arena(), root);
    // After skipping pass-throughs, should reach the equality
    if let AstNode::PrimaryExpr(pe) = p.arena().get_node(n) {
        let inner = skip(p.arena(), pe.children[0]);
        // The inner expression should eventually reach the EqualityExpression
        assert!(matches!(p.arena().get_node(inner), AstNode::PrimaryExpr(_) | AstNode::EqualityExpression(_)));
    } else if let AstNode::EqualityExpression(eq) = p.arena().get_node(n) {
        assert_eq!(eq.operators, vec![EqualityOp::Equal]);
    } else {
        // Either form is acceptable; just verify it parses
    }
}

// ================================================================
// POSITIVE TESTS: Precedence
// ================================================================

#[test]
fn test_or_lower_than_and() {
    // a OR b AND c => OrExpression(a, AndExpression(b, c))
    let (p, root) = parse_ok("a = 1 OR b = 2 AND c = 3");
    let n = skip(p.arena(), root);
    if let AstNode::OrExpression(or) = p.arena().get_node(n) {
        assert_eq!(or.children.len(), 2);
        // Second child of OR should be an AND
        let second = skip(p.arena(), or.children[1]);
        assert!(matches!(p.arena().get_node(second), AstNode::AndExpression(_)));
    } else {
        panic!("expected OrExpression");
    }
}

#[test]
fn test_mult_higher_than_add() {
    // a + b * c => AddExpression(a, MultExpr(b, c))
    let (p, root) = parse_ok("a + b * c");
    let n = skip(p.arena(), root);
    if let AstNode::AddExpression(add) = p.arena().get_node(n) {
        assert_eq!(add.operators, vec![AddOp::Plus]);
        let rhs = skip(p.arena(), add.children[1]);
        if let AstNode::MultExpr(mul) = p.arena().get_node(rhs) {
            assert_eq!(mul.operators, vec![MultExprOp::Star]);
        } else {
            panic!("expected MultExpr on rhs");
        }
    } else {
        panic!("expected AddExpression");
    }
}

#[test]
fn test_parens_override_precedence() {
    // (a + b) * c => MultExpr(PrimaryExpr(AddExpression(a,b)), c)
    let (p, root) = parse_ok("(a + b) * c");
    let n = skip(p.arena(), root);
    if let AstNode::MultExpr(mul) = p.arena().get_node(n) {
        assert_eq!(mul.operators, vec![MultExprOp::Star]);
        assert_eq!(mul.children.len(), 2);
    } else {
        panic!("expected MultExpr");
    }
}

#[test]
fn test_comparison_higher_than_equality() {
    // a > 5 = TRUE parsed as EqualityExpression(ComparisonExpression(a, 5), TRUE)
    let (p, root) = parse_ok("a > 5 = TRUE");
    let n = skip(p.arena(), root);
    if let AstNode::EqualityExpression(eq) = p.arena().get_node(n) {
        assert_eq!(eq.operators, vec![EqualityOp::Equal]);
        let lhs = skip(p.arena(), eq.children[0]);
        assert!(matches!(p.arena().get_node(lhs), AstNode::ComparisonExpression(_)));
    } else {
        panic!("expected EqualityExpression");
    }
}

#[test]
fn test_unary_higher_than_mult() {
    // -a * b => MultExpr(UnaryExpr(-,a), b)
    let (p, root) = parse_ok("-a * b");
    let n = skip(p.arena(), root);
    if let AstNode::MultExpr(mul) = p.arena().get_node(n) {
        assert_eq!(mul.operators, vec![MultExprOp::Star]);
        let lhs = skip(p.arena(), mul.children[0]);
        if let AstNode::UnaryExpr(u) = p.arena().get_node(lhs) {
            assert_eq!(u.operator, Some(UnaryOp::Negate));
        } else {
            panic!("expected UnaryExpr on lhs");
        }
    } else {
        panic!("expected MultExpr");
    }
}

// ================================================================
// POSITIVE TESTS: Complex/combined expressions
// ================================================================

#[test]
fn test_complex_and_or_equality_comparison() {
    let (p, root) = parse_ok("a = 1 AND b > 5 OR c < 10");
    let n = skip(p.arena(), root);
    assert!(matches!(p.arena().get_node(n), AstNode::OrExpression(_)));
}

#[test]
fn test_is_null_and_is_not_null_combined() {
    let (p, root) = parse_ok("x IS NULL AND y IS NOT NULL");
    let n = skip(p.arena(), root);
    if let AstNode::AndExpression(and) = p.arena().get_node(n) {
        assert_eq!(and.children.len(), 2);
        let first = skip(p.arena(), and.children[0]);
        let second = skip(p.arena(), and.children[1]);
        if let AstNode::EqualityExpression(eq1) = p.arena().get_node(first) {
            assert_eq!(eq1.operators, vec![EqualityOp::IsNull]);
        } else {
            panic!("expected EqualityExpression for first");
        }
        if let AstNode::EqualityExpression(eq2) = p.arena().get_node(second) {
            assert_eq!(eq2.operators, vec![EqualityOp::IsNotNull]);
        } else {
            panic!("expected EqualityExpression for second");
        }
    } else {
        panic!("expected AndExpression");
    }
}

#[test]
fn test_between_with_arithmetic_bounds() {
    // BETWEEN bounds must be literals, not expressions
    let msg = parse_err("x BETWEEN a + 1 AND b - 2");
    assert!(msg.contains("BETWEEN bounds must be literal values"), "msg was: {}", msg);
}

#[test]
fn test_in_with_and_or() {
    let (p, root) = parse_ok("a IN ('x', 'y') AND b = 1 OR c <> 2");
    let n = skip(p.arena(), root);
    assert!(matches!(p.arena().get_node(n), AstNode::OrExpression(_)));
}

#[test]
fn test_in_with_and_or_2() {
    let (p, root) = parse_ok("a IN ('x', 'y') AND b = 1 OR c != 2");
    let n = skip(p.arena(), root);
    assert!(matches!(p.arena().get_node(n), AstNode::OrExpression(_)));
}

#[test]
fn test_not_with_comparison() {
    // NOT binds tighter than >, so: (NOT a) > 5
    let (p, root) = parse_ok("NOT a > 5");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::GreaterThan]);
        // LHS should be UnaryExpr [NOT]
        let lhs = skip(p.arena(), cmp.children[0]);
        if let AstNode::UnaryExpr(u) = p.arena().get_node(lhs) {
            assert_eq!(u.operator, Some(UnaryOp::Not));
        } else {
            panic!("expected UnaryExpr on lhs");
        }
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_arithmetic_in_comparison() {
    let (p, root) = parse_ok("a * 2 + b >= c / 3 - d");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::GreaterThanEqual]);
        assert_eq!(cmp.children.len(), 2);
        let lhs = skip(p.arena(), cmp.children[0]);
        assert!(matches!(p.arena().get_node(lhs), AstNode::AddExpression(_)));
        let rhs = skip(p.arena(), cmp.children[1]);
        assert!(matches!(p.arena().get_node(rhs), AstNode::AddExpression(_)));
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_whitespace_variations() {
    // Extra whitespace should not affect parsing
    let (p1, r1) = parse_ok("a=1");
    let (p2, r2) = parse_ok("a = 1");
    let (p3, r3) = parse_ok("  a  =  1  ");
    let pp1 = p1.arena().pretty_print(r1, 0, "a=1");
    let pp2 = p2.arena().pretty_print(r2, 0, "a = 1");
    let pp3 = p3.arena().pretty_print(r3, 0, "  a  =  1  ");
    // Structure should be equivalent (ignoring the AST header line)
    let strip_header = |s: &str| s.lines().skip(1).collect::<Vec<_>>().join("\n");
    assert_eq!(strip_header(&pp1), strip_header(&pp2));
    assert_eq!(strip_header(&pp2), strip_header(&pp3));
}

#[test]
fn test_like_not_like_combined() {
    let (p, root) = parse_ok("a LIKE '%foo' AND b NOT LIKE '%bar'");
    let n = skip(p.arena(), root);
    if let AstNode::AndExpression(and) = p.arena().get_node(n) {
        assert_eq!(and.children.len(), 2);
        let first = skip(p.arena(), and.children[0]);
        if let AstNode::ComparisonExpression(c) = p.arena().get_node(first) {
            assert_eq!(c.operators, vec![ComparisonOp::Like]);
        } else {
            panic!("expected ComparisonExpression LIKE");
        }
        let second = skip(p.arena(), and.children[1]);
        if let AstNode::ComparisonExpression(c) = p.arena().get_node(second) {
            assert_eq!(c.operators, vec![ComparisonOp::NotLike]);
        } else {
            panic!("expected ComparisonExpression NOT LIKE");
        }
    } else {
        panic!("expected AndExpression");
    }
}

#[test]
fn test_between_and_not_between_combined() {
    let (p, root) = parse_ok("a BETWEEN 1 AND 10 OR b NOT BETWEEN 20 AND 30");
    let n = skip(p.arena(), root);
    if let AstNode::OrExpression(or) = p.arena().get_node(n) {
        assert_eq!(or.children.len(), 2);
        let first = skip(p.arena(), or.children[0]);
        if let AstNode::ComparisonExpression(c) = p.arena().get_node(first) {
            assert_eq!(c.operators, vec![ComparisonOp::Between]);
        } else {
            panic!("expected BETWEEN");
        }
        let second = skip(p.arena(), or.children[1]);
        if let AstNode::ComparisonExpression(c) = p.arena().get_node(second) {
            assert_eq!(c.operators, vec![ComparisonOp::NotBetween]);
        } else {
            panic!("expected NOT BETWEEN");
        }
    } else {
        panic!("expected OrExpression");
    }
}

#[test]
fn test_string_literal_in_equality() {
    let (p, root) = parse_ok("'hello' = 'world'");
    let n = skip(p.arena(), root);
    if let AstNode::EqualityExpression(eq) = p.arena().get_node(n) {
        assert_eq!(eq.operators, vec![EqualityOp::Equal]);
        assert_eq!(eq.children.len(), 2);
    } else {
        panic!("expected EqualityExpression");
    }
}

#[test]
fn test_numeric_literals_in_arithmetic() {
    let (p, root) = parse_ok("1 + 2.5 * 3");
    let n = skip(p.arena(), root);
    if let AstNode::AddExpression(add) = p.arena().get_node(n) {
        assert_eq!(add.operators, vec![AddOp::Plus]);
        assert_eq!(primary_value(p.arena(), add.children[0]), "1");
        let rhs = skip(p.arena(), add.children[1]);
        if let AstNode::MultExpr(mul) = p.arena().get_node(rhs) {
            assert_eq!(primary_value(p.arena(), mul.children[0]), "2.5");
            assert_eq!(primary_value(p.arena(), mul.children[1]), "3");
        } else {
            panic!("expected MultExpr on rhs");
        }
    } else {
        panic!("expected AddExpression");
    }
}

// ================================================================
// POSITIVE TESTS: Edge cases
// ================================================================

#[test]
fn test_keyword_prefix_as_variable() {
    // "NOTIFY" starts with "NOT" but should be parsed as ID
    let (p, root) = parse_ok("NOTIFY = 1");
    let n = skip(p.arena(), root);
    if let AstNode::EqualityExpression(eq) = p.arena().get_node(n) {
        assert_eq!(primary_value(p.arena(), eq.children[0]), "NOTIFY");
    } else {
        panic!("expected EqualityExpression");
    }
}

#[test]
fn test_identifier_starting_with_in() {
    let (p, root) = parse_ok("INSIDE = 1");
    let n = skip(p.arena(), root);
    if let AstNode::EqualityExpression(eq) = p.arena().get_node(n) {
        assert_eq!(primary_value(p.arena(), eq.children[0]), "INSIDE");
    } else {
        panic!("expected EqualityExpression");
    }
}

#[test]
fn test_identifier_starting_with_or() {
    let (p, root) = parse_ok("ORDER = 1");
    let n = skip(p.arena(), root);
    if let AstNode::EqualityExpression(eq) = p.arena().get_node(n) {
        assert_eq!(primary_value(p.arena(), eq.children[0]), "ORDER");
    } else {
        panic!("expected EqualityExpression");
    }
}

#[test]
fn test_identifier_starting_with_and() {
    let (p, root) = parse_ok("ANDROID = 1");
    let n = skip(p.arena(), root);
    if let AstNode::EqualityExpression(eq) = p.arena().get_node(n) {
        assert_eq!(primary_value(p.arena(), eq.children[0]), "ANDROID");
    } else {
        panic!("expected EqualityExpression");
    }
}

#[test]
fn test_null_used_as_rhs() {
    let (p, root) = parse_ok("x = NULL");
    let n = skip(p.arena(), root);
    if let AstNode::EqualityExpression(eq) = p.arena().get_node(n) {
        assert_eq!(eq.operators, vec![EqualityOp::Equal]);
        assert_eq!(primary_value(p.arena(), eq.children[1]), "NULL");
    } else {
        panic!("expected EqualityExpression");
    }
}

#[test]
fn test_true_as_rhs() {
    let (p, root) = parse_ok("x = TRUE");
    let n = skip(p.arena(), root);
    if let AstNode::EqualityExpression(eq) = p.arena().get_node(n) {
        assert_eq!(primary_value(p.arena(), eq.children[1]), "TRUE");
    } else {
        panic!("expected EqualityExpression");
    }
}

#[test]
fn test_false_as_rhs() {
    let (p, root) = parse_ok("x = FALSE");
    let n = skip(p.arena(), root);
    if let AstNode::EqualityExpression(eq) = p.arena().get_node(n) {
        assert_eq!(primary_value(p.arena(), eq.children[1]), "FALSE");
    } else {
        panic!("expected EqualityExpression");
    }
}

// ================================================================
// NEGATIVE TESTS: Parse errors
// ================================================================

#[test]
fn test_err_empty_input() {
    let msg = parse_err("");
    assert!(msg.contains("Expected expression"), "msg was: {}", msg);
}

#[test]
fn test_err_only_whitespace() {
    let msg = parse_err("   ");
    assert!(msg.contains("Expected expression"), "msg was: {}", msg);
}

#[test]
fn test_err_missing_rhs_of_equals() {
    let msg = parse_err("a =");
    assert!(msg.contains("Expected expression"), "msg was: {}", msg);
}

#[test]
fn test_err_missing_rhs_of_gt() {
    let msg = parse_err("a >");
    assert!(msg.contains("Expected expression"), "msg was: {}", msg);
}

#[test]
fn test_err_missing_rhs_of_lt() {
    let msg = parse_err("a <");
    // After consuming '<', it hits EOF where it expects an expression
    assert!(msg.contains("Expected expression") || msg.contains("Expected"), "msg was: {}", msg);
}

#[test]
fn test_err_trailing_and() {
    let msg = parse_err("a = 1 AND");
    assert!(msg.contains("Expected expression"), "msg was: {}", msg);
}

#[test]
fn test_err_trailing_or() {
    let msg = parse_err("a = 1 OR");
    assert!(msg.contains("Expected expression"), "msg was: {}", msg);
}

#[test]
fn test_err_unclosed_paren() {
    let msg = parse_err("(a = 1");
    assert!(msg.contains("Expected"), "msg was: {}", msg);
}

#[test]
fn test_err_extra_rparen() {
    let msg = parse_err("a = 1)");
    assert!(msg.contains("Expected") || msg.contains("found"), "msg was: {}", msg);
}

#[test]
fn test_err_double_operator() {
    let msg = parse_err("a = = 1");
    // The second = would be unexpected in the comparison production
    assert!(msg.contains("Expected") || msg.contains("found"), "msg was: {}", msg);
}

#[test]
fn test_err_like_without_string() {
    let msg = parse_err("a LIKE b");
    // LIKE expects a string literal, not an identifier
    assert!(msg.contains("Expected") || msg.contains("STRING_LITERAL"), "msg was: {}", msg);
}

#[test]
fn test_err_in_without_lparen() {
    let msg = parse_err("a IN 'x'");
    // IN expects '(' after it
    assert!(msg.contains("Expected"), "msg was: {}", msg);
}

#[test]
fn test_err_in_unclosed() {
    let msg = parse_err("a IN ('x', 'y'");
    assert!(msg.contains("Expected"), "msg was: {}", msg);
}

#[test]
fn test_err_in_empty_list() {
    let msg = parse_err("a IN ()");
    // Expects at least one literal value inside parens
    assert!(msg.contains("IN list elements must be literal values") || msg.contains("Expected"), "msg was: {}", msg);
}

#[test]
fn test_err_between_missing_and() {
    let msg = parse_err("a BETWEEN 1 OR 10");
    // BETWEEN expects AND between bounds
    assert!(msg.contains("Expected") || msg.contains("AND"), "msg was: {}", msg);
}

#[test]
fn test_err_between_missing_high() {
    let msg = parse_err("a BETWEEN 1 AND");
    assert!(msg.contains("BETWEEN bounds must be literal values") || msg.contains("Expected"), "msg was: {}", msg);
}

#[test]
fn test_err_between_missing_low() {
    // "a BETWEEN AND 10" - AND is not a valid literal bound
    let msg = parse_err("a BETWEEN AND 10");
    assert!(msg.contains("BETWEEN bounds must be literal values") || msg.contains("Expected"), "msg was: {}", msg);
}

#[test]
fn test_err_trailing_token() {
    let msg = parse_err("a = 1 b");
    // After parsing "a = 1", the parser expects EOF but finds "b"
    assert!(msg.contains("Expected") && msg.contains("EOF"), "msg was: {}", msg);
}

#[test]
fn test_err_unexpected_comma() {
    let msg = parse_err(", a");
    assert!(msg.contains("Expected expression") || msg.contains("Unexpected"), "msg was: {}", msg);
}

#[test]
fn test_err_is_without_null() {
    // "a IS 5" — IS expects NULL or NOT NULL
    let msg = parse_err("a IS 5");
    assert!(msg.contains("Expected"), "msg was: {}", msg);
}

#[test]
fn test_err_not_in_without_lparen() {
    // NOT IN requires ( after IN
    let msg = parse_err("a NOT IN 'x'");
    assert!(msg.contains("Expected"), "msg was: {}", msg);
}

#[test]
fn test_err_not_like_without_string() {
    let msg = parse_err("a NOT LIKE b");
    assert!(msg.contains("Expected") || msg.contains("STRING_LITERAL"), "msg was: {}", msg);
}

#[test]
fn test_err_unary_not_missing_operand() {
    let msg = parse_err("NOT");
    assert!(msg.contains("Expected expression"), "msg was: {}", msg);
}

#[test]
fn test_err_minus_missing_operand() {
    let msg = parse_err("-");
    assert!(msg.contains("Expected expression"), "msg was: {}", msg);
}

#[test]
fn test_err_plus_missing_operand() {
    let msg = parse_err("+");
    assert!(msg.contains("Expected expression"), "msg was: {}", msg);
}

#[test]
fn test_err_empty_parens() {
    let msg = parse_err("()");
    assert!(msg.contains("Expected expression"), "msg was: {}", msg);
}

#[test]
fn test_err_mismatched_parens() {
    let msg = parse_err(")(");
    assert!(msg.contains("Expected expression") || msg.contains("Unexpected"), "msg was: {}", msg);
}

// ================================================================
// POSITIVE TESTS: BETWEEN with literal bounds
// ================================================================

#[test]
fn test_between_string_bounds() {
    let (p, root) = parse_ok("x BETWEEN 'a' AND 'z'");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::Between]);
        assert_eq!(cmp.children.len(), 3);
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_between_float_bounds() {
    let (p, root) = parse_ok("x BETWEEN 1.5 AND 9.5");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::Between]);
        assert_eq!(cmp.children.len(), 3);
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_between_mixed_int_float_bounds() {
    // Integer and float mixing within numeric is OK
    let (p, root) = parse_ok("x BETWEEN 1 AND 9.5");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::Between]);
        assert_eq!(cmp.children.len(), 3);
    } else {
        panic!("expected ComparisonExpression");
    }
}

// ================================================================
// NEGATIVE TESTS: BETWEEN validation
// ================================================================

#[test]
fn test_between_type_mismatch() {
    let msg = parse_err("x BETWEEN 1 AND 'z'");
    assert!(msg.contains("same type"), "msg was: {}", msg);
}

#[test]
fn test_between_bounds_ordering() {
    let msg = parse_err("x BETWEEN 10 AND 1");
    assert!(msg.contains("lower bound") && msg.contains("upper bound"), "msg was: {}", msg);
}

#[test]
fn test_between_boolean_bound() {
    let msg = parse_err("x BETWEEN TRUE AND FALSE");
    assert!(msg.contains("boolean"), "msg was: {}", msg);
}

#[test]
fn test_between_null_bound() {
    let msg = parse_err("x BETWEEN NULL AND 10");
    assert!(msg.contains("NULL"), "msg was: {}", msg);
}

#[test]
fn test_between_variable_bound() {
    let msg = parse_err("x BETWEEN a AND b");
    assert!(msg.contains("literal values"), "msg was: {}", msg);
}

// ================================================================
// POSITIVE TESTS: IN with numeric literals
// ================================================================

#[test]
fn test_in_integer_list() {
    let (p, root) = parse_ok("x IN (1, 2, 3)");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::In]);
        assert_eq!(cmp.children.len(), 4); // variable + 3 integers
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_in_float_list() {
    let (p, root) = parse_ok("x IN (1.5, 2.5)");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::In]);
        assert_eq!(cmp.children.len(), 3); // variable + 2 floats
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_not_in_integer_list() {
    let (p, root) = parse_ok("x NOT IN (10, 20)");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::NotIn]);
        assert_eq!(cmp.children.len(), 3); // variable + 2 integers
    } else {
        panic!("expected ComparisonExpression");
    }
}

// ================================================================
// NEGATIVE TESTS: IN type checking
// ================================================================

#[test]
fn test_in_mixed_types_error() {
    let msg = parse_err("x IN (1, 'a')");
    assert!(msg.contains("same type"), "msg was: {}", msg);
}

#[test]
fn test_in_boolean_error() {
    let msg = parse_err("x IN (TRUE)");
    assert!(msg.contains("boolean"), "msg was: {}", msg);
}

#[test]
fn test_in_null_error() {
    let msg = parse_err("x IN (NULL)");
    assert!(msg.contains("NULL"), "msg was: {}", msg);
}

#[test]
fn test_in_int_float_mixed_error() {
    let msg = parse_err("x IN (1, 2.5)");
    assert!(msg.contains("same type"), "msg was: {}", msg);
}

// ========== Hex/Octal/Decimal interchangeability tests ==========

// ---------- Basic parsing of hex and octal literals ----------

#[test]
fn test_hex_literal() {
    let (p, root) = parse_ok("0xFF = 255");
    let n = skip(p.arena(), root);
    if let AstNode::EqualityExpression(eq) = p.arena().get_node(n) {
        assert_eq!(eq.operators, vec![EqualityOp::Equal]);
        assert_eq!(leaf_image(p.arena(), eq.children[0]), "0xFF");
        assert_eq!(leaf_image(p.arena(), eq.children[1]), "255");
    } else {
        panic!("expected EqualityExpression");
    }
}

#[test]
fn test_octal_literal() {
    let (p, root) = parse_ok("010 = 8");
    let n = skip(p.arena(), root);
    if let AstNode::EqualityExpression(eq) = p.arena().get_node(n) {
        assert_eq!(eq.operators, vec![EqualityOp::Equal]);
        assert_eq!(leaf_image(p.arena(), eq.children[0]), "010");
        assert_eq!(leaf_image(p.arena(), eq.children[1]), "8");
    } else {
        panic!("expected EqualityExpression");
    }
}

// ---------- Comparison operators with mixed formats ----------

#[test]
fn test_hex_greater_than_decimal() {
    let (p, root) = parse_ok("0xA > 5");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::GreaterThan]);
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_octal_less_than_hex() {
    let (p, root) = parse_ok("077 < 0xFF");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::LessThan]);
        assert_eq!(leaf_image(p.arena(), cmp.children[0]), "077");
        assert_eq!(leaf_image(p.arena(), cmp.children[1]), "0xFF");
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_decimal_gte_octal() {
    let (p, root) = parse_ok("100 >= 010");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::GreaterThanEqual]);
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_hex_lte_decimal() {
    let (p, root) = parse_ok("0x10 <= 20");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::LessThanEqual]);
    } else {
        panic!("expected ComparisonExpression");
    }
}

// ---------- Equality operators with mixed formats ----------

#[test]
fn test_hex_eq_octal() {
    let (p, root) = parse_ok("0xA = 012");
    let n = skip(p.arena(), root);
    if let AstNode::EqualityExpression(eq) = p.arena().get_node(n) {
        assert_eq!(eq.operators, vec![EqualityOp::Equal]);
        assert_eq!(leaf_image(p.arena(), eq.children[0]), "0xA");
        assert_eq!(leaf_image(p.arena(), eq.children[1]), "012");
    } else {
        panic!("expected EqualityExpression");
    }
}

#[test]
fn test_octal_ne_decimal() {
    let (p, root) = parse_ok("077 <> 100");
    let n = skip(p.arena(), root);
    if let AstNode::EqualityExpression(eq) = p.arena().get_node(n) {
        assert_eq!(eq.operators, vec![EqualityOp::NotEqual]);
    } else {
        panic!("expected EqualityExpression");
    }
}

// ---------- Arithmetic with mixed formats ----------

#[test]
fn test_hex_plus_decimal() {
    let (p, root) = parse_ok("0xA + 5 = 15");
    let n = skip(p.arena(), root);
    if let AstNode::EqualityExpression(eq) = p.arena().get_node(n) {
        let lhs = skip(p.arena(), eq.children[0]);
        if let AstNode::AddExpression(add) = p.arena().get_node(lhs) {
            assert_eq!(add.operators, vec![AddOp::Plus]);
            assert_eq!(leaf_image(p.arena(), add.children[0]), "0xA");
            assert_eq!(leaf_image(p.arena(), add.children[1]), "5");
        } else {
            panic!("expected AddExpression on LHS");
        }
    } else {
        panic!("expected EqualityExpression");
    }
}

#[test]
fn test_octal_minus_hex() {
    let (p, root) = parse_ok("0100 - 0x10 > 0");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::GreaterThan]);
        let lhs = skip(p.arena(), cmp.children[0]);
        if let AstNode::AddExpression(add) = p.arena().get_node(lhs) {
            assert_eq!(add.operators, vec![AddOp::Minus]);
        } else {
            panic!("expected AddExpression");
        }
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_hex_star_octal() {
    let (p, root) = parse_ok("0x2 * 010 = 16");
    let n = skip(p.arena(), root);
    if let AstNode::EqualityExpression(eq) = p.arena().get_node(n) {
        let lhs = skip(p.arena(), eq.children[0]);
        if let AstNode::MultExpr(mult) = p.arena().get_node(lhs) {
            assert_eq!(mult.operators, vec![MultExprOp::Star]);
        } else {
            panic!("expected MultExpr");
        }
    } else {
        panic!("expected EqualityExpression");
    }
}

#[test]
fn test_decimal_slash_hex() {
    let (p, root) = parse_ok("100 / 0xA > 0");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        let lhs = skip(p.arena(), cmp.children[0]);
        if let AstNode::MultExpr(mult) = p.arena().get_node(lhs) {
            assert_eq!(mult.operators, vec![MultExprOp::Slash]);
        } else {
            panic!("expected MultExpr");
        }
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_octal_percent_decimal() {
    let (p, root) = parse_ok("017 % 5 = 0");
    let n = skip(p.arena(), root);
    if let AstNode::EqualityExpression(eq) = p.arena().get_node(n) {
        let lhs = skip(p.arena(), eq.children[0]);
        if let AstNode::MultExpr(mult) = p.arena().get_node(lhs) {
            assert_eq!(mult.operators, vec![MultExprOp::Percent]);
        } else {
            panic!("expected MultExpr");
        }
    } else {
        panic!("expected EqualityExpression");
    }
}

// ---------- Unary operators with hex/octal ----------

#[test]
fn test_unary_negate_hex() {
    let (p, root) = parse_ok("-0xFF > 0");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        let lhs = skip(p.arena(), cmp.children[0]);
        if let AstNode::UnaryExpr(u) = p.arena().get_node(lhs) {
            assert_eq!(u.operator, Some(UnaryOp::Negate));
        } else {
            panic!("expected UnaryExpr");
        }
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_unary_plus_octal() {
    let (p, root) = parse_ok("+010 = 8");
    let n = skip(p.arena(), root);
    if let AstNode::EqualityExpression(eq) = p.arena().get_node(n) {
        let lhs = skip(p.arena(), eq.children[0]);
        if let AstNode::UnaryExpr(u) = p.arena().get_node(lhs) {
            assert_eq!(u.operator, Some(UnaryOp::Plus));
        } else {
            panic!("expected UnaryExpr");
        }
    } else {
        panic!("expected EqualityExpression");
    }
}

// ---------- BETWEEN with hex/octal bounds ----------

#[test]
fn test_between_hex_bounds() {
    let (p, root) = parse_ok("x BETWEEN 0xA AND 0xFF");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::Between]);
        assert_eq!(cmp.children.len(), 3);
        assert_eq!(leaf_image(p.arena(), cmp.children[1]), "0xA");
        assert_eq!(leaf_image(p.arena(), cmp.children[2]), "0xFF");
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_between_octal_bounds() {
    let (p, root) = parse_ok("x BETWEEN 010 AND 077");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::Between]);
        assert_eq!(cmp.children.len(), 3);
        assert_eq!(leaf_image(p.arena(), cmp.children[1]), "010");
        assert_eq!(leaf_image(p.arena(), cmp.children[2]), "077");
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_between_mixed_decimal_hex() {
    let (p, root) = parse_ok("x BETWEEN 10 AND 0xFF");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::Between]);
        assert_eq!(cmp.children.len(), 3);
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_between_mixed_octal_decimal() {
    let (p, root) = parse_ok("x BETWEEN 010 AND 100");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::Between]);
        assert_eq!(cmp.children.len(), 3);
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_between_mixed_hex_octal() {
    let (p, root) = parse_ok("x BETWEEN 0x1 AND 077");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::Between]);
        assert_eq!(cmp.children.len(), 3);
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_not_between_hex_bounds() {
    let (p, root) = parse_ok("x NOT BETWEEN 0x0 AND 0xF");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::NotBetween]);
        assert_eq!(cmp.children.len(), 3);
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_between_hex_bounds_ordering_error() {
    let msg = parse_err("x BETWEEN 0xFF AND 0xA");
    assert!(msg.contains("lower bound"), "msg was: {}", msg);
}

#[test]
fn test_between_octal_bounds_ordering_error() {
    let msg = parse_err("x BETWEEN 077 AND 010");
    assert!(msg.contains("lower bound"), "msg was: {}", msg);
}

// ---------- IN with hex/octal elements ----------

#[test]
fn test_in_hex_list() {
    let (p, root) = parse_ok("x IN (0xA, 0xB, 0xC)");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::In]);
        assert_eq!(cmp.children.len(), 4); // variable + 3 hex values
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_in_octal_list() {
    let (p, root) = parse_ok("x IN (010, 020, 030)");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::In]);
        assert_eq!(cmp.children.len(), 4); // variable + 3 octal values
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_in_mixed_decimal_hex() {
    let (p, root) = parse_ok("x IN (10, 0xFF, 42)");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::In]);
        assert_eq!(cmp.children.len(), 4);
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_in_mixed_decimal_octal() {
    let (p, root) = parse_ok("x IN (8, 010, 16)");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::In]);
        assert_eq!(cmp.children.len(), 4);
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_in_mixed_hex_octal() {
    let (p, root) = parse_ok("x IN (0xA, 012, 0xB)");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::In]);
        assert_eq!(cmp.children.len(), 4);
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_in_all_three_formats() {
    let (p, root) = parse_ok("x IN (42, 0xFF, 010)");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::In]);
        assert_eq!(cmp.children.len(), 4);
    } else {
        panic!("expected ComparisonExpression");
    }
}

#[test]
fn test_not_in_hex_list() {
    let (p, root) = parse_ok("x NOT IN (0xA, 0xB)");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::NotIn]);
        assert_eq!(cmp.children.len(), 3);
    } else {
        panic!("expected ComparisonExpression");
    }
}

// ---------- IN type errors: hex/octal cannot mix with float or string ----------

#[test]
fn test_in_hex_float_mixed_error() {
    let msg = parse_err("x IN (0xFF, 2.5)");
    assert!(msg.contains("same type"), "msg was: {}", msg);
}

#[test]
fn test_in_octal_float_mixed_error() {
    let msg = parse_err("x IN (010, 1.5)");
    assert!(msg.contains("same type"), "msg was: {}", msg);
}

#[test]
fn test_in_float_hex_mixed_error() {
    let msg = parse_err("x IN (1.5, 0xA)");
    assert!(msg.contains("same type"), "msg was: {}", msg);
}

#[test]
fn test_in_string_hex_mixed_error() {
    let msg = parse_err("x IN ('a', 0xA)");
    assert!(msg.contains("same type"), "msg was: {}", msg);
}

#[test]
fn test_in_hex_string_mixed_error() {
    let msg = parse_err("x IN (0xA, 'a')");
    assert!(msg.contains("same type"), "msg was: {}", msg);
}

// ---------- Parenthesized expressions with hex/octal ----------

#[test]
fn test_parenthesized_hex_arithmetic() {
    let (p, root) = parse_ok("(0xA + 010) > 5");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::GreaterThan]);
    } else {
        panic!("expected ComparisonExpression");
    }
}

// ---------- Complex expressions mixing all three formats ----------

#[test]
fn test_complex_all_formats_in_and_or() {
    let (p, root) = parse_ok("x = 0xFF AND y > 010 OR z < 42");
    let n = skip(p.arena(), root);
    assert!(matches!(p.arena().get_node(n), AstNode::OrExpression(_)));
}

#[test]
fn test_complex_between_and_in_mixed_formats() {
    let (p, root) = parse_ok("x BETWEEN 0xA AND 0xFF AND y IN (010, 020)");
    let n = skip(p.arena(), root);
    if let AstNode::AndExpression(and) = p.arena().get_node(n) {
        assert_eq!(and.children.len(), 2);
    } else {
        panic!("expected AndExpression");
    }
}

#[test]
fn test_complex_arithmetic_all_formats() {
    let (p, root) = parse_ok("0xA + 010 - 5 > 0");
    let n = skip(p.arena(), root);
    if let AstNode::ComparisonExpression(cmp) = p.arena().get_node(n) {
        assert_eq!(cmp.operators, vec![ComparisonOp::GreaterThan]);
        let lhs = skip(p.arena(), cmp.children[0]);
        if let AstNode::AddExpression(add) = p.arena().get_node(lhs) {
            assert_eq!(add.operators, vec![AddOp::Plus, AddOp::Minus]);
            assert_eq!(add.children.len(), 3);
            assert_eq!(leaf_image(p.arena(), add.children[0]), "0xA");
            assert_eq!(leaf_image(p.arena(), add.children[1]), "010");
            assert_eq!(leaf_image(p.arena(), add.children[2]), "5");
        } else {
            panic!("expected AddExpression");
        }
    } else {
        panic!("expected ComparisonExpression");
    }
}
