use crate::{
    ast::expression::Expression,
    errors::ParserError,
    lexer::token::{OperatorKind, PuncuationKind, Token},
    parser::Parser,
};
use anyhow::{Context, Ok, Result};

impl Expression {
    pub fn parse(parser: &mut Parser) -> Result<Expression> {
        parse_expression(parser, 0)
    }
}

fn parse_prefix(parser: &mut Parser) -> Result<Expression> {
    let new = parser.consume()?;
    match new {
        Token::Number(num) => Ok(Expression::Literal { literal: num }),
        Token::Ident(ident) => Ok(Expression::Ident { identifier: ident }),
        Token::Operator(OperatorKind::Bang) => {
            let expr = parse_expression(parser, 8).context("Could not parse expression")?;
            Ok(Expression::Unary {
                operator: OperatorKind::Bang,
                operand: Box::new(expr),
            })
        }
        Token::Operator(OperatorKind::Minus) => {
            let expr = parse_expression(parser, 8).context("Could not parse expression")?;
            Ok(Expression::Unary {
                operator: OperatorKind::Minus,
                operand: Box::new(expr),
            })
        }
        Token::Punctuation(PuncuationKind::LeftParen) => {
            let expr = parse_expression(parser, 0)?;
            parser.expect(Token::Punctuation(PuncuationKind::RightParen))?;
            Ok(expr)
        }

        _ => {
            return Err(ParserError::UnexpectedToken { token: new.clone() }.into());
        }
    }
}

fn parse_expression(parser: &mut Parser, min_bp: u8) -> Result<Expression> {
    let mut lhs = parse_prefix(parser).context("Could not parse lhs")?;

    loop {
        let op = match parser.peek()? {
            Token::Operator(operator_kind) => operator_kind,
            _ => break,
        };
        let oprator = op.clone();

        let (l_bp, r_bp) = infix_binding_power(&op);
        if l_bp < min_bp {
            break;
        }
        parser.consume()?;
        let rhs = parse_expression(parser, r_bp).context("Could not parse rhs")?;
        lhs = Expression::Binary {
            left: Box::new(lhs),
            operator: oprator,
            right: Box::new(rhs),
        };
    }
    Ok(lhs)
}

fn infix_binding_power(op: &&OperatorKind) -> (u8, u8) {
    match op {
        // Assignment (right associative)
        OperatorKind::Equal => (1, 1),

        // Comparisons (non-associative)
        OperatorKind::EqualEqual | OperatorKind::Less | OperatorKind::Greater => (2, 3),

        // Addition / subtraction
        OperatorKind::Plus | OperatorKind::Minus => (4, 5),

        // Multiplication / division / modulo
        OperatorKind::Star | OperatorKind::Slash | OperatorKind::Percent => (6, 7),

        // Everything else MUST NOT bind
        _ => (0, 0),
    }
}
