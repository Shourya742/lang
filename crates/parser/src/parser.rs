pub(crate) mod marker;

use syntax::SyntaxKind;

use crate::{
    event::Event,
    grammar::{self, expr},
    parser::marker::Marker,
    source::Source,
};

pub struct Parser<'t, 'input> {
    source: Source<'t, 'input>,
    events: Vec<Event>,
}

impl<'t, 'input> Parser<'t, 'input> {
    pub fn new(source: Source<'t, 'input>) -> Self {
        Self {
            source,
            events: Vec::new(),
        }
    }

    pub fn at(&mut self, kind: SyntaxKind) -> bool {
        self.peek() == Some(kind)
    }

    pub fn parse(mut self) -> Vec<Event> {
        grammar::expr(&mut self);
        self.events
    }

    pub fn peek(&mut self) -> Option<SyntaxKind> {
        self.source.peek_kind()
    }

    pub fn bump(&mut self) {
        self.source.next_token().unwrap();

        self.events.push(Event::AddToken);
    }

    pub fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::Placeholder);
        Marker::new(pos)
    }
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use crate::check;

    #[test]
    fn parse_nothing() {
        check("", expect![[r#"Root@0..0"#]]);
    }
}
