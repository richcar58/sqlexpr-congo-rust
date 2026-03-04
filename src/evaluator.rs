use crate::parser::Parser;
use crate::error::ParseError;
use crate::arena::*;
use crate::tokens::TokenType;

use std::collections::HashMap;
use std::fmt;

// ============================================================================
// PUBLIC API
// ============================================================================

/// User-provided values for variable substitution
#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
    /// 64-bit integer value
    Integer(i64),
    /// 64-bit floating point value
    Float(f64),
    /// String value
    String(String),
    /// Boolean value
    Boolean(bool),
    /// SQL NULL value
    Null,
}

/// Comprehensive error type for evaluation failures
#[derive(Debug, Clone, PartialEq)]
pub enum EvalError {
    /// Parse error from the parser
    EvalParseError(/// Error message
        String),

    /// Variable referenced but not found in value map
    UnboundVariable {
        /// Variable name
        name: String,
    },

    /// Type mismatch in operation
    TypeError {
        /// Operation that failed
        operation: String,
        /// Expected type
        expected: String,
        /// Actual type found
        actual: String,
        /// Expression context
        context: String,
    },

    /// NULL used in arithmetic or comparison operation (not IS NULL/IS NOT NULL)
    NullInOperation {
        /// Operation that failed
        operation: String,
        /// Expression context
        context: String,
    },

    /// Division by zero
    DivisionByZero {
        /// Expression that caused division by zero
        expression: String,
    },

    /// Invalid literal format
    InvalidLiteral {
        /// The literal value
        literal: String,
        /// The expected type
        literal_type: String,
        /// The parse error
        error: String,
    },
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::EvalParseError(msg) => write!(f, "Parse error: {}", msg),
            EvalError::UnboundVariable { name } => {
                write!(f, "Unbound variable '{}' - not found in value map", name)
            }
            EvalError::TypeError { operation, expected, actual, context } => {
                write!(f, "Type error in {}: expected {}, got {} (context: {})",
                    operation, expected, actual, context)
            }
            EvalError::NullInOperation { operation, context } => {
                write!(f, "NULL value in {} operation (context: {}). NULL is only allowed in IS NULL/IS NOT NULL",
                    operation, context)
            }
            EvalError::DivisionByZero { expression } => {
                write!(f, "Division by zero in expression: {}", expression)
            }
            EvalError::InvalidLiteral { literal, literal_type, error } => {
                write!(f, "Invalid {} literal '{}': {}", literal_type, literal, error)
            }
        }
    }
}

impl std::error::Error for EvalError {}

impl From<String> for EvalError {
    fn from(msg: String) -> Self {
        EvalError::EvalParseError(msg)
    }
}

impl From<&str> for EvalError {
    fn from(msg: &str) -> Self {
        EvalError::EvalParseError(msg.to_string())
    }
}

impl From<ParseError> for EvalError {
    fn from(msg: ParseError) -> Self {
        EvalError::EvalParseError(msg.to_string())
    }
}

/// Evaluate a SQL boolean expression string with variable bindings.
///
/// # Arguments
/// * `input` - SQL expression string to evaluate (must be boolean-valued)
/// * `map` - Variable name to value bindings for substitution
///
/// # Returns
/// * `Ok(bool)` - The evaluated boolean result
/// * `Err(EvalError)` - Error during parsing, variable resolution, type checking, or evaluation
///
/// # Examples
/// ```ignore
/// use std::collections::HashMap;
/// use parser::{evaluate, RuntimeValue};
///
/// let mut map = HashMap::new();
/// map.insert("x".to_string(), RuntimeValue::Integer(42));
///
/// let result = evaluate("x > 10", &map).unwrap();
/// assert_eq!(result, true);
/// ```
pub fn evaluate(input: &str, map: &HashMap<String, RuntimeValue>) -> Result<bool, EvalError> {
    let evaluator = Evaluator::new(input, map)?;
    let result = evaluator.eval_node(evaluator.root)?;
    match result {
        EvalValue::Bool(b) => Ok(b),
        other => Err(EvalError::TypeError {
            operation: "top-level evaluation".to_string(),
            expected: "boolean".to_string(),
            actual: other.type_name().to_string(),
            context: input.to_string(),
        }),
    }
}

// ============================================================================
// INTERNAL VALUE TYPE
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
enum EvalValue {
    Integer(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Null,
}

impl EvalValue {
    fn type_name(&self) -> &'static str {
        match self {
            EvalValue::Integer(_) => "integer",
            EvalValue::Float(_) => "float",
            EvalValue::Str(_) => "string",
            EvalValue::Bool(_) => "boolean",
            EvalValue::Null => "null",
        }
    }
}

