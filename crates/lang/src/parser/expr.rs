use crate::{
    lexer::SyntaxKind,
    parser::{Parser, marker::CompletedMarker},
};

pub(super) fn expr(p: &mut Parser) {
    expr_binding_power(p, 0);
}

fn lhs(p: &mut Parser) -> Option<CompletedMarker> {
    let cm = match p.peek() {
        Some(SyntaxKind::Number) => literal(p),
        Some(SyntaxKind::Ident) => variable_ref(p),
        Some(SyntaxKind::Minus) => prefix_expr(p),
        Some(SyntaxKind::LParen) => paren_expr(p),
        _ => return None,
    };

    Some(cm)
}

fn literal(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(SyntaxKind::Number));

    let m = p.start();
    p.bump();
    m.complete(p, SyntaxKind::Literal)
}

fn variable_ref(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(SyntaxKind::Ident));

    let m = p.start();
    p.bump();
    m.complete(p, SyntaxKind::VariableRef)
}

fn prefix_expr(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(SyntaxKind::Minus));

    let m = p.start();

    let op = PrefixOp::Neg;
    let ((), right_binding_power) = op.binding_power();

    // Eat the operator’s token.
    p.bump();

    expr_binding_power(p, right_binding_power);

    m.complete(p, SyntaxKind::PrefixExpr)
}

fn paren_expr(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(SyntaxKind::LParen));

    let m = p.start();

    p.bump();
    expr_binding_power(p, 0);

    assert!(p.at(SyntaxKind::RParen));
    p.bump();

    m.complete(p, SyntaxKind::ParenExpr)
}

pub fn expr_binding_power(p: &mut Parser, minimum_binding_power: u8) {
    let mut lhs = if let Some(lhs) = lhs(p) {
        lhs
    } else {
        return;
    };

    loop {
        let op = match p.peek() {
            Some(SyntaxKind::Plus) => Infix::Add,
            Some(SyntaxKind::Minus) => Infix::Sub,
            Some(SyntaxKind::Star) => Infix::Mul,
            Some(SyntaxKind::Slash) => Infix::Div,
            _ => return,
        };

        let (left_binding_power, right_binding_power) = op.binding_power();

        if left_binding_power < minimum_binding_power {
            return;
        }

        p.bump();

        let m = lhs.precede(p);
        expr_binding_power(p, right_binding_power);
        lhs = m.complete(p, SyntaxKind::BinaryExpr);
    }
}

enum Infix {
    Add,
    Sub,
    Mul,
    Div,
}

enum PrefixOp {
    Neg,
}

impl PrefixOp {
    pub fn binding_power(&self) -> ((), u8) {
        match self {
            Self::Neg => ((), 5),
        }
    }
}

impl Infix {
    fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Add | Self::Sub => (1, 2),
            Self::Mul | Self::Div => (3, 4),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::check;

    use super::*;
    use expect_test::expect;

    #[test]
    fn parse_number() {
        check(
            "123",
            expect![[r#"
Root@0..3
  Number@0..3 "123""#]],
        );
    }

    #[test]
    fn parse_binding_usage() {
        check(
            "counter",
            expect![[r#"
Root@0..7
  Ident@0..7 "counter""#]],
        );
    }
}
