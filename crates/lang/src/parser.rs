use std::iter::Peekable;
mod event;
mod expr;
mod sink;
mod source;
#[cfg(test)]
use expect_test::Expect;
use logos::Logos;
use rowan::{Checkpoint, GreenNode, GreenNodeBuilder, Language, SyntaxNode};

use crate::{
    lexer::{Lexeme, Lexer, SyntaxKind},
    parser::{event::Event, sink::Sink, source::Source},
    syntax::LangLanguage,
};

pub struct Parser<'l, 'input> {
    source: Source<'l, 'input>,
    events: Vec<Event>,
}

impl<'l, 'input> Parser<'l, 'input> {
    pub fn new(lexemes: &'l [Lexeme<'input>]) -> Self {
        Self {
            source: Source::new(lexemes),
            events: Vec::new(),
        }
    }

    pub fn parse(mut self) -> Vec<Event> {
        self.start_node(SyntaxKind::Root);
        expr::expr(&mut self);
        self.finish_node();
        self.events
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.events.push(Event::StartNode { kind });
    }

    fn finish_node(&mut self) {
        self.events.push(Event::FinishNode);
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        self.source.peek_kind()
    }

    fn bump(&mut self) {
        let Lexeme { kind, text } = self.source.next_lexeme().unwrap();

        self.events.push(Event::AddToken {
            kind: *kind,
            text: text.into(),
        });
    }

    fn start_node_at(&mut self, checkpoint: usize, kind: SyntaxKind) {
        self.events.push(Event::StartNodeAt { kind, checkpoint });
    }

    fn checkpoint(&self) -> usize {
        self.events.len()
    }
}

pub fn parse(input: &str) -> Parse {
    let lexemes: Vec<_> = Lexer::new(input).collect();
    let parser = Parser::new(&lexemes);
    let events = parser.parse();
    let sink = Sink::new(&lexemes, events);

    Parse {
        green_node: sink.finish(),
    }
}

pub struct Parse {
    green_node: GreenNode,
}

impl Parse {
    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::<LangLanguage>::new_root(self.green_node.clone());
        let formatted = format!("{:#?}", syntax_node);

        // We cut off the last byte because formatting the SyntaxNode adds on a newline at the end.
        formatted[0..formatted.len() - 1].to_string()
    }
}

#[cfg(test)]
fn check(input: &str, expected_tree: Expect) {
    let parse = parse(input);
    expected_tree.assert_debug_eq(&parse.debug_tree());
}

#[cfg(test)]
mod tests {
    use expect_test::{Expect, expect};

    use crate::parser::check;

    #[test]
    fn parse_nothing() {
        check("", expect![[r#"Root@0..0"#]]);
    }
}