impl From<&RuntimeValue> for EvalValue {
    fn from(rv: &RuntimeValue) -> Self {
        match rv {
            RuntimeValue::Integer(i) => EvalValue::Integer(*i),
            RuntimeValue::Float(f) => EvalValue::Float(*f),
            RuntimeValue::String(s) => EvalValue::Str(s.clone()),
            RuntimeValue::Boolean(b) => EvalValue::Bool(*b),
            RuntimeValue::Null => EvalValue::Null,
        }
    }
}

// ============================================================================
// EVALUATOR
// ============================================================================

struct Evaluator<'a> {
    parser: Parser,
    root: NodeId,
    value_map: &'a HashMap<String, RuntimeValue>,
}

impl<'a> Evaluator<'a> {
    fn new(input: &str, value_map: &'a HashMap<String, RuntimeValue>) -> Result<Self, EvalError> {
        let mut parser = Parser::new(input.to_string())?;
        let root = parser.parse()?;
        Ok(Evaluator {
            parser,
            root,
            value_map,
        })
    }

    fn arena(&self) -> &Arena {
        self.parser.arena()
    }

    fn input(&self) -> &str {
        self.parser.input()
    }

    // ========================================================================
    // CORE DISPATCH
    // ========================================================================

    fn eval_node(&self, node_id: NodeId) -> Result<EvalValue, EvalError> {
        match self.arena().get_node(node_id) {
            AstNode::JmsSelector(_) => self.eval_jms_selector(node_id),
            AstNode::OrExpression(_) => self.eval_or(node_id),
            AstNode::AndExpression(_) => self.eval_and(node_id),
            AstNode::EqualityExpression(_) => self.eval_equality(node_id),
            AstNode::ComparisonExpression(_) => self.eval_comparison(node_id),
            AstNode::AddExpression(_) => self.eval_add(node_id),
            AstNode::MultExpr(_) => self.eval_mult(node_id),
            AstNode::UnaryExpr(_) => self.eval_unary(node_id),
            AstNode::PrimaryExpr(_) => self.eval_primary(node_id),
            AstNode::Literal(_) => self.eval_literal(node_id),
            AstNode::StringLiteral(_) => self.eval_string_literal(node_id),
            AstNode::Variable(_) => self.eval_variable(node_id),
        }
    }

    // ========================================================================
    // NODE EVALUATION METHODS
    // ========================================================================

    fn eval_jms_selector(&self, node_id: NodeId) -> Result<EvalValue, EvalError> {
        let children = self.get_children(node_id);
        self.eval_node(children[0])
    }

    fn eval_or(&self, node_id: NodeId) -> Result<EvalValue, EvalError> {
        let children = self.get_children(node_id);
        // Single child: pass-through (no OR operator present)
        if children.len() == 1 {
            return self.eval_node(children[0]);
        }
        for child_id in &children {
            let val = self.eval_node(*child_id)?;
            match val {
                EvalValue::Bool(true) => return Ok(EvalValue::Bool(true)),
                EvalValue::Bool(false) => {}
                other => {
                    return Err(EvalError::TypeError {
                        operation: "OR".to_string(),
                        expected: "boolean".to_string(),
                        actual: other.type_name().to_string(),
                        context: self.input().to_string(),
                    });
                }
            }
        }
        Ok(EvalValue::Bool(false))
    }

    fn eval_and(&self, node_id: NodeId) -> Result<EvalValue, EvalError> {
        let children = self.get_children(node_id);
        // Single child: pass-through (no AND operator present)
        if children.len() == 1 {
            return self.eval_node(children[0]);
        }
        for child_id in &children {
            let val = self.eval_node(*child_id)?;
            match val {
                EvalValue::Bool(false) => return Ok(EvalValue::Bool(false)),
                EvalValue::Bool(true) => {}
                other => {
                    return Err(EvalError::TypeError {
                        operation: "AND".to_string(),
                        expected: "boolean".to_string(),
                        actual: other.type_name().to_string(),
                        context: self.input().to_string(),
                    });
                }
            }
        }
        Ok(EvalValue::Bool(true))
    }

    fn eval_equality(&self, node_id: NodeId) -> Result<EvalValue, EvalError> {
        let (children, operators) = match self.arena().get_node(node_id) {
            AstNode::EqualityExpression(n) => (n.children.clone(), n.operators.clone()),
            other => return Err(EvalError::EvalParseError(
                format!("Internal error: expected EqualityExpression node, found {:?}", other),
            )),
        };

        let mut current = self.eval_node(children[0])?;
        let mut child_idx = 1;

        for op in &operators {
            match op {
                EqualityOp::Equal => {
                    let right = self.eval_node(children[child_idx])?;
                    child_idx += 1;
                    current = self.eval_eq_values(&current, &right, "Equal")?;
                }
                EqualityOp::NotEqual => {
                    let right = self.eval_node(children[child_idx])?;
                    child_idx += 1;
                    let eq_result = self.eval_eq_values(&current, &right, "NotEqual")?;
                    current = match eq_result {
                        EvalValue::Bool(b) => EvalValue::Bool(!b),
                        other => return Err(EvalError::EvalParseError(
                            format!("Internal error: equality comparison returned non-boolean {:?}", other.type_name()),
                        )),
                    };
                }
                EqualityOp::IsNull => {
                    current = EvalValue::Bool(matches!(current, EvalValue::Null));
                }
                EqualityOp::IsNotNull => {
                    current = EvalValue::Bool(!matches!(current, EvalValue::Null));
                }
            }
        }

        Ok(current)
    }

