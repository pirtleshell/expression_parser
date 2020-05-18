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
        let tree = parser.parse_addsub();
        return tree.eval();
    }

    pub fn parse_addsub(&mut self) -> Node {
        let mut left = self.parse_multdiv();

        while self.tokenizer.current_token != Token::EOF {
            let op: fn(f64, f64) -> f64 = match self.tokenizer.current_token {
                Token::Add      => |x, y| x + y,
                Token::Subtract => |x, y| x - y,
                _ => return left
            };

            self.tokenizer.next_token();
            let right = self.parse_multdiv();
            left = BinaryNode::new(left, right, op);
        }

        return left;
    }

    fn parse_multdiv(&mut self) -> Node {
        let left = self.parse_unary();

        let op: fn(f64, f64) -> f64 = match self.tokenizer.current_token {
            Token::Multiply => |x, y| x * y,
            Token::Divide   => |x, y| x / y,
            _ => return left
        };

        self.tokenizer.next_token();
        let right = self.parse_unary();
        return BinaryNode::new(left, right, op);
    }

    fn parse_unary(&mut self) -> Node {
        if self.tokenizer.current_token == Token::Negate {
            self.tokenizer.next_token();
            let child = self.parse_unary();
            return UnaryNode::new(child, |x| -1.0 * x);
        }

        return self.parse_literal();
    }

    fn parse_literal(&mut self) -> Node {
        if self.tokenizer.current_token == Token::ParenOpen {
            self.tokenizer.next_token();

            let internal_expression: Node = self.parse_addsub();
            if self.tokenizer.current_token != Token::ParenClose {
                panic!("No closing parenthesis found.");
            }
            self.tokenizer.next_token();
            return internal_expression;
        }

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
        assert_eq!(Parser::evaluate("30.5 + 62"), 92.5);
    }

    #[test]
    fn handles_negatives() {
        assert_eq!(Parser::evaluate("-42"), -42.0);
        assert_eq!(Parser::evaluate("---42"), -42.0);
        assert_eq!(Parser::evaluate("10 + -100"), -90.0);
    }

    #[test]
    fn follows_order_of_operations() {
        assert_eq!(Parser::evaluate("2 + 20 * 2"), 42.0);
        assert_eq!(Parser::evaluate("(2 + 20) * 2"), 44.0);
        assert_eq!(Parser::evaluate("-(10+2)*3"), -36.0);
        assert_eq!(Parser::evaluate("-(10*2) / 5"), -4.0);
    }
}
