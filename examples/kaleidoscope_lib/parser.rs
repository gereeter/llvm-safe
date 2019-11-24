use std::iter::Peekable;

use kaleidoscope_lib::lexer::Token;
use kaleidoscope_lib::ast::{Expr, Prototype, Function};

/// identifierexpr
///   ::= identifier
///   ::= identifier '(' expression* ')'
pub fn parse_identifier_expr<I: Iterator<Item=Token>>(iter: &mut Peekable<I>) -> Result<Expr, &'static str> {
    if let Some(Token::Identifier(name)) = iter.next() {
        if iter.peek() != Some(&Token::Other('(')) {
            return Ok(Expr::Variable(name));
        }

        assert_eq!(iter.next(), Some(Token::Other('(')));

        let mut args = Vec::new();
        if iter.peek() != Some(&Token::Other(')')) {
            loop {
                args.push(parse_expr(iter)?);
                
                if iter.peek() == Some(&Token::Other(')')) {
                    break;
                } else if iter.next() != Some(Token::Other(',')) {
                    return Err("Expected ')' or ',' in argument list")
                }
            }
        }

        assert_eq!(iter.next(), Some(Token::Other(')')));

        Ok(Expr::Call(name, args))
    } else {
        Err("Expected identifier")
    }
}

/// numberexpr ::= number
pub fn parse_number_expr<I: Iterator<Item=Token>>(iter: &mut Peekable<I>) -> Result<Expr, &'static str> {
    if let Some(Token::Number(val)) = iter.next() {
        Ok(Expr::Number(val))
    } else {
        Err("Expected number")
    }
}

/// parenexpr ::= '(' expression ')'
pub fn parse_paren_expr<I: Iterator<Item=Token>>(iter: &mut Peekable<I>) -> Result<Expr, &'static str> {
    if let Some(Token::Other('(')) = iter.next() {
        let ret = parse_expr(iter)?;
        
        if let Some(Token::Other(')')) = iter.next() {
            Ok(ret)
        } else {
            Err("Expected ')'")
        }
    } else {
        Err("Expected '('")
    }
}

/// ifexpr ::= 'if' expression 'then' expression 'else' expression
pub fn parse_if_expr<I: Iterator<Item=Token>>(iter: &mut Peekable<I>) -> Result<Expr, &'static str> {
    if let Some(Token::If) = iter.next() {
        let cond_expr = parse_expr(iter)?;
        if iter.next() != Some(Token::Then) {
            return Err("Expected 'then'");
        }
        let then_expr = parse_expr(iter)?;
        if iter.next() != Some(Token::Else) {
            return Err("Expected 'else'");
        }
        let else_expr = parse_expr(iter)?;
        Ok(Expr::If(Box::new(cond_expr), Box::new(then_expr), Box::new(else_expr)))
    } else {
        Err("Expected 'if'")
    }
}

/// primary
///   ::= identifierexpr
///   ::= numberexpr
///   ::= parenexpr
///   ::= ifexpr
pub fn parse_primary<I: Iterator<Item=Token>>(iter: &mut Peekable<I>) -> Result<Expr, &'static str> {
    match iter.peek() {
        Some(&Token::Identifier(_)) => parse_identifier_expr(iter),
        Some(&Token::Number(_)) => parse_number_expr(iter),
        Some(&Token::Other('(')) => parse_paren_expr(iter),
        Some(&Token::If) => parse_if_expr(iter),
        None => Err("Unexpected EOF when expecting expression"),
        _ => {
            iter.next();
            Err("Unexpected token when expecting expression")
        }
    }
}

/// expression
///   ::= primary binoprhs
///
pub fn parse_expr<I: Iterator<Item=Token>>(iter: &mut Peekable<I>) -> Result<Expr, &'static str> {
    let lhs = parse_primary(iter)?;
    parse_binop_rhs(iter, 0, lhs)
}

// In precedence order
const BINOPS: [char; 6] = ['<', '>', '+', '-', '*', '/'];

/// binoprhs
///   ::= ('+' primary)*
pub fn parse_binop_rhs<I: Iterator<Item=Token>>(iter: &mut Peekable<I>, precedence: usize, mut lhs: Expr) -> Result<Expr, &'static str> {
    fn get_token_precedence(tok: &Token) -> Option<usize> {
        match *tok {
            Token::Other(op) => BINOPS.iter().position(|&test_op| test_op == op),
            _ => None
        }
    }

    loop {
        let tok_prec = match iter.peek().and_then(get_token_precedence) {
            Some(tok_prec) if tok_prec >= precedence => tok_prec,
            _ => return Ok(lhs)
        };

        let binop = if let Some(Token::Other(op)) = iter.next() {
            op
        } else {
            return Err("Expected binary operation");
        };

        let next = parse_primary(iter)?;

        let rhs = if let Some(next_prec) = iter.peek().and_then(get_token_precedence) {
            if tok_prec < next_prec {
                parse_binop_rhs(iter, tok_prec + 1, next)?
            } else {
                next
            }
        } else {
            next
        };

        lhs = Expr::BinaryOp(binop, Box::new(lhs), Box::new(rhs));
    }
}

/// prototype
///   ::= id '(' id* ')'
pub fn parse_prototype<I: Iterator<Item=Token>>(iter: &mut Peekable<I>) -> Result<Prototype, &'static str> {
    if let Some(Token::Identifier(name)) = iter.next() {
       if iter.next() != Some(Token::Other('(')) {
           return Err("Expected '(' in prototype");
       }

       let mut arg_names = Vec::new();
       for t in iter {
           match t {
               Token::Identifier(arg_name) => arg_names.push(arg_name),
               Token::Other(')') => break,
               _ => return Err("Expected ')' in prototype")
           }
       }

       Ok(Prototype {
           name: name,
           args: arg_names
       })
    } else {
        Err("Expected identifier in prototype")
    }
}

/// definition ::= 'def' prototype expression
pub fn parse_definition<I: Iterator<Item=Token>>(iter: &mut Peekable<I>) -> Result<Function, &'static str> {
    if let Some(Token::Def) = iter.next() {
        Ok(Function {
            proto: parse_prototype(iter)?,
            body: parse_expr(iter)?
        })
    } else {
        Err("Expected 'def' in definition")
    }
}

/// external ::= 'extern' prototype
pub fn parse_extern<I: Iterator<Item=Token>>(iter: &mut Peekable<I>) -> Result<Prototype, &'static str> {
    if let Some(Token::Extern) = iter.next() {
        parse_prototype(iter)
    } else {
        Err("Expected 'extern' in extern declaration")
    }
}

/// toplevelexpr ::= expression
pub fn parse_top_level_expr<I: Iterator<Item=Token>>(iter: &mut Peekable<I>) -> Result<Function, &'static str> {
    Ok(Function {
        proto: Prototype { name: String::new(), args: Vec::new() },
        body: parse_expr(iter)?
    })
}
