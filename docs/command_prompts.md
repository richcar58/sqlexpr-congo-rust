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

## Pre-Evaluator Task 3

Please generate tests/parser_test.rs to comprehensively test the parsing implemented in parser.rs.  Please generate common usage cases and edge cases for each logical, relational, comparison and arithmetic operation.  Generate positive and negative test cases. Compile and run the new tests.  Do not ignore any failed test cases and do not leave TODOs in the code.


# SQL Boolean Expression Evaluation - Plan

Let's plan for an "Approach B: Evaluator Struct" implementation with these requirements:  

1. Use the content of evaluation.rs as the starting point for the public interface of the Evaluator.
    1, The **Evaluator** struct is used internally during AST processing.  
    1. The **RuntimeValue** enum provides the mechanism by which callers specify variable names and the values to which they should be bound. 
    2. The caller calls *evaluate(input: &str, map: &HashMap<String, RuntimeValue>)* to evaluate a boolean expression (input) using a value mapping (map), which is the mapping of variable names to actual values.
    3. The **EvalError** type can be enhanced or redefined to improve error reporting.
2. Logical AND and OR expression evaluation is short-circuited as soon as the outcome is known.
3. Before an AST clause can be evaluated, all variable names in the clause need to be replaced with their value defined in the value mapping.
    1. Type checking can only be applied after substituting values for variable names.
    2. Evaluation fails with an error if a variable name cannot be mapped to an runtime value.
4. If possible, all code needed to evaluate an expression should be contained in evaluator.rs.  Parsing enhancements can be implemented where appropriate.
5. Type checking during parsing allows some errors to be detected early (i.e., before evaluation), which allows for greater efficiency and simplified semantics.  Enhance parser type checking as follows:
    1. BETWEEN and NOT BETWEEN - The lower and upper bounds must either both be strings or both be numeric when an input string is submitted to the parser.  Numeric means either integer or float literals can be used separately or in combination.  The mixing of numeric types is acceptable since they are compatible for comparison purposes.  The Parser should check that (1) the lower and upper limits are always assigned, (2) the assignments are type compatible, and (3) that the lower bound is always less than or equal to the upper bound.  No other literal or expression values can be assigned to the lower and upper bounds.  During parsing, a clear and informative type error should be issued when the new restrictions are violated.
    2. IN and NOT IN - The only types that should appear in an elements list are integer, float or string literals.  Type checking should detect when non-literal, NULL or boolean elements are assigned and clearly report that as a type error.  Empty element lists are also an error.  Since all elements of a vector must be of the same Rust type, the three allowed literal types cannot be mixed:  All elements must be the exact same literal type (i.e., integers and floats cannot be mixed in element lists).  Rather than executing a separate type checking pass over all elements in a list, the first element in a list establishes the type against which all subsequent elements are checked.  
6. Careful attention must be paid to how literal values are mapped to Rust types for evaluation.  
    1. The literal values that appear in the AST are defined in arena.rs using tokens defined in tokens.rs using the following mappings:
        1. TokenType::DECIMAL_LITERAL, TokenType::HEX_LITERAL and TokenType::OCTAL_LITERAL values map to i64 in Rust 
        2. TokenType::FLOATING_POINT_LITERAL maps to f64 in Rust 
        3. TokenType::STRING_LITERAL maps to String in Rust 
        4. TokenType::TRUE and TokenType::FALSE map to bool in Rust
    2. The literal values in RuntimeValue in evaluator.rs appear in the value mapping on the evaluate() call using the following mappings:
        1. RuntimeValue::Integer map to i64 in Rust
        2. RuntimeValue::Float map to f64 in Rust
        3. RuntimeValue::String map to String in Rust
        4. RuntimeValue::Boolean map to bool in Rust
    3. TokenType::NULL and RuntimeValue::Null are case insensitive inputs that both map to the String "NULL" in Rust.
7. Due to variable substitution, some type checking must be performed on operands on-demand just before executing an operation. Types are compatible in the following situations:
    1. The TokenType and RuntimeValue types that map to i64 in Rust are compatible.
    2. The TokenType and RuntimeValue types that map to f64 in Rust are compatible.
    3. When mapping numeric types to Rust type, implement type coercion of integer (i64) to float (f64) only (1) on division operations or (2) on arimethic operations that contain both types.
    4. The TokenType and RuntimeValue types that map to String in Rust are compatible.
    5. The TokenType and RuntimeValue types that map to bool in Rust are compatible.
    6. The TokenType and RuntimeValue Null types that map to "NULL" in Rust are compatible.
    7. It is an error for NULL to appear in comparison or arimethic operations.
8. Perform type checking on operands before executing the operation.
    1. Comparison and Equality operators (GT, LT, GE, LE, LIKE, BETWEEN, IN, EQ, NE, etc.) can have numeric or string operand types, but not a mixture of these types in the same clause.
    2. Equality operators (EQ, NE) can also have boolean operand types in addition to numeric or string.
    3. Generate precise, human readable error messages when type errors are detected.
9. Correctness requires preserving operator precedence as described in the SqlExprParser-EBNF-Final.ebnf documentation.  The parser encodes these precedence rules into the AST structure it produces, so as long as the evaluator respects AST semantics, operator precedence will be preserved.
10. Generate tests/evaluation_test.rs that contains a comprehensive set of evaluation test cases, including positive and negative cases as well as typical and edge condition cases.  All tests must pass.
11. Leave no TODOs in the code. 

Please generate a plan to evaluate boolean expressions.

# TODO

1. DONE - Enhance pretty printer to show operators on expression nodes.
2. Document parser generation:  
    1. ~/git/sqlexpr-congo-rust$ java -jar /home/rich/git/congo-rust/congocc.jar -d src -lang rust SqlExprParser.ccc
    2. ~/git/sqlexpr-congo-rust$ cargo clean
    3. ~/git/sqlexpr-congo-rust$ cargo build --features serde
3. Document testing:
    1. :~/git/sqlexpr-congo-rust$ cargo test -- --nocapture
4. Generate tests that preserve operator precedence.