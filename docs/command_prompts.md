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





# TODO

1. Enhance pretty printer to show operators on expression nodes.