    fn eval_comparison(&self, node_id: NodeId) -> Result<EvalValue, EvalError> {
        let (children, operators) = match self.arena().get_node(node_id) {
            AstNode::ComparisonExpression(n) => (n.children.clone(), n.operators.clone()),
            other => return Err(EvalError::EvalParseError(
                format!("Internal error: expected ComparisonExpression node, found {:?}", other),
            )),
        };

        let mut current = self.eval_node(children[0])?;
        let mut child_idx = 1;

        for op in &operators {
            match op {
                ComparisonOp::GreaterThan
                | ComparisonOp::GreaterThanEqual
                | ComparisonOp::LessThan
                | ComparisonOp::LessThanEqual => {
                    let right = self.eval_node(children[child_idx])?;
                    child_idx += 1;
                    current = self.eval_ordering_comparison(&current, &right, *op)?;
                }
                ComparisonOp::Like | ComparisonOp::NotLike => {
                    let pattern = self.eval_node(children[child_idx])?;
                    child_idx += 1;
                    current = self.eval_like(&current, &pattern, None, *op)?;
                }
                ComparisonOp::LikeEscape | ComparisonOp::NotLikeEscape => {
                    let pattern = self.eval_node(children[child_idx])?;
                    child_idx += 1;
                    let escape = self.eval_node(children[child_idx])?;
                    child_idx += 1;
                    let escape_char = match &escape {
                        EvalValue::Str(s) => {
                            if s.len() != 1 {
                                return Err(EvalError::TypeError {
                                    operation: "LIKE ESCAPE".to_string(),
                                    expected: "single character string".to_string(),
                                    actual: format!("string of length {}", s.len()),
                                    context: self.input().to_string(),
                                });
                            }
                            Some(s.chars().next().unwrap())
                        }
                        _ => {
                            return Err(EvalError::TypeError {
                                operation: "LIKE ESCAPE".to_string(),
                                expected: "string".to_string(),
                                actual: escape.type_name().to_string(),
                                context: self.input().to_string(),
                            });
                        }
                    };
                    let base_op = if *op == ComparisonOp::LikeEscape {
                        ComparisonOp::Like
                    } else {
                        ComparisonOp::NotLike
                    };
                    current = self.eval_like(&current, &pattern, escape_char, base_op)?;
                }
                ComparisonOp::Between | ComparisonOp::NotBetween => {
                    let low = self.eval_node(children[child_idx])?;
                    child_idx += 1;
                    let high = self.eval_node(children[child_idx])?;
                    child_idx += 1;
                    current = self.eval_between(&current, &low, &high, *op)?;
                }
                ComparisonOp::In | ComparisonOp::NotIn => {
                    let elements: Vec<EvalValue> = children[child_idx..]
                        .iter()
                        .map(|id| self.eval_node(*id))
                        .collect::<Result<Vec<_>, _>>()?;
                    child_idx = children.len();
                    current = self.eval_in(&current, &elements, *op)?;
                }
            }
        }

        Ok(current)
    }

    fn eval_add(&self, node_id: NodeId) -> Result<EvalValue, EvalError> {
        let (children, operators) = match self.arena().get_node(node_id) {
            AstNode::AddExpression(n) => (n.children.clone(), n.operators.clone()),
            other => return Err(EvalError::EvalParseError(
                format!("Internal error: expected AddExpression node, found {:?}", other),
            )),
        };

        let mut result = self.eval_node(children[0])?;
        for (i, op) in operators.iter().enumerate() {
            let right = self.eval_node(children[i + 1])?;
            result = self.eval_arithmetic_add(&result, &right, *op)?;
        }
        Ok(result)
    }

    fn eval_mult(&self, node_id: NodeId) -> Result<EvalValue, EvalError> {
        let (children, operators) = match self.arena().get_node(node_id) {
            AstNode::MultExpr(n) => (n.children.clone(), n.operators.clone()),
            other => return Err(EvalError::EvalParseError(
                format!("Internal error: expected MultExpr node, found {:?}", other),
            )),
        };

        let mut result = self.eval_node(children[0])?;
        for (i, op) in operators.iter().enumerate() {
            let right = self.eval_node(children[i + 1])?;
            result = self.eval_arithmetic_mult(&result, &right, *op)?;
        }
        Ok(result)
    }

