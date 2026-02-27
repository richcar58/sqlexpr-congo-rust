use parser::Parser;

fn parse_and_print(input: &str) -> String {
    let mut p = Parser::new(input.to_string()).unwrap();
    let root = p.parse().unwrap();
    p.arena().pretty_print(root, 0, p.input())
}

/// Helper: parse, print (visible with --nocapture), and assert against expected output.
fn assert_pretty_print(input: &str, expected: &str) {
    let actual = parse_and_print(input);
    println!("{}", actual);
    assert_eq!(actual, expected, "Pretty print mismatch for input: {}", input);
}

// ---------- Tests ----------
// Each test uses two logical operators (AND, OR, NOT) and at least two
// relational/comparison operators. Together they cover every expression type
// and every operator variant at least once.

#[test]
fn test_and_or_equal_gt_lt() {
    assert_pretty_print(
        "a = 1 AND b > 5 OR c < 10",
        "\
AST: \"a = 1 AND b > 5 OR c < 10\"
  OrExpression [OR x1]
    AndExpression [AND x1]
      EqualityExpression [=]
        PrimaryExpr
          Variable(\"a\")
        PrimaryExpr
          Literal(\"1\")
      ComparisonExpression [>]
        PrimaryExpr
          Variable(\"b\")
        PrimaryExpr
          Literal(\"5\")
    ComparisonExpression [<]
      PrimaryExpr
        Variable(\"c\")
      PrimaryExpr
        Literal(\"10\")
",
    );
}

#[test]
fn test_is_null_is_not_null_not_equal() {
    assert_pretty_print(
        "x IS NULL AND y IS NOT NULL OR z <> 3",
        "\
AST: \"x IS NULL AND y IS NOT NULL OR z <> 3\"
  OrExpression [OR x1]
    AndExpression [AND x1]
      EqualityExpression [IS NULL]
        PrimaryExpr
          Variable(\"x\")
      EqualityExpression [IS NOT NULL]
        PrimaryExpr
          Variable(\"y\")
    EqualityExpression [<>]
      PrimaryExpr
        Variable(\"z\")
      PrimaryExpr
        Literal(\"3\")
",
    );
}

#[test]
fn test_gte_lte_not_unary_false_literal() {
    assert_pretty_print(
        "a >= 1 AND b <= 5 AND NOT c = FALSE",
        "\
AST: \"a >= 1 AND b <= 5 AND NOT c = FALSE\"
  AndExpression [AND x2]
    ComparisonExpression [>=]
      PrimaryExpr
        Variable(\"a\")
      PrimaryExpr
        Literal(\"1\")
    ComparisonExpression [<=]
      PrimaryExpr
        Variable(\"b\")
      PrimaryExpr
        Literal(\"5\")
    EqualityExpression [=]
      UnaryExpr [NOT]
        PrimaryExpr
          Variable(\"c\")
      PrimaryExpr
        Literal(\"FALSE\")
",
    );
}

#[test]
fn test_like_not_like_string_litteral() {
    assert_pretty_print(
        "a LIKE '%foo' OR b NOT LIKE '%bar' AND c > 1",
        "\
AST: \"a LIKE '%foo' OR b NOT LIKE '%bar' AND c > 1\"
  OrExpression [OR x1]
    ComparisonExpression [LIKE]
      PrimaryExpr
        Variable(\"a\")
      StringLitteral(\"'%foo'\")
    AndExpression [AND x1]
      ComparisonExpression [NOT LIKE]
        PrimaryExpr
          Variable(\"b\")
        StringLitteral(\"'%bar'\")
      ComparisonExpression [>]
        PrimaryExpr
          Variable(\"c\")
        PrimaryExpr
          Literal(\"1\")
",
    );
}

#[test]
fn test_between_not_between() {
    assert_pretty_print(
        "a BETWEEN 1 AND 10 AND b NOT BETWEEN 20 AND 30 OR c = 1",
        "\
AST: \"a BETWEEN 1 AND 10 AND b NOT BETWEEN 20 AND 30 OR c = 1\"
  OrExpression [OR x1]
    AndExpression [AND x1]
      ComparisonExpression [BETWEEN]
        PrimaryExpr
          Variable(\"a\")
        PrimaryExpr
          Literal(\"1\")
        PrimaryExpr
          Literal(\"10\")
      ComparisonExpression [NOT BETWEEN]
        PrimaryExpr
          Variable(\"b\")
        PrimaryExpr
          Literal(\"20\")
        PrimaryExpr
          Literal(\"30\")
    EqualityExpression [=]
      PrimaryExpr
        Variable(\"c\")
      PrimaryExpr
        Literal(\"1\")
",
    );
}

#[test]
fn test_in_not_in() {
    assert_pretty_print(
        "a IN ('x', 'y') AND b NOT IN ('z') OR c <> 1",
        "\
AST: \"a IN ('x', 'y') AND b NOT IN ('z') OR c <> 1\"
  OrExpression [OR x1]
    AndExpression [AND x1]
      ComparisonExpression [IN]
        PrimaryExpr
          Variable(\"a\")
        StringLitteral(\"'x'\")
        StringLitteral(\"'y'\")
      ComparisonExpression [NOT IN]
        PrimaryExpr
          Variable(\"b\")
        StringLitteral(\"'z'\")
    EqualityExpression [<>]
      PrimaryExpr
        Variable(\"c\")
      PrimaryExpr
        Literal(\"1\")
",
    );
}

#[test]
fn test_add_subtract_expressions() {
    assert_pretty_print(
        "a + b > 5 AND c - d < 10 OR NOT e = 1",
        "\
AST: \"a + b > 5 AND c - d < 10 OR NOT e = 1\"
  OrExpression [OR x1]
    AndExpression [AND x1]
      ComparisonExpression [>]
        AddExpression [+]
          PrimaryExpr
            Variable(\"a\")
          PrimaryExpr
            Variable(\"b\")
        PrimaryExpr
          Literal(\"5\")
      ComparisonExpression [<]
        AddExpression [-]
          PrimaryExpr
            Variable(\"c\")
          PrimaryExpr
            Variable(\"d\")
        PrimaryExpr
          Literal(\"10\")
    EqualityExpression [=]
      UnaryExpr [NOT]
        PrimaryExpr
          Variable(\"e\")
      PrimaryExpr
        Literal(\"1\")
",
    );
}

#[test]
fn test_mult_div_mod_expressions() {
    assert_pretty_print(
        "a * b >= 10 AND c / d <= 5 OR e % f <> 0",
        "\
AST: \"a * b >= 10 AND c / d <= 5 OR e % f <> 0\"
  OrExpression [OR x1]
    AndExpression [AND x1]
      ComparisonExpression [>=]
        MultExpr [*]
          PrimaryExpr
            Variable(\"a\")
          PrimaryExpr
            Variable(\"b\")
        PrimaryExpr
          Literal(\"10\")
      ComparisonExpression [<=]
        MultExpr [/]
          PrimaryExpr
            Variable(\"c\")
          PrimaryExpr
            Variable(\"d\")
        PrimaryExpr
          Literal(\"5\")
    EqualityExpression [<>]
      MultExpr [%]
        PrimaryExpr
          Variable(\"e\")
        PrimaryExpr
          Variable(\"f\")
      PrimaryExpr
        Literal(\"0\")
",
    );
}

#[test]
fn test_unary_minus_plus_not_true_literal() {
    assert_pretty_print(
        "-a > 0 AND NOT b = TRUE OR +c < 5",
        "\
AST: \"-a > 0 AND NOT b = TRUE OR +c < 5\"
  OrExpression [OR x1]
    AndExpression [AND x1]
      ComparisonExpression [>]
        UnaryExpr [-]
          PrimaryExpr
            Variable(\"a\")
        PrimaryExpr
          Literal(\"0\")
      EqualityExpression [=]
        UnaryExpr [NOT]
          PrimaryExpr
            Variable(\"b\")
        PrimaryExpr
          Literal(\"TRUE\")
    ComparisonExpression [<]
      UnaryExpr [+]
        PrimaryExpr
          Variable(\"c\")
      PrimaryExpr
        Literal(\"5\")
",
    );
}

#[test]
fn test_parenthesized_expression() {
    assert_pretty_print(
        "(a OR b) AND c = 1 AND d > 2",
        "\
AST: \"(a OR b) AND c = 1 AND d > 2\"
  AndExpression [AND x2]
    PrimaryExpr
      OrExpression [OR x1]
        PrimaryExpr
          Variable(\"a\")
        PrimaryExpr
          Variable(\"b\")
    EqualityExpression [=]
      PrimaryExpr
        Variable(\"c\")
      PrimaryExpr
        Literal(\"1\")
    ComparisonExpression [>]
      PrimaryExpr
        Variable(\"d\")
      PrimaryExpr
        Literal(\"2\")
",
    );
}
