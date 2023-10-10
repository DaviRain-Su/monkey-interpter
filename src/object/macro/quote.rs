use crate::ast::{Node, NodeInterface};
use crate::error::Error;
use crate::object::ObjectType::QueueObj;
use crate::object::{Object, ObjectInterface, ObjectType};
use std::any::Any;
use std::fmt::{Display, Formatter};

const QUOTE: &str = "quote";

#[derive(Debug, Clone, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct Quote {
    node: Box<Node>,
}

impl Quote {
    pub fn new(node: Node) -> Self {
        Self {
            node: Box::new(node),
        }
    }

    pub fn node(&self) -> &Node {
        &self.node
    }
}

impl Display for Quote {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "QUOTE({})", self.node)
    }
}

impl NodeInterface for Quote {
    fn token_literal(&self) -> String {
        QUOTE.to_string()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ObjectInterface for Quote {
    fn r#type(&self) -> ObjectType {
        QueueObj
    }

    fn inspect(&self) -> String {
        format!("{self}")
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TryFrom<Object> for Quote {
    type Error = anyhow::Error;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        match value {
            Object::Quote(value) => Ok(value),
            _ => Err(Error::UnknownObjectType.into()),
        }
    }
}

#[test]
fn test_create_quote() {
    use crate::ast::expression::Expression;
    use crate::ast::Identifier;

    let identitier = Identifier::default();

    let quote = Quote {
        node: Box::new(Node::Expression(Expression::IdentifierExpression(
            identitier,
        ))),
    };

    println!("Quote = {:?}", quote);
}