    fn eval_unary(&self, node_id: NodeId) -> Result<EvalValue, EvalError> {
        let (children, operator) = match self.arena().get_node(node_id) {
            AstNode::UnaryExpr(n) => (n.children.clone(), n.operator),
            other => return Err(EvalError::EvalParseError(
                format!("Internal error: expected UnaryExpr node, found {:?}", other),
            )),
        };

        let child_val = self.eval_node(children[0])?;
        match operator {
            None => Ok(child_val),
            Some(UnaryOp::Plus) => {
                match &child_val {
                    EvalValue::Integer(_) | EvalValue::Float(_) => Ok(child_val),
                    EvalValue::Null => Err(EvalError::NullInOperation {
                        operation: "unary +".to_string(),
                        context: self.input().to_string(),
                    }),
                    _ => Err(EvalError::TypeError {
                        operation: "unary +".to_string(),
                        expected: "numeric".to_string(),
                        actual: child_val.type_name().to_string(),
                        context: self.input().to_string(),
                    }),
                }
            }
            Some(UnaryOp::Negate) => {
                match &child_val {
                    EvalValue::Integer(i) => Ok(EvalValue::Integer(-i)),
                    EvalValue::Float(f) => Ok(EvalValue::Float(-f)),
                    EvalValue::Null => Err(EvalError::NullInOperation {
                        operation: "unary minus".to_string(),
                        context: self.input().to_string(),
                    }),
                    _ => Err(EvalError::TypeError {
                        operation: "unary minus".to_string(),
                        expected: "numeric".to_string(),
                        actual: child_val.type_name().to_string(),
                        context: self.input().to_string(),
                    }),
                }
            }
            Some(UnaryOp::Not) => {
                match &child_val {
                    EvalValue::Bool(b) => Ok(EvalValue::Bool(!b)),
                    _ => Err(EvalError::TypeError {
                        operation: "NOT".to_string(),
                        expected: "boolean".to_string(),
                        actual: child_val.type_name().to_string(),
                        context: self.input().to_string(),
                    }),
                }
            }
        }
    }

    fn eval_primary(&self, node_id: NodeId) -> Result<EvalValue, EvalError> {
        let children = self.get_children(node_id);
        self.eval_node(children[0])
    }

    fn eval_literal(&self, node_id: NodeId) -> Result<EvalValue, EvalError> {
        let node = match self.arena().get_node(node_id) {
            AstNode::Literal(n) => n.clone(),
            other => return Err(EvalError::EvalParseError(
                format!("Internal error: expected Literal node, found {:?}", other),
            )),
        };

        if !node.children.is_empty() {
            // Has StringLiteral child
            return self.eval_node(node.children[0]);
        }

        let token = self.arena().get_token(node.begin_token);
        let image = &token.image;
        match token.token_type {
            TokenType::TRUE => Ok(EvalValue::Bool(true)),
            TokenType::FALSE => Ok(EvalValue::Bool(false)),
            TokenType::NULL => Ok(EvalValue::Null),
            TokenType::FLOATING_POINT_LITERAL => {
                let f: f64 = image.parse().map_err(|e: std::num::ParseFloatError| {
                    EvalError::InvalidLiteral {
                        literal: image.clone(),
                        literal_type: "float".to_string(),
                        error: e.to_string(),
                    }
                })?;
                Ok(EvalValue::Float(f))
            }
            TokenType::DECIMAL_LITERAL => {
                let clean = image.strip_suffix('L').or_else(|| image.strip_suffix('l')).unwrap_or(image);
                if clean.contains('.') {
                    let f: f64 = clean.parse().map_err(|e: std::num::ParseFloatError| {
                        EvalError::InvalidLiteral {
                            literal: image.clone(),
                            literal_type: "float".to_string(),
                            error: e.to_string(),
                        }
                    })?;
                    Ok(EvalValue::Float(f))
                } else {
                    let i: i64 = clean.parse().map_err(|e: std::num::ParseIntError| {
                        EvalError::InvalidLiteral {
                            literal: image.clone(),
                            literal_type: "integer".to_string(),
                            error: e.to_string(),
                        }
                    })?;
                    Ok(EvalValue::Integer(i))
                }
            }
            TokenType::HEX_LITERAL => {
                let clean = image.strip_suffix('L').or_else(|| image.strip_suffix('l')).unwrap_or(image);
                let hex_str = clean.strip_prefix("0x")
                    .or_else(|| clean.strip_prefix("0X"))
                    .unwrap_or(clean);
                let i = i64::from_str_radix(hex_str, 16).map_err(|e| {
                    EvalError::InvalidLiteral {
                        literal: image.clone(),
                        literal_type: "hex".to_string(),
                        error: e.to_string(),
                    }
                })?;
                Ok(EvalValue::Integer(i))
            }
            TokenType::OCTAL_LITERAL => {
                let clean = image.strip_suffix('L').or_else(|| image.strip_suffix('l')).unwrap_or(image);
                let oct_str = clean.strip_prefix('0').unwrap_or(clean);
                let i = i64::from_str_radix(oct_str, 8).map_err(|e| {
                    EvalError::InvalidLiteral {
                        literal: image.clone(),
                        literal_type: "octal".to_string(),
                        error: e.to_string(),
                    }
                })?;
                Ok(EvalValue::Integer(i))
            }
            _ => Err(EvalError::InvalidLiteral {
                literal: image.clone(),
                literal_type: format!("{:?}", token.token_type),
                error: "unsupported literal type".to_string(),
            }),
        }
    }

