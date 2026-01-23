use lexer::TokenKind;

use crate::{
    grammar::expr,
    parser::{Parser, marker::CompletedMarker},
};

pub fn stmt(p: &mut Parser) -> Option<CompletedMarker> {
    if p.at(TokenKind::LetKw) {
        variable_def(p)
    } else {
        expr::expr(p)
    }
}

fn variable_def(p: &mut Parser) -> Option<CompletedMarker> {
    assert!(p.at(TokenKind::LetKw));
    let m = p.start();
    p.bump();

    p.expect(TokenKind::Ident);
    p.expect(TokenKind::Equals);

    expr::expr(p)?;

    Some(m.complete(p, syntax::SyntaxKind::VariableRef))
}
