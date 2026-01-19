use std::iter::Peekable;
mod expr;
#[cfg(test)]
use expect_test::Expect;
use logos::Logos;
use rowan::{Checkpoint, GreenNode, GreenNodeBuilder, Language, SyntaxNode};

use crate::{
    lexer::{Lexer, SyntaxKind},
    syntax::LangLanguage,
};

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    builder: GreenNodeBuilder<'static>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input).peekable(),
            builder: GreenNodeBuilder::new(),
        }
    }

    pub fn parse(mut self) -> Parse {
        self.start_node(SyntaxKind::Root);
        expr::expr(&mut self);
        self.finish_node();
        Parse {
            green_node: self.builder.finish(),
        }
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(LangLanguage::kind_to_raw(kind));
    }

    fn finish_node(&mut self) {
        self.builder.finish_node();
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        self.lexer.peek().map(|(kind, _)| *kind)
    }

    fn bump(&mut self) {
        let (kind, text) = self.lexer.next().unwrap();

        self.builder
            .token(LangLanguage::kind_to_raw(kind), text.into());
    }

    fn start_node_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind) {
        self.builder
            .start_node_at(checkpoint, LangLanguage::kind_to_raw(kind));
    }

    fn checkpoint(&self) -> Checkpoint {
        self.builder.checkpoint()
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
    let parse = Parser::new(input).parse();
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