    fn eval_string_literal(&self, node_id: NodeId) -> Result<EvalValue, EvalError> {
        let node = match self.arena().get_node(node_id) {
            AstNode::StringLiteral(n) => n.clone(),
            other => return Err(EvalError::EvalParseError(
                format!("Internal error: expected StringLiteral node, found {:?}", other),
            )),
        };

        let token = self.arena().get_token(node.begin_token);
        let image = &token.image;
        // Strip surrounding single quotes
        let inner = &image[1..image.len() - 1];
        Ok(EvalValue::Str(inner.to_string()))
    }

    fn eval_variable(&self, node_id: NodeId) -> Result<EvalValue, EvalError> {
        let node = match self.arena().get_node(node_id) {
            AstNode::Variable(n) => n.clone(),
            other => return Err(EvalError::EvalParseError(
                format!("Internal error: expected Variable node, found {:?}", other),
            )),
        };

        let token = self.arena().get_token(node.begin_token);
        let name = &token.image;

        match self.value_map.get(name) {
            Some(rv) => Ok(EvalValue::from(rv)),
            None => Err(EvalError::UnboundVariable {
                name: name.clone(),
            }),
        }
    }

    // ========================================================================
    // TYPE CHECKING AND ARITHMETIC HELPERS
    // ========================================================================

    fn eval_eq_values(&self, left: &EvalValue, right: &EvalValue, op_name: &str) -> Result<EvalValue, EvalError> {
        match (left, right) {
            (EvalValue::Null, _) | (_, EvalValue::Null) => {
                Err(EvalError::NullInOperation {
                    operation: op_name.to_string(),
                    context: self.input().to_string(),
                })
            }
            (EvalValue::Integer(a), EvalValue::Integer(b)) => Ok(EvalValue::Bool(a == b)),
            (EvalValue::Float(a), EvalValue::Float(b)) => Ok(EvalValue::Bool(a == b)),
            (EvalValue::Integer(a), EvalValue::Float(b)) => Ok(EvalValue::Bool((*a as f64) == *b)),
            (EvalValue::Float(a), EvalValue::Integer(b)) => Ok(EvalValue::Bool(*a == (*b as f64))),
            (EvalValue::Str(a), EvalValue::Str(b)) => Ok(EvalValue::Bool(a == b)),
            (EvalValue::Bool(a), EvalValue::Bool(b)) => Ok(EvalValue::Bool(a == b)),
            _ => Err(EvalError::TypeError {
                operation: op_name.to_string(),
                expected: left.type_name().to_string(),
                actual: right.type_name().to_string(),
                context: self.input().to_string(),
            }),
        }
    }

    fn eval_ordering_comparison(&self, left: &EvalValue, right: &EvalValue, op: ComparisonOp) -> Result<EvalValue, EvalError> {
        let op_name = match op {
            ComparisonOp::GreaterThan => "GreaterThan",
            ComparisonOp::GreaterThanEqual => "GreaterThanEqual",
            ComparisonOp::LessThan => "LessThan",
            ComparisonOp::LessThanEqual => "LessThanEqual",
            other => return Err(EvalError::EvalParseError(
                format!("Internal error: unexpected operator {:?} in ordering comparison", other),
            )),
        };

        match (left, right) {
            (EvalValue::Null, _) | (_, EvalValue::Null) => {
                Err(EvalError::NullInOperation {
                    operation: op_name.to_string(),
                    context: self.input().to_string(),
                })
            }
            (EvalValue::Integer(a), EvalValue::Integer(b)) => {
                Ok(EvalValue::Bool(apply_ordering(*a, *b, op)?))
            }
            (EvalValue::Float(a), EvalValue::Float(b)) => {
                Ok(EvalValue::Bool(apply_ordering_f64(*a, *b, op)?))
            }
            (EvalValue::Integer(a), EvalValue::Float(b)) => {
                Ok(EvalValue::Bool(apply_ordering_f64(*a as f64, *b, op)?))
            }
            (EvalValue::Float(a), EvalValue::Integer(b)) => {
                Ok(EvalValue::Bool(apply_ordering_f64(*a, *b as f64, op)?))
            }
            (EvalValue::Str(a), EvalValue::Str(b)) => {
                Ok(EvalValue::Bool(apply_ordering(a.as_str(), b.as_str(), op)?))
            }
            _ => Err(EvalError::TypeError {
                operation: op_name.to_string(),
                expected: left.type_name().to_string(),
                actual: right.type_name().to_string(),
                context: self.input().to_string(),
            }),
        }
    }

