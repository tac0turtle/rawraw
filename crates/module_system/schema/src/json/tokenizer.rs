use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, PartialEq)]
pub enum Token {
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Colon,
    Comma,
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

pub struct Tokenizer {
    input: Vec<char>,
    position: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Tokenizer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Result<Token, String>> {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return None;
        }

        let c = self.input[self.position];
        self.position += 1;

        let token = match c {
            '{' => Ok(Token::LBrace),
            '}' => Ok(Token::RBrace),
            '[' => Ok(Token::LBracket),
            ']' => Ok(Token::RBracket),
            ':' => Ok(Token::Colon),
            ',' => Ok(Token::Comma),
            '"' => self.read_string(),
            '0'..='9' | '-' => {
                self.position -= 1;
                self.read_number()
            }
            't' => {
                self.position -= 1;
                self.read_true()
            }
            'f' => {
                self.position -= 1;
                self.read_false()
            }
            'n' => {
                self.position -= 1;
                self.read_null()
            }
            _ => Err(format!("Unexpected character: {}", c)),
        };

        Some(token)
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len()
            && self.input[self.position].is_whitespace() {
            self.position += 1;
        }
    }

    fn read_string(&mut self) -> Result<Token, String> {
        let mut value = String::new();

        while self.position < self.input.len() {
            let c = self.input[self.position];
            self.position += 1;

            match c {
                '"' => return Ok(Token::String(value)),
                '\\' => {
                    if self.position >= self.input.len() {
                        return Err("Unexpected end of string".to_string());
                    }
                    let escaped = self.input[self.position];
                    self.position += 1;
                    match escaped {
                        '"' | '\\' | '/' => value.push(escaped),
                        'b' => value.push('\u{0008}'),
                        'f' => value.push('\u{000C}'),
                        'n' => value.push('\n'),
                        'r' => value.push('\r'),
                        't' => value.push('\t'),
                        'u' => {
                            // Handle unicode escape sequences
                            if self.position + 4 > self.input.len() {
                                return Err("Invalid unicode escape sequence".to_string());
                            }
                            let hex: String = self.input[self.position..self.position + 4]
                                .iter().collect();
                            self.position += 4;

                            let code = u32::from_str_radix(&hex, 16)
                                .map_err(|_| "Invalid unicode escape sequence".to_string())?;

                            if let Some(c) = std::char::from_u32(code) {
                                value.push(c);
                            } else {
                                return Err("Invalid unicode codepoint".to_string());
                            }
                        }
                        _ => return Err(format!("Invalid escape sequence: \\{}", escaped)),
                    }
                }
                _ => value.push(c),
            }
        }

        Err("Unterminated string".to_string())
    }

    fn read_number(&mut self) -> Result<Token, String> {
        let start = self.position;

        // Handle negative sign
        if self.input[self.position] == '-' {
            self.position += 1;
        }

        // Read integer part
        while self.position < self.input.len()
            && self.input[self.position].is_ascii_digit() {
            self.position += 1;
        }

        // Handle decimal point and fractional part
        if self.position < self.input.len() && self.input[self.position] == '.' {
            self.position += 1;
            while self.position < self.input.len()
                && self.input[self.position].is_ascii_digit() {
                self.position += 1;
            }
        }

        // Handle exponent
        if self.position < self.input.len()
            && (self.input[self.position] == 'e' || self.input[self.position] == 'E') {
            self.position += 1;

            if self.position < self.input.len()
                && (self.input[self.position] == '+' || self.input[self.position] == '-') {
                self.position += 1;
            }

            while self.position < self.input.len()
                && self.input[self.position].is_ascii_digit() {
                self.position += 1;
            }
        }

        let number_str: String = self.input[start..self.position].iter().collect();
        match number_str.parse::<f64>() {
            Ok(n) => Ok(Token::Number(n)),
            Err(_) => Err(format!("Invalid number: {}", number_str)),
        }
    }

    fn read_true(&mut self) -> Result<Token, String> {
        self.read_keyword("true").map(|_| Token::Boolean(true))
    }

    fn read_false(&mut self) -> Result<Token, String> {
        self.read_keyword("false").map(|_| Token::Boolean(false))
    }

    fn read_null(&mut self) -> Result<Token, String> {
        self.read_keyword("null").map(|_| Token::Null)
    }

    fn read_keyword(&mut self, keyword: &str) -> Result<(), String> {
        if self.position + keyword.len() > self.input.len() {
            return Err(format!("Expected keyword: {}", keyword));
        }

        let slice: String = self.input[self.position..self.position + keyword.len()]
            .iter().collect();

        if slice == keyword {
            self.position += keyword.len();
            Ok(())
        } else {
            Err(format!("Expected keyword: {}", keyword))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let mut tokenizer = Tokenizer::new(r#"{"name": "John", "age": 30}"#);

        assert_eq!(tokenizer.next_token(), Some(Ok(Token::LBrace)));
        assert_eq!(tokenizer.next_token(), Some(Ok(Token::String("name".to_string()))));
        assert_eq!(tokenizer.next_token(), Some(Ok(Token::Colon)));
        assert_eq!(tokenizer.next_token(), Some(Ok(Token::String("John".to_string()))));
        assert_eq!(tokenizer.next_token(), Some(Ok(Token::Comma)));
        assert_eq!(tokenizer.next_token(), Some(Ok(Token::String("age".to_string()))));
        assert_eq!(tokenizer.next_token(), Some(Ok(Token::Colon)));
        assert_eq!(tokenizer.next_token(), Some(Ok(Token::Number(30.0))));
        assert_eq!(tokenizer.next_token(), Some(Ok(Token::RBrace)));
        assert_eq!(tokenizer.next_token(), None);
    }

    #[test]
    fn test_complex_values() {
        let input = r#"{
            "string": "Hello, \"World\"!",
            "number": -12.34e+56,
            "array": [true, false, null]
        }"#;

        let mut tokenizer = Tokenizer::new(input);
        let expected_tokens = vec![
            Token::LBrace,
            Token::String("string".to_string()),
            Token::Colon,
            Token::String("Hello, \"World\"!".to_string()),
            Token::Comma,
            Token::String("number".to_string()),
            Token::Colon,
            Token::Number(-12.34e+56),
            Token::Comma,
            Token::String("array".to_string()),
            Token::Colon,
            Token::LBracket,
            Token::Boolean(true),
            Token::Comma,
            Token::Boolean(false),
            Token::Comma,
            Token::Null,
            Token::RBracket,
            Token::RBrace,
        ];

        for expected in expected_tokens {
            match tokenizer.next_token() {
                Some(Ok(token)) => assert_eq!(token, expected),
                other => panic!("Expected token {:?}, got {:?}", expected, other),
            }
        }

        // Verify we've consumed all tokens
        assert_eq!(tokenizer.next_token(), None);
    }
}