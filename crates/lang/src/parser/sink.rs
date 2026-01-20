use rowan::{Checkpoint, GreenNode, GreenNodeBuilder, Language, SmolStr};

use crate::{
    lexer::{Lexeme, SyntaxKind},
    parser::Event,
    syntax::LangLanguage,
};

pub(super) struct Sink<'l, 'input> {
    builder: GreenNodeBuilder<'static>,
    events: Vec<Event>,
    lexemes: &'l [Lexeme<'input>],
    cursor: usize,
}

impl<'l, 'input> Sink<'l, 'input> {
    pub(super) fn new(lexemes: &'l [Lexeme<'input>], events: Vec<Event>) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            lexemes,
            events,
            cursor: 0,
        }
    }

    pub(super) fn finish(mut self) -> GreenNode {
        let mut reordered_events = self.events.clone();

        for (idx, event) in self.events.iter().enumerate() {
            if let Event::StartNodeAt { kind, checkpoint } = event {
                reordered_events.remove(idx);
                reordered_events.insert(*checkpoint, Event::StartNode { kind: *kind });
            }
        }

        for event in reordered_events {
            match event {
                Event::StartNode { kind } => {
                    self.builder.start_node(LangLanguage::kind_to_raw(kind))
                }
                Event::StartNodeAt { .. } => unreachable!(),
                Event::AddToken { kind, text } => self.token(kind, text),
                Event::FinishNode => self.builder.finish_node(),
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
        while let Some(lexeme) = self.lexemes.get(self.cursor) {
            if lexeme.kind != SyntaxKind::Whitespace {
                break;
            }

            self.token(lexeme.kind, lexeme.text.into());
        }
    }
}