    fn eval_arithmetic_add(&self, left: &EvalValue, right: &EvalValue, op: AddOp) -> Result<EvalValue, EvalError> {
        let op_name = match op {
            AddOp::Plus => "addition",
            AddOp::Minus => "subtraction",
        };

        match (left, right) {
            (EvalValue::Null, _) | (_, EvalValue::Null) => {
                Err(EvalError::NullInOperation {
                    operation: op_name.to_string(),
                    context: self.input().to_string(),
                })
            }
            (EvalValue::Integer(a), EvalValue::Integer(b)) => {
                match op {
                    AddOp::Plus => Ok(EvalValue::Integer(a + b)),
                    AddOp::Minus => Ok(EvalValue::Integer(a - b)),
                }
            }
            (EvalValue::Float(a), EvalValue::Float(b)) => {
                match op {
                    AddOp::Plus => Ok(EvalValue::Float(a + b)),
                    AddOp::Minus => Ok(EvalValue::Float(a - b)),
                }
            }
            (EvalValue::Integer(a), EvalValue::Float(b)) => {
                let a = *a as f64;
                match op {
                    AddOp::Plus => Ok(EvalValue::Float(a + b)),
                    AddOp::Minus => Ok(EvalValue::Float(a - b)),
                }
            }
            (EvalValue::Float(a), EvalValue::Integer(b)) => {
                let b = *b as f64;
                match op {
                    AddOp::Plus => Ok(EvalValue::Float(a + b)),
                    AddOp::Minus => Ok(EvalValue::Float(a - b)),
                }
            }
            _ => Err(EvalError::TypeError {
                operation: op_name.to_string(),
                expected: "numeric".to_string(),
                actual: format!("{} and {}", left.type_name(), right.type_name()),
                context: self.input().to_string(),
            }),
        }
    }

    fn eval_arithmetic_mult(&self, left: &EvalValue, right: &EvalValue, op: MultExprOp) -> Result<EvalValue, EvalError> {
        let op_name = match op {
            MultExprOp::Star => "*",
            MultExprOp::Slash => "/",
            MultExprOp::Percent => "%",
        };

        match (left, right) {
            (EvalValue::Null, _) | (_, EvalValue::Null) => {
                Err(EvalError::NullInOperation {
                    operation: op_name.to_string(),
                    context: self.input().to_string(),
                })
            }
            (EvalValue::Integer(a), EvalValue::Integer(b)) => {
                match op {
                    MultExprOp::Star => Ok(EvalValue::Integer(a * b)),
                    MultExprOp::Slash => {
                        if *b == 0 {
                            return Err(EvalError::DivisionByZero {
                                expression: self.input().to_string(),
                            });
                        }
                        Ok(EvalValue::Float(*a as f64 / *b as f64))
                    }
                    MultExprOp::Percent => {
                        if *b == 0 {
                            return Err(EvalError::DivisionByZero {
                                expression: self.input().to_string(),
                            });
                        }
                        Ok(EvalValue::Integer(a % b))
                    }
                }
            }
            (EvalValue::Float(a), EvalValue::Float(b)) => {
                match op {
                    MultExprOp::Star => Ok(EvalValue::Float(a * b)),
                    MultExprOp::Slash => {
                        if *b == 0.0 {
                            return Err(EvalError::DivisionByZero {
                                expression: self.input().to_string(),
                            });
                        }
                        Ok(EvalValue::Float(a / b))
                    }
                    MultExprOp::Percent => {
                        if *b == 0.0 {
                            return Err(EvalError::DivisionByZero {
                                expression: self.input().to_string(),
                            });
                        }
                        Ok(EvalValue::Float(a % b))
                    }
                }
            }
            (EvalValue::Integer(a), EvalValue::Float(b)) => {
                let a = *a as f64;
                match op {
                    MultExprOp::Star => Ok(EvalValue::Float(a * b)),
                    MultExprOp::Slash => {
                        if *b == 0.0 {
                            return Err(EvalError::DivisionByZero {
                                expression: self.input().to_string(),
                            });
                        }
                        Ok(EvalValue::Float(a / b))
                    }
                    MultExprOp::Percent => {
                        if *b == 0.0 {
                            return Err(EvalError::DivisionByZero {
                                expression: self.input().to_string(),
                            });
                        }
                        Ok(EvalValue::Float(a % b))
                    }
                }
            }
            (EvalValue::Float(a), EvalValue::Integer(b)) => {
                let b = *b as f64;
                match op {
                    MultExprOp::Star => Ok(EvalValue::Float(a * b)),
                    MultExprOp::Slash => {
                        if b == 0.0 {
                            return Err(EvalError::DivisionByZero {
                                expression: self.input().to_string(),
                            });
                        }
                        Ok(EvalValue::Float(a / b))
                    }
                    MultExprOp::Percent => {
                        if b == 0.0 {
                            return Err(EvalError::DivisionByZero {
                                expression: self.input().to_string(),
                            });
                        }
                        Ok(EvalValue::Float(a % b))
                    }
                }
            }
            _ => Err(EvalError::TypeError {
                operation: op_name.to_string(),
                expected: "numeric".to_string(),
                actual: format!("{} and {}", left.type_name(), right.type_name()),
                context: self.input().to_string(),
            }),
        }
    }

