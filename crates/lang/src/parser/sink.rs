use std::mem;

use rowan::{Checkpoint, GreenNode, GreenNodeBuilder, Language, SmolStr};

use crate::{
    lexer::{SyntaxKind, Token},
    parser::Event,
    syntax::LangLanguage,
};

pub(super) struct Sink<'t, 'input> {
    builder: GreenNodeBuilder<'static>,
    events: Vec<Event>,
    tokens: &'t [Token<'input>],
    cursor: usize,
}

impl<'t, 'input> Sink<'t, 'input> {
    pub(super) fn new(tokens: &'t [Token<'input>], events: Vec<Event>) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            tokens,
            events,
            cursor: 0,
        }
    }

    pub(super) fn finish(mut self) -> GreenNode {
        for idx in 0..self.events.len() {
            match mem::replace(&mut self.events[idx], Event::Placeholder) {
                Event::StartNode {
                    kind,
                    forward_parent,
                } => {
                    let mut kinds = vec![kind];
                    let mut idx = idx;
                    let mut forward_parent = forward_parent;

                    // Walk through the forward parent of the forward parent, and the forward parent
                    // of that, and of that, etc. until we reach a StartNode event without a forward
                    // parent
                    while let Some(fp) = forward_parent {
                        idx += fp;
                        forward_parent = if let Event::StartNode {
                            kind,
                            forward_parent,
                        } =
                            mem::replace(&mut self.events[idx], Event::Placeholder)
                        {
                            kinds.push(kind);
                            forward_parent
                        } else {
                            unreachable!()
                        };
                    }

                    for kind in kinds.into_iter().rev() {
                        self.builder.start_node(LangLanguage::kind_to_raw(kind));
                    }
                }
                Event::StartNodeAt { .. } => unreachable!(),
                Event::AddToken { kind, text } => self.token(kind, text),
                Event::FinishNode => self.builder.finish_node(),
                Event::Placeholder => {}
            }

            self.eat_trivia();
        }

        self.builder.finish()
    }

    fn token(&mut self, kind: SyntaxKind, text: SmolStr) {
        self.builder.token(LangLanguage::kind_to_raw(kind), text);
        self.cursor += 1;
    }

    fn eat_trivia(&mut self) {
        while let Some(token) = self.tokens.get(self.cursor) {
            if token.kind != SyntaxKind::Whitespace {
                break;
            }

            self.token(token.kind, token.text.into());
        }
    }
}
