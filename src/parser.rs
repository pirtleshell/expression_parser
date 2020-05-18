use crate::tokenizer::{Token, Tokenizer};
use crate::tree::{Node, Leaf, BinaryNode, UnaryNode, Evaluable};

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(expression: &str) -> Parser {
        let mut tokenizer = Tokenizer::new(expression.chars());
        tokenizer.next_token();
        Parser { tokenizer }
    }

    pub fn evaluate(expression: &str) -> f64 {
        let mut parser = Parser::new(expression);
        let tree = parser.parse();
        return tree.eval();
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
            left = BinaryNode::new(left, right, op);
        }

        return left;
    }

    fn parse_literal(&mut self) -> Node {
        if self.tokenizer.current_token != Token::Number {
            panic!("Unexpected token! {:?}", self.tokenizer.current_token);
        }
        let num = self.tokenizer.number;
        self.tokenizer.next_token();
        return Leaf::new(num);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn works() {
        assert_eq!(Parser::evaluate("10 + 5"), 15.0);
        assert_eq!(Parser::evaluate("15 * 2"), 30.0);
        assert_eq!(Parser::evaluate("15 + 20 - 12"), 23.0);
    }
}
