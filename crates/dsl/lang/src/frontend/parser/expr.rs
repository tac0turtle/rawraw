use crate::frontend::ast;
use crate::frontend::lexer::Token;
use crate::frontend::lexer::Token::*;
use crate::frontend::parser::state::{MarkClosed, Parser};

pub fn expr(p: &mut Parser) {
    expr_rec(p, Eof); // Eof is just a dummy token until we have an actual left-hand side token
}

fn expr_rec(p: &mut Parser, left: Token) {
    fn right_binds_tighter(left: &Token, right: &Token) -> bool {
        fn tightness(kind: &Token) -> Option<usize> {
            [
                // Precedence table:
                &[Eq],
                // [Plus, Minus].as_slice(),
                // &[Star, Slash],
                &[Dot],
            ]
                .iter()
                .position(|level| level.contains(kind))
        }
        let Some(right_tightness) = tightness(right) else {
            return false;
        };
        let Some(left_tightness) = tightness(left) else {
            assert_eq!(left, &Eof);
            return true;
        };
        right_tightness > left_tightness
    }

    // ExprCall
    let mut lhs = expr_delimited(p);
    while p.at(LParen) {
        let m = p.open_before(lhs);
        arg_list(p);
        lhs = p.close::<ast::ExprCall>(m);
    }
    loop {
        let right = p.nth(0);
        if right_binds_tighter(&left, &right) {
            let m = p.open_before(lhs);
            p.advance();
            expr_rec(p, right);
            lhs = p.close::<ast::ExprBinary>(m);
        } else {
            break;
        }
    }
}

fn expr_delimited(p: &mut Parser) -> MarkClosed {
    let m = p.open();
    match p.cur() {
        Ident => {
            p.expect(Ident);
            let mut lhs = p.close::<ast::NameExpr>(m);
            lhs
        }
        LParen => {
            p.expect(LParen);
            expr(p);
            p.expect(RParen);
            p.close::<ast::ExprParen>(m)
        }
        _ => {
            if !p.eof() {
                p.advance();
            }
            p.close::<ast::ErrorNode>(m)
        }
    }
}

// ArgList = '(' Arg* ')'
fn arg_list(p: &mut Parser) {
    assert!(p.at(LParen));
    let m = p.open();
    p.expect(LParen);
    while !p.at(RParen) && !p.eof() {
        arg(p);
    }
    p.expect(RParen);
    p.close::<ast::ArgList>(m);
}

// Arg = Expr ','?
fn arg(p: &mut Parser) {
    let m = p.open();
    expr(p);
    if !p.at(RParen) {
        p.expect(Comma);
    }
    p.close::<ast::Arg>(m);
}

