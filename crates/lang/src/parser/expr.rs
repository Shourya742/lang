use crate::{lexer::SyntaxKind, parser::Parser};

pub(super) fn expr(p: &mut Parser) {
    expr_binding_power(p, 0);
}

pub fn expr_binding_power(p: &mut Parser, minimum_binding_power: u8) {
    let checkpoint = p.checkpoint();
    match p.peek() {
        Some(SyntaxKind::Number) | Some(SyntaxKind::Ident) => p.bump(),
        Some(SyntaxKind::Minus) => {
            let op = PrefixOp::Neg;
            let ((), righ_binding_power) = op.binding_power();
            p.bump();

            p.start_node_at(checkpoint, SyntaxKind::PrefixExpr);
            expr_binding_power(p, righ_binding_power);
            p.finish_node();
        }
        Some(SyntaxKind::LParen) => {
            p.bump();
            expr_binding_power(p, 0);
            assert_eq!(p.peek(), Some(SyntaxKind::RParen));
            p.bump();
        }
        _ => {}
    }

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

        p.start_node_at(checkpoint, SyntaxKind::BinaryExpr);
        expr_binding_power(p, right_binding_power);
        p.finish_node();
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
