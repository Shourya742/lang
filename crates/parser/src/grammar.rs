mod expr;
pub(crate) use expr::expr;

use crate::parser::Parser;
use crate::parser::marker::CompletedMarker;
use syntax::SyntaxKind;

pub(crate) fn root(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    expr::expr(p);

    m.complete(p, SyntaxKind::Root)
}
