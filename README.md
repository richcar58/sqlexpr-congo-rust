# sqlexpr-congo-rust

A Rust parser and evaluator for JMS SQL-like selector expressions, generated from a [CongoCC](https://congocc.org) grammar. The parser handles boolean, comparison, and arithmetic expressions with SQL keywords (`AND`, `OR`, `NOT`, `BETWEEN`, `LIKE`, `IN`, `IS NULL`, etc.).

Originally derived from an Apache ActiveMQ JMS message selector parser, the Rust code was generated from the CongoCC grammar file `SqlExprParser.ccc` using CongoCC's prototype Rust target support.

## Supported Syntax

```
-- Boolean operators
x > 5 AND y < 10
a = 1 OR b = 2
NOT active

-- Comparison operators
price >= 100
name <> 'admin'

-- Arithmetic in comparisons
(a + b) * c > 100

-- LIKE with wildcards (% and _) and optional ESCAPE
name LIKE 'John%'
code LIKE 'A_B%' ESCAPE '\'

-- BETWEEN (inclusive, bounds checked at parse time)
age BETWEEN 18 AND 65
temp BETWEEN -10.5 AND 100.0

-- IN lists (type-homogeneous)
status IN ('active', 'pending', 'completed')
code IN (100, 200, 300)

-- NULL checks
value IS NULL
value IS NOT NULL

-- Literals: integers, hex, octal, floats, scientific notation, long suffix
x = 42
flags = 0xFF
perms = 0755
rate = 3.14e-2
big = 1000000L

-- String literals with escaped quotes
name = 'It''s a test'

-- Identifiers with $ and _
$variable > 0
_internal = TRUE

-- Comments
x > 5 -- line comment
x /* block comment */ > 5
```

## Building

```bash
cargo build                    # Debug build
cargo build --release          # Release build
cargo build --features serde   # With serde serialization support for AST types
```

## Running Tests

```bash
cargo test                     # Run all 876 tests (summary output)
cargo test -- --nocapture      # Run with pretty-printed AST output visible
cargo test --test parser_test  # Run a specific test file
cargo clippy                   # Lint check
```

The test suite includes 876 tests across 7 test files covering the lexer, parser, evaluator, pretty-printer, and parse-time type checking.

## Usage

Add this crate as a dependency (using a path or git reference):

```toml
[dependencies]
parser = { path = "../sqlexpr-congo-rust" }
```

### Evaluating Expressions

The simplest way to use the library is through the `evaluate()` function, which parses and evaluates an expression in one step:

```rust
use std::collections::HashMap;
use parser::{evaluate, RuntimeValue, EvalError};

fn main() -> Result<(), EvalError> {
    // Define variable bindings
    let mut vars = HashMap::new();
    vars.insert("age".to_string(), RuntimeValue::Integer(25));
    vars.insert("name".to_string(), RuntimeValue::String("Alice".to_string()));
    vars.insert("active".to_string(), RuntimeValue::Boolean(true));

    // Evaluate expressions - returns Result<bool, EvalError>
    let result = evaluate("age >= 18 AND age <= 65", &vars)?;
    assert!(result);

    let result = evaluate("name LIKE 'A%' AND active", &vars)?;
    assert!(result);

    let result = evaluate("age IN (20, 25, 30)", &vars)?;
    assert!(result);

    let result = evaluate("name IS NOT NULL", &vars)?;
    assert!(result);

    Ok(())
}
```

`RuntimeValue` variants:
- `RuntimeValue::Integer(i64)`
- `RuntimeValue::Float(f64)`
- `RuntimeValue::String(String)`
- `RuntimeValue::Boolean(bool)`
- `RuntimeValue::Null`

### Parsing and Inspecting the AST

For lower-level access, use the `Parser` directly to get the AST:

```rust
use parser::{Parser, ParseError, AstNode, Arena, NodeId};

fn main() -> Result<(), ParseError> {
    let input = "age >= 18 AND name LIKE 'A%'";
    let mut parser = Parser::new(input.to_string())?;
    let root_id = parser.parse()?;

    // Pretty-print the AST
    let ast = parser.arena().pretty_print(root_id, 0, parser.input());
    println!("{}", ast);
    // Output:
    //   AST: "age >= 18 AND name LIKE 'A%'"
    //     AndExpression [AND x1]
    //       ComparisonExpression [>=]
    //         PrimaryExpr
    //           Variable("age")
    //         PrimaryExpr
    //           Literal("18")
    //       ComparisonExpression [LIKE]
    //         PrimaryExpr
    //           Variable("name")
    //         StringLiteral("'A%'")

    // Access nodes directly
    let node = parser.arena().get_node(root_id);
    if let AstNode::JmsSelector(selector) = node {
        println!("Root has {} children", selector.children.len());
    }

    Ok(())
}
```

### Traversing the AST

Use the visitor for depth-first traversal:

```rust
use parser::{Parser, Arena, AstNode, NodeId, VisitControl};

let input = "x > 5 AND y < 10";
let mut parser = Parser::new(input.to_string()).unwrap();
let root = parser.parse().unwrap();

// Collect all variable names
let mut variables = Vec::new();
parser.arena().visit(root, &mut |_id, node, arena, _depth, _opts| {
    if let AstNode::Variable(var) = node {
        let name = arena.get_token(var.begin_token).image.clone();
        variables.push(name);
    }
    VisitControl::Continue
}, None);

assert_eq!(variables, vec!["x", "y"]);
```

### Error Handling

Parse errors include position information:

```rust
use parser::{Parser, ParseError};

let result = Parser::new("x >".to_string())
    .and_then(|mut p| p.parse().map(|_| p));

if let Err(e) = result {
    println!("{}", e);
    // "Parse error at position 3: Expected expression, found EOF ''"
    println!("Position: {:?}", e.position);
    println!("Message: {}", e.message);
}
```

Evaluation errors are categorized:

```rust
use parser::{evaluate, EvalError, RuntimeValue};
use std::collections::HashMap;

let vars = HashMap::new();
let err = evaluate("x > 5", &vars).unwrap_err();
match err {
    EvalError::UnboundVariable { name } => println!("Missing variable: {}", name),
    EvalError::TypeError { operation, expected, actual, .. } => {
        println!("Type error in {}: expected {}, got {}", operation, expected, actual);
    }
    EvalError::DivisionByZero { .. } => println!("Division by zero"),
    EvalError::EvalParseError(msg) => println!("Parse error: {}", msg),
    _ => println!("Other error: {:?}", err),
}
```

## Project Layout

```
sqlexpr-congo-rust/
  SqlExprParser.ccc          # CongoCC grammar (source of truth for the language)
  Cargo.toml                 # Rust package manifest
  CLAUDE.md                  # Developer guidance for Claude Code
  src/
    lib.rs                   # Crate root: module declarations and public re-exports
    main.rs                  # Example binary
    tokens.rs                # TokenType enum and Token struct
    lexer.rs                 # Lexer/tokenizer
    parser.rs                # Recursive-descent parser
    arena.rs                 # Arena allocator, AstNode enum, node structs, pretty_print()
    evaluator.rs             # Expression evaluator
    visitor.rs               # Depth-first AST visitor
    error.rs                 # ParseError type
  tests/
    lexer_test.rs            # 129 lexer tests
    parser_test.rs           # 164 parser/AST structure tests
    parser_test2.rs          # 155 comprehensive parser feature tests
    parser_type_checking_tests.rs  # 97 BETWEEN/IN type checking tests
    pretty_parse_test.rs     # 10 pretty-print integration tests
    evaluator_test.rs        # 130 evaluator tests
    evaluator_test2.rs       # 191 evaluator tests with variable bindings
```

## AST Structure

The parser produces an arena-allocated AST. Each node is identified by a `NodeId` (a type-safe index). The expression hierarchy reflects operator precedence (lowest to highest):

| Node Type | Operators |
|-----------|-----------|
| `OrExpression` | `OR` |
| `AndExpression` | `AND` |
| `EqualityExpression` | `=`, `<>`, `IS NULL`, `IS NOT NULL` |
| `ComparisonExpression` | `>`, `>=`, `<`, `<=`, `LIKE`, `NOT LIKE`, `BETWEEN`, `NOT BETWEEN`, `IN`, `NOT IN` |
| `AddExpression` | `+`, `-` |
| `MultExpr` | `*`, `/`, `%` |
| `UnaryExpr` | `+` (prefix), `-` (negate), `NOT` |
| `PrimaryExpr` | Parenthesized expressions, literals, variables |
| `Literal` | `TRUE`, `FALSE`, `NULL`, numeric literals |
| `StringLiteral` | Single-quoted strings |
| `Variable` | Identifiers |

## Dependencies

- **No runtime dependencies** (zero-dependency by default)
- **Optional**: `serde` (1.0) with `derive` feature, enabled via the `serde` cargo feature for serializing/deserializing AST types

## Serde Support

Enable the `serde` feature to derive `Serialize` and `Deserialize` on all AST node types, tokens, and error types:

```toml
[dependencies]
parser = { path = "../sqlexpr-congo-rust", features = ["serde"] }
```

## Related Projects

- [CongoCC](https://congocc.org) - The parser generator used to produce this code
- [sqlexpr-rust](https://github.com/richcar58/sqlexpr-rust) - A related SQL expression parser project; several test suites were ported from there to validate parity

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Acknowledgments

Anthopic's Claude Opus 4.6 was used to generate most of the code and documentation in this project.

