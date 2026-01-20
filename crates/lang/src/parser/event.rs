use rowan::SmolStr;

use crate::lexer::SyntaxKind;

#[derive(Debug, Clone)]
pub(super) enum Event {
    StartNode { kind: SyntaxKind },
    StartNodeAt { kind: SyntaxKind, checkpoint: usize },
    AddToken { kind: SyntaxKind, text: SmolStr },
    FinishNode,
}
