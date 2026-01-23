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

use crate::{
    event::Event,
    parser::{ParseError, Parser},
    sink::Sink,
    source::Source,
};

pub fn parse(input: &str) -> Parse {
    let tokens: Vec<_> = Lexer::new(input).collect();
    let source = Source::new(&tokens);
    let parser = Parser::new(source);
    let events = parser.parse();
    let sink = Sink::new(&tokens, events);

    sink.finish()
}

pub struct Parse {
    green_node: GreenNode,
    errors: Vec<ParseError>,
}

impl Parse {
    pub fn debug_tree(&self) -> String {
        let mut s = String::new();
        let syntax_node = SyntaxNode::<LangLanguage>::new_root(self.green_node.clone());
        let tree = format!("{:#?}", syntax_node);

        // We cut off the last byte because formatting the SyntaxNode adds on a newline at the end.
        s.push_str(&tree[0..tree.len() - 1]);

        for error in &self.errors {
            s.push_str(&format!("\n{}", error));
        }
        s
    }
}

#[cfg(test)]
pub fn check(input: &str, expected_tree: Expect) {
    let parse = parse(input);
    expected_tree.assert_debug_eq(&parse.debug_tree());
}
