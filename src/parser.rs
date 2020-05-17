use crate::tokenizer::{Token, Tokenizer};
use crate::tree::Node;

struct ExpressionParser<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> ExpressionParser<'a> {
    pub fn new(expression: &str) -> ExpressionParser {
        let mut tokenizer = Tokenizer::new(expression.chars());
        tokenizer.next_token();
        ExpressionParser { tokenizer }
    }

    pub fn parse(&mut self) -> Node {
        let mut left = self.parse_literal();

        while self.tokenizer.current_token != Token::EOF {
            let op: fn(f64, f64) -> f64 = match self.tokenizer.current_token {
                Token::Add      => |x, y| x + y,
                Token::Subtract => |x, y| x - y,
                Token::Multiply => |x, y| x * y,
                Token::Divide   => |x, y| x / y,
                _ => panic!("Invalid parsing token found: {:?}", self.tokenizer.current_token)
            };

            self.tokenizer.next_token();
            let right = self.parse_literal();
            left = Node::operation(op, left, right);
        }

        return left;
    }

    fn parse_literal(&mut self) -> Node {
        if self.tokenizer.current_token != Token::Number {
            panic!("Unexpected token! {:?}", self.tokenizer.current_token);
        }
        let num = self.tokenizer.number;
        self.tokenizer.next_token();
        return Node::number(num);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn works() {
        let mut parser = ExpressionParser::new("10 + 3 - 5");
        let tree = parser.parse();
        assert_eq!(tree.eval(), 8.0);
    }
}
