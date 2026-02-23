/*
   Types which are used in multiple different places across the lexer, parser, and
   the evaluator, hence they are 'common'.
*/

#[derive(Debug, Clone, PartialEq)]
pub enum Boolean {
    True,
    False,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ComparisonOperator {
    LessThan,
    GreaterThan,
    EqualTo,
    NotEqualTo,
    LessThanOrEqualTo,
    GreaterThanOrEqualTo,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogicalOperation {
    And,
    Or,
    Not,
}
