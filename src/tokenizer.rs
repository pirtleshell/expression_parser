use std::{
    str::Chars,
    f64::NAN,
    vec::Vec,
};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    None,
    EOF,
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Number,
}

pub struct Tokenizer<'a> {
    chars: Chars<'a>,
    pub current_char: char,
    pub current_token: Token,
    pub number: f64,
}

impl<'a> Tokenizer<'a> {
    /** create a tokenizer from a Read */
    pub fn new(mut chars: Chars<'a>) -> Tokenizer {
        Tokenizer {
            current_char: chars.next().unwrap(),
            chars,
            current_token: Token::None,
            number: NAN,
        }
    }

    /** move to next character */
    fn next_char(&mut self) {
        self.current_char = self.chars.next().unwrap_or('\0');
    }

    /** parse the next token */
    pub fn next_token(&mut self) {
        // skip whitespace
        while self.current_char.is_whitespace() {
            self.next_char();
        }

        // handle negatives
        // this distinguishes from subtract by knowing that numbers (& therefore negatives)
        // must always be at the beginning or follow an operator, both cases having self.number = NAN
        if self.number.is_nan() && self.current_char == '-' {
            self.current_token = Token::Negate;
            self.next_char();
            return;
        }

        // handle numbers
        if self.current_char.is_digit(10) {
            let mut digits: Vec<char> = Vec::new();
            while self.current_char.is_digit(10) || self.current_char == '.' {
                digits.push(self.current_char);
                self.next_char();
            }

            let num_as_str: String = digits.iter().collect::<String>();
            self.number = num_as_str.parse::<f64>().unwrap();
            self.current_token = Token::Number;
            return;
        }

        // handle all other characters
        self.current_token = match self.current_char {
            '\0' => Token::EOF,
            '+'  => Token::Add,
            '-'  => Token::Subtract,
            '/'  => Token::Divide,
            '*'  => Token::Multiply,
            _ => panic!("Unexpected character: {}", self.current_char),
        };
        self.number = NAN;
        self.next_char();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_tokens() {
        let mut tokenizer = Tokenizer::new("10 + 6 - 23".chars());

        tokenizer.next_token();
        assert_eq!(tokenizer.current_token, Token::Number);
        assert_eq!(tokenizer.number, 10.0);

        tokenizer.next_token();
        assert_eq!(tokenizer.current_token, Token::Add);

        tokenizer.next_token();
        assert_eq!(tokenizer.current_token, Token::Number);
        assert_eq!(tokenizer.number, 6.0);

        tokenizer.next_token();
        assert_eq!(tokenizer.current_token, Token::Subtract);

        tokenizer.next_token();
        assert_eq!(tokenizer.current_token, Token::Number);
        assert_eq!(tokenizer.number, 23.0);

        tokenizer.next_token();
        assert_eq!(tokenizer.current_token, Token::EOF);
    }

    #[test]
    fn handles_decimals() {
        let mut tokenizer = Tokenizer::new("42.91".chars());
        tokenizer.next_token();
        assert_eq!(tokenizer.current_token, Token::Number);
        assert_eq!(tokenizer.number, 42.91);
    }
}