    fn eval_between(&self, value: &EvalValue, low: &EvalValue, high: &EvalValue, op: ComparisonOp) -> Result<EvalValue, EvalError> {
        let between_name = match op {
            ComparisonOp::Between => "BETWEEN",
            ComparisonOp::NotBetween => "NOT BETWEEN",
            other => return Err(EvalError::EvalParseError(
                format!("Internal error: unexpected operator {:?} in BETWEEN evaluation", other),
            )),
        };
        // Check for NULL at the BETWEEN level so the error names the right operation
        if matches!(value, EvalValue::Null) || matches!(low, EvalValue::Null) || matches!(high, EvalValue::Null) {
            return Err(EvalError::NullInOperation {
                operation: between_name.to_string(),
                context: self.input().to_string(),
            });
        }
        // low <= value AND value <= high
        let ge_low = self.eval_ordering_comparison(value, low, ComparisonOp::GreaterThanEqual)?;
        let le_high = self.eval_ordering_comparison(value, high, ComparisonOp::LessThanEqual)?;

        let result = match (&ge_low, &le_high) {
            (EvalValue::Bool(a), EvalValue::Bool(b)) => *a && *b,
            _ => return Err(EvalError::EvalParseError(
                format!("Internal error: BETWEEN comparison returned non-boolean results ({}, {})",
                    ge_low.type_name(), le_high.type_name()),
            )),
        };

        match op {
            ComparisonOp::Between => Ok(EvalValue::Bool(result)),
            ComparisonOp::NotBetween => Ok(EvalValue::Bool(!result)),
            other => Err(EvalError::EvalParseError(
                format!("Internal error: unexpected operator {:?} in BETWEEN result", other),
            )),
        }
    }

    fn eval_in(&self, value: &EvalValue, elements: &[EvalValue], op: ComparisonOp) -> Result<EvalValue, EvalError> {
        // Check that the value type is compatible with the list element type.
        // Null is handled by eval_eq_values (produces NullInOperation).
        // Integer and Float are compatible (numeric coercion).
        if let Some(first) = elements.first()
            && !matches!(value, EvalValue::Null) && !Self::in_types_compatible(value, first) {
                return Err(EvalError::TypeError {
                    operation: "IN".to_string(),
                    expected: first.type_name().to_string(),
                    actual: value.type_name().to_string(),
                    context: self.input().to_string(),
                });
            }
        for elem in elements {
            let eq = self.eval_eq_values(value, elem, "IN")?;
            if eq == EvalValue::Bool(true) {
                return match op {
                    ComparisonOp::In => Ok(EvalValue::Bool(true)),
                    ComparisonOp::NotIn => Ok(EvalValue::Bool(false)),
                    other => Err(EvalError::EvalParseError(
                        format!("Internal error: unexpected operator {:?} in IN evaluation", other),
                    )),
                };
            }
        }
        match op {
            ComparisonOp::In => Ok(EvalValue::Bool(false)),
            ComparisonOp::NotIn => Ok(EvalValue::Bool(true)),
            other => Err(EvalError::EvalParseError(
                format!("Internal error: unexpected operator {:?} in IN evaluation", other),
            )),
        }
    }

