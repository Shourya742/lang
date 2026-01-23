use syntax::{SyntaxElement, SyntaxKind, SyntaxNode, SyntaxToken};





#[derive(Debug)]
pub struct VariableDef(SyntaxNode);


impl VariableDef {
    pub fn name(&self) -> Option<SyntaxToken> {
        self.0.children_with_tokens().filter_map(SyntaxElement::into_token)
        .find(|token| token.kind() == SyntaxKind::Ident)
    }

    pub fn value(&self) -> Option<Expr> {
        self.0.children().find_map(Expr::cast)
    }
}