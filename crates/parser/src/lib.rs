mod event;
mod grammar;
mod parser;
mod sink;
mod source;
#[cfg(test)]
use expect_test::Expect;
use rowan::{GreenNode, SyntaxNode};

use lexer::Lexer;
use syntax::LangLanguage;

use crate::{event::Event, parser::Parser, sink::Sink, source::Source};

pub fn parse(input: &str) -> Parse {
    let tokens: Vec<_> = Lexer::new(input).collect();
    let source = Source::new(&tokens);
    let parser = Parser::new(source);
    let events = parser.parse();
    let sink = Sink::new(&tokens, events);

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
pub fn check(input: &str, expected_tree: Expect) {
    let parse = parse(input);
    expected_tree.assert_debug_eq(&parse.debug_tree());
}
