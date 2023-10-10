use crate::ast::expression::Expression;
use crate::ast::NodeInterface;
use crate::error::Error;
use crate::token::Token;
use std::any::Any;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct StringLiteral {
    pub token: Token,
    pub value: String,
}

impl StringLiteral {
    pub fn new(token: Token, value: String) -> Self {
        Self { token, value }
    }
}

impl Display for StringLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token.literal())
    }
}

impl NodeInterface for StringLiteral {
    fn token_literal(&self) -> String {
        self.token.literal().into()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl TryFrom<Expression> for StringLiteral {
    type Error = anyhow::Error;

    fn try_from(value: Expression) -> Result<Self, Self::Error> {
        match value {
            Expression::StringLiteral(string_exp) => Ok(string_exp),
            unknow => Err(Error::UnknownExpression(unknow.to_string()).into()),
        }
    }
}