    fn eval_like(&self, value: &EvalValue, pattern: &EvalValue, escape: Option<char>, op: ComparisonOp) -> Result<EvalValue, EvalError> {
        let op_name = match op {
            ComparisonOp::Like => "LIKE",
            ComparisonOp::NotLike => "NOT LIKE",
            other => return Err(EvalError::EvalParseError(
                format!("Internal error: unexpected operator {:?} in LIKE evaluation", other),
            )),
        };

        match (value, pattern) {
            (EvalValue::Null, _) | (_, EvalValue::Null) => {
                Err(EvalError::NullInOperation {
                    operation: op_name.to_string(),
                    context: self.input().to_string(),
                })
            }
            (EvalValue::Str(val), EvalValue::Str(pat)) => {
                let matched = sql_like_match(val, pat, escape);
                match op {
                    ComparisonOp::Like => Ok(EvalValue::Bool(matched)),
                    ComparisonOp::NotLike => Ok(EvalValue::Bool(!matched)),
                    other => Err(EvalError::EvalParseError(
                        format!("Internal error: unexpected operator {:?} in LIKE result", other),
                    )),
                }
            }
            _ => Err(EvalError::TypeError {
                operation: op_name.to_string(),
                expected: "string".to_string(),
                actual: format!("{} and {}", value.type_name(), pattern.type_name()),
                context: self.input().to_string(),
            }),
        }
    }

    // ========================================================================
    // UTILITY
    // ========================================================================

    /// Check if a value type is compatible with an IN list element type.
    /// Integer and Float are mutually compatible (numeric coercion).
    fn in_types_compatible(a: &EvalValue, b: &EvalValue) -> bool {
        matches!((a, b),
            (EvalValue::Integer(_), EvalValue::Integer(_))
            | (EvalValue::Float(_), EvalValue::Float(_))
            | (EvalValue::Integer(_), EvalValue::Float(_))
            | (EvalValue::Float(_), EvalValue::Integer(_))
            | (EvalValue::Str(_), EvalValue::Str(_))
            | (EvalValue::Bool(_), EvalValue::Bool(_))
        )
    }

    fn get_children(&self, node_id: NodeId) -> Vec<NodeId> {
        match self.arena().get_node(node_id) {
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
}

// ============================================================================
// ORDERING HELPERS
// ============================================================================

fn apply_ordering<T: PartialOrd>(a: T, b: T, op: ComparisonOp) -> Result<bool, EvalError> {
    match op {
        ComparisonOp::GreaterThan => Ok(a > b),
        ComparisonOp::GreaterThanEqual => Ok(a >= b),
        ComparisonOp::LessThan => Ok(a < b),
        ComparisonOp::LessThanEqual => Ok(a <= b),
        other => Err(EvalError::EvalParseError(
            format!("Internal error: unexpected operator {:?} in ordering comparison", other),
        )),
    }
}

fn apply_ordering_f64(a: f64, b: f64, op: ComparisonOp) -> Result<bool, EvalError> {
    match op {
        ComparisonOp::GreaterThan => Ok(a > b),
        ComparisonOp::GreaterThanEqual => Ok(a >= b),
        ComparisonOp::LessThan => Ok(a < b),
        ComparisonOp::LessThanEqual => Ok(a <= b),
        other => Err(EvalError::EvalParseError(
            format!("Internal error: unexpected operator {:?} in ordering comparison", other),
        )),
    }
}

// ============================================================================
// SQL LIKE PATTERN MATCHING
// ============================================================================

#[derive(Debug)]
enum PatternElement {
    Percent,
    Underscore,
    Literal(char),
}

fn sql_like_match(value: &str, pattern: &str, escape: Option<char>) -> bool {
    // Pre-process pattern into elements
    let mut elements = Vec::new();
    let mut chars = pattern.chars().peekable();

    while let Some(c) = chars.next() {
        if Some(c) == escape {
            // Next char is a literal
            if let Some(next) = chars.next() {
                elements.push(PatternElement::Literal(next));
            }
        } else if c == '%' {
            elements.push(PatternElement::Percent);
        } else if c == '_' {
            elements.push(PatternElement::Underscore);
        } else {
            elements.push(PatternElement::Literal(c));
        }
    }

    // DP matching
    let val_chars: Vec<char> = value.chars().collect();
    let n = val_chars.len();
    let m = elements.len();

    // dp[i][j] = true if val[0..i] matches pattern[0..j]
    let mut dp = vec![vec![false; m + 1]; n + 1];
    dp[0][0] = true;

    // Handle leading % patterns
    for j in 1..=m {
        if matches!(elements[j - 1], PatternElement::Percent) {
            dp[0][j] = dp[0][j - 1];
        } else {
            break;
        }
    }

    for i in 1..=n {
        for j in 1..=m {
            match &elements[j - 1] {
                PatternElement::Percent => {
                    // % matches zero or more characters
                    dp[i][j] = dp[i][j - 1] || dp[i - 1][j];
                }
                PatternElement::Underscore => {
                    // _ matches exactly one character
                    dp[i][j] = dp[i - 1][j - 1];
                }
                PatternElement::Literal(c) => {
                    dp[i][j] = dp[i - 1][j - 1] && val_chars[i - 1] == *c;
                }
            }
        }
    }

    dp[n][m]
}
