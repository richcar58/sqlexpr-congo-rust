# Fix Pretty Print

The arena.pretty_print() method does not print the operator when outputing nodes.  For example, the current output of main.rs is:

AST: "b > 5"
  ComparisonExpression
    PrimaryExpr
      Variable("b")
    PrimaryExpr
      Literal("5")

The desired output is:

AST: "b > 5"
  ComparisonExpression [>]
    PrimaryExpr
      Variable("b")
    PrimaryExpr
      Literal("5")

The goal is for pretty printing to display all information available in each AST node.  Please make a plan for review for enhanced pretty printing.      

# Pretty Print Test Generation

Please generate a set of tests that verify that pretty printing works as expected.  The each test should contain two logical operators (AND, OR, NOT) randomly chosen and at least two relational or comparison operators.  Every expression type should be tested at least once.  The pretty printed output should be validated against the expected output.  When "cargo test" is issued, only the standard summary results should be displayed.  When "cargo test -- --nocapture" is issued, then the pretty printed AST should be displayed.  Put all tests in the tests/pretty_parse_test.rs file.  Please update CLAUDE.md with relevant information.   

# SQL Boolean Expression Evaluation - Design Discussion

The next enhancement will allow SQL boolean expressions to be evaluated when all variables are bound to actual values.  Building on the current implementation that already parses SQL expression strings into ASTs of type BooleanExpression, we want the following new capabilities:

1. Allow the user to specify a mapping of variable names to actual values, which we will call the *value mapping*.
2. Use the value mapping and the user's parsed expression's AST to bind all free variables in the AST to values.
3. Validate that all variables in the AST are bound.
4. Use a new Evaluator data type to execute the logical expression represented by the AST and its variable bindings.
5. The evaluator's result is either true, false or an error, which is returned to the user.

Please suggest different design approaches for the above new capabilities.  Play special attention to capability 4, which focuses on the operational semantics that define how each AST is evaluated, how actual values are substituted for unbound variables, and how the final result is attained.  

# Pre-Evaluator Implementation Tasks

Before implementing Approach B: Evaluator Struct, let's get the codebase ready.

## Pre-Evaluator Task 1
Many of the TokenType enumerations have descriptive names like TokenType::AND and TokenType::BETWEEN.  These descriptive names clearly identify the language elements they represent.  Other values have generic names like TokenType::token3, which are not descriptive and whose assignment can change if new values are added to the enumeration.  Please generate descriptive UPPER_SNAKE_CASE names for all TokenType values that begin with "token".  For example, TokenType::Token19 would be replaced with TokenType::GE because it represents the greater-than symbol.  Change all code to conform to the renamed TokenType values.  Rerun tests to avoid any regression.

## Pre-Evaluator Task 2

Please generate tests/lexer_test.rs to comprehensively test the tokenization implemented in lexer.rs.  Please generate common usage cases and edge cases.  Generate positive and negative test cases. Compile and run the new tests.  Do not ignore any failed test cases and do not leave TODOs in the code.


# SQL Boolean Expression Evaluation - Plan

Let's proceed with Approach B where logical operations (AND, OR) short-circuit expression and subexpression evaluation as soon as possible. 

1. Use the content of evaluation.rs as the starting point for the public interface of the Evaluator.
    1, The **Evaluator** struct obviates the need for ValueMapping and EvalContext types as described in Approach 1.  
    1. The **RuntimeValue** enum provides the mechanism by which callers specify variable names and the values to which they should be bound. 
    2. The caller calls *evaluate(input: &str, map: &HashMap<String, RuntimeValue>)* to evaluate a boolean expression (input) using a value mapping (map).
    3. The **EvalError** type can be enhanced or redefined to improve error reporting.
2. Before an AST clause can be evaluated, all variable names in the clause need to be replaced with their value defined in the value mapping.
    1. Type checking can only be applied after substituting values for variable names.
    2. Evaluation fails with an error if a variable name cannot be mapped to an runtime value.
3. If possible, all code needed to evaluate an expression should be contained in evaluator.rs.
4. Careful attention must be paid to how literal values are mapped to Rust types for evaluation.  
    1. The Literal values that appear in the AST are defined in ast.rs.  The ValueLiteral::Integer values map to i64 in Rust; ValueLiteral::Float map to f64 in Rust; ValueLiteral::String map to String in Rust; BooleanTerm::BooleanLiteral map to bool in Rust.
    2. The literal values in RuntimeValue in evaluator.rs appear in the value mapping on the evaluate() call.  The RuntimeValue::Integer map to i64 in Rust; the RuntimeValue::Float map to f64 in Rust; RuntimeValue::String map to String in Rust; RuntimeValue::Boolean map to bool in Rust.
    3. ValueLiteral::Null and RuntimeValue::Null are case insensitive inputs that both map to the String "NULL" in Rust.
5. Type checking can be performed on operands on-demand before executing an operation.
    1. The ValueLiteral and RuntimeValue types that map to i64 in Rust are compatible.
    2. The ValueLiteral and RuntimeValue types that map to f64 in Rust are compatible.
    3. When mapping numeric types to Rust type, implement type coercion of integer (i64) to float (f64) only (1) on division operations or (2) on arimethic operations that contain both types.
    4. The ValueLiteral and RuntimeValue types that map to String in Rust are compatible.
    5. The ValueLiteral and RuntimeValue types that map to bool in Rust are compatible.
    6. The ValueLiteral and RuntimeValue Null types that map to "NULL" in Rust are compatible.
    7. It is an error for NULL to appear in comparison or arimethic operations.
6. Perform type checking on operands before executing the operation.
    1. Comparison and Equality operators (Gt, Lt, Gte, Lte, Like, Between, Eq, NotEq, etc.) can have numeric or string operand types, but not a mixture of these types in the same clause.
    2. Equality operators (Eq, NotEq) can also have boolean operand types in addition to numeric or string.
    3. Generate precise, human readable error messages when type errors are detected.
7. Correctness requires preserving operator precedence as described in the SqlExprParser-EBNF-Final.ebnf documentation.  The parser encodes these precedence rules into the AST structure it produces, so as long as the evaluator respects AST semantics, operator precedence will be preserved. 


# TODO

1. Enhance pretty printer to show operators on expression nodes.