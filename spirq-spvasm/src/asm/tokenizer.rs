use std::iter::Peekable;
use std::str::Chars;
use std::str::FromStr;

use anyhow::{anyhow, bail, Result};

#[derive(Debug, Clone)]
pub enum Lit {
    Int(i64),
    // Base numeric and the exponent bias. The effect of the bias depends on the
    // actual floating-point type it casts to.
    Float(f64),
    String(String),
}

#[derive(Debug)]
pub enum Token {
    Comment(String),
    Literal(Lit),
    Ident(String),
    IdRef(String),
    Eq,
    NewLine,
}

pub struct Tokenizer<'a> {
    chars: Box<Peekable<Chars<'a>>>,
}
impl<'a> Tokenizer<'a> {
    pub fn new(code: &'a str) -> Self {
        Tokenizer {
            chars: Box::new(code.chars().peekable()),
        }
    }

    pub fn tokenize_comment(&mut self) -> Result<Token> {
        self.chars.next(); // Consume the initial ';'.

        let mut comment = String::new();
        while let Some(c) = self.chars.peek() {
            if *c == '\n' {
                break;
            }
            comment.push(*c);
            self.chars.next();
        }
        return Ok(Token::Comment(comment));
    }

    pub fn tokenize_idref(&mut self) -> Result<Token> {
        self.chars.next(); // Consume the '%'.

        let mut buf = String::new();
        while let Some(c) = self.chars.peek() {
            if c.is_ascii_alphanumeric() || c == &'_' {
                buf.push(*c);
                self.chars.next();
            } else {
                break;
            }
        }
        return Ok(Token::IdRef(buf));
    }

    pub fn tokenize_numeric_literal_decimal(&mut self) -> Result<Lit> {
        let mut buf = String::new();
        buf.push('0'); // So that we can tolerate numerics failed from hexadecimal parsing.
        while let Some(c) = self.chars.peek() {
            if c.is_ascii_digit() {
                buf.push(*c);
                self.chars.next();
            } else {
                break;
            }
        }
        let lit = match self.chars.peek() {
            Some('.') => {
                // Float.
                buf.push('.');
                self.chars.next();
                while let Some(c) = self.chars.peek() {
                    if c.is_ascii_digit() {
                        buf.push(*c);
                        self.chars.next();
                    } else {
                        break;
                    }
                }
                if let Some(c) = self.chars.peek() {
                    if c == &'e' || c == &'E' {
                        // Float with exponent.
                        buf.push(*c);
                        self.chars.next();
                        while let Some(c) = self.chars.peek() {
                            if c.is_ascii_digit() || c == &'+' || c == &'-' {
                                buf.push(*c);
                                self.chars.next();
                            } else {
                                break;
                            }
                        }
                    }
                }
                Lit::Float(f64::from_str(buf.as_str())?)
            },
            _ => {
                // Integer.
                Lit::Int(i64::from_str(buf.as_str())?)
            },
        };
        Ok(lit)
    }

    pub fn tokenize_numeric_literal_hexadecimal(&mut self) -> Result<Lit> {
        let mut int_buf = String::new();
        while let Some(c) = self.chars.peek() {
            if c.is_ascii_hexdigit() {
                int_buf.push(*c);
                self.chars.next();
            } else {
                break;
            }
        }

        // Fraction without the dot.
        let mut fraction_buf = String::new();
        if (int_buf == "0" || int_buf == "1") && (self.chars.peek() == Some(&'.')) {
            self.chars.next(); // Consume the '.'.

            while let Some(c) = self.chars.peek() {
                if c.is_ascii_hexdigit() {
                    fraction_buf.push(*c);
                    self.chars.next();
                } else {
                    break;
                }
            }
        }

        let mut exponent_buf = String::new();
        println!("peek: {:?}", self.chars.peek());
        if let Some('p') = self.chars.peek().map(|x| x.to_ascii_lowercase()) {
            self.chars.next(); // Consume the 'p' or 'P'.

            // Float with exponent.
            match self.chars.peek() {
                Some('+') => {
                    self.chars.next();
                },
                Some('-') => {
                    exponent_buf.push('-');
                    self.chars.next();
                },
                _ => {},
            }
            while let Some(c) = self.chars.peek() {
                if c.is_ascii_digit() {
                    exponent_buf.push(*c);
                    self.chars.next();
                } else {
                    break;
                }
            }
        }

        let is_hexadecimal_float = fraction_buf.len() > 0 || exponent_buf.len() > 0;
        if !fraction_buf.is_empty() {
            if exponent_buf.is_empty() {
                bail!("hexadecimal value with fraction part but the exponent bias is missing");
            }
        }

        let lit = if is_hexadecimal_float {
            // Assemble hexadecimal floating point numbers.
            let int = i64::from_str_radix(&int_buf, 16)?;
            let fraction = match fraction_buf.is_empty() {
                true => 0,
                false => i64::from_str_radix(&fraction_buf, 16)?,
            };
            let exponent = i32::from_str(&exponent_buf)?;

            fraction_buf = fraction_buf.trim_start_matches('0').to_string();
            let f = ((int as f64) + (fraction as f64) / 16f64.powi(fraction_buf.len() as i32)) * 2f64.powi(exponent);
            Lit::Float(f)
        } else {
            let i = i64::from_str_radix(&int_buf, 16)?;
            Lit::Int(i)
        };

        Ok(lit)
    }

    /// Tokenize a SPIR-V assembly numeric literal that can be decimal, hexadecimal,
    /// decimal and hexadecimal numbers.
    pub fn tokenize_numeric_literal(&mut self) -> Result<Token> {
        let mut c = *self.chars.peek()
            .ok_or_else(|| anyhow!("unexpected end of input"))?;

        let mantissa_sign = match c {
            '-' => {
                self.chars.next();
                c = *self.chars.peek()
                    .ok_or_else(|| anyhow!("unexpected end of input"))?;
                -1
            },
            _ => 1,
        };

        let lit = if c == '0' {
            self.chars.next(); // Consume the initial '0'.
            match self.chars.peek() {
                Some('x') | Some('X') => {
                    // Hexadecimal.
                    self.chars.next(); // Consume the 'x' or 'X'.
                    self.tokenize_numeric_literal_hexadecimal()?
                },
                _ => {
                    // Decimal.
                    self.tokenize_numeric_literal_decimal()?
                },
            }
        } else {
            // Decimal.
            self.tokenize_numeric_literal_decimal()?
        };

        // Weird special case of image `Dim`, in which there are `1D``, `2D``
        // and `3D` as identifiers. This it rather annoying.
        if let Lit::Int(i) = lit {
            if self.chars.peek() == Some(&'D') {
                self.chars.next(); // Consume the 'D'.
                return Ok(Token::Ident(format!("{}D", i)));
            }
        }

        let lit = match lit {
            Lit::Int(i) => Lit::Int(i * mantissa_sign),
            Lit::Float(f) => Lit::Float(f * mantissa_sign as f64),
            Lit::String(_) => unreachable!(),
        };

        let token = Token::Literal(lit);
        Ok(token)
    }

    pub fn tokenize_string_literal(&mut self) -> Result<Token> {
        self.chars.next(); // Consume the initial '"'.

        let mut string = String::new();
        let mut escape = false;

        while let Some(c) = self.chars.next() {
            if escape {
                // Escaped character.
                escape = false;
                string.push(c)
            } else {
                match c {
                    '\\' => {
                        escape = true;
                        continue;
                    },
                    '"' => break,
                    _ => string.push(c),
                }
            }
        }
        let lit = Lit::String(string);
        let token = Token::Literal(lit);

        return Ok(token);
    }

    pub fn tokenize_ident(&mut self) -> Result<Token> {
        let mut ident = String::new();
        while let Some(c) = self.chars.peek() {
            if c.is_ascii_alphanumeric() || c == &'_' {
                ident.push(*c);
                self.chars.next();
            } else {
                break;
            }
        }
        return Ok(Token::Ident(ident));
    }

    pub fn tokenize(&mut self) -> Result<Option<Token>> {
        if let Some(c) = self.chars.peek() {
            // Ignore LWS.
            if c.is_ascii_whitespace() {
                while let Some(c) = self.chars.peek() {
                    if *c == '\n' {
                        self.chars.next();
                        return Ok(Some(Token::NewLine));
                    } else if c.is_ascii_whitespace() {
                        self.chars.next();
                    } else {
                        break;
                    }
                }
                return self.tokenize();
            }

            // Comments.
            if c == &';' {
                let token = self.tokenize_comment()?;
                return Ok(Some(token));
            }

            // Punctuations.
            if c == &'=' {
                self.chars.next(); // Consume the '='.
                let token = Token::Eq;
                return Ok(Some(token));
            }

            // IdRefs.
            if c == &'%' {
                let token = self.tokenize_idref()?;
                return Ok(Some(token));
            }

            // Literal numerics.
            if c == &'-' || c.is_ascii_digit() {
                let token = self.tokenize_numeric_literal();
                return Ok(Some(token?));
            }

            // Literal string.
            if c == &'"' {
                let token = self.tokenize_string_literal()?;
                return Ok(Some(token));
            }

            // Identifiers.
            if c.is_ascii_alphabetic() || c == &'_' {
                let token = self.tokenize_ident()?;
                return Ok(Some(token));
            }

            bail!("unexpected character: {}", c);

        } else {
            return Ok(None);
        }
    }
}
impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tokenize().transpose()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    pub fn tokenize(code: &str) -> Result<Vec<Token>> {
        let tokenizer = Tokenizer::new(code);
        let tokens = tokenizer.collect::<Result<Vec<_>>>();
        tokens
    }

    #[test]
    fn test_tokenize_nothing() {
        let code = "";
        let tokens = tokenize(code).unwrap();
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_tokenize_integers() {
        let code = "0 1 2 3 4 5 6 7 8 9";
        let tokens = tokenize(code).unwrap();
        assert_eq!(tokens.len(), 10);
        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::Literal(Lit::Int(n)) => assert_eq!(*n, i as i64),
                _ => panic!("unexpected token: {:?}", token),
            }
        }
    }

    #[test]
    fn test_tokenize_floats() {
        let code = "0.0 1.0 2.0 3.0 4.0 5.0 6.0 7.0";
        let tokens = tokenize(code).unwrap();
        assert_eq!(tokens.len(), 8);
        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::Literal(Lit::Float(n)) => assert_eq!(*n, i as f64),
                _ => panic!("unexpected token: {:?}", token),
            }
        }
    }

    #[test]
    fn test_tokenize_floats_with_exponent() {
        let code = "0.0e0 1.0e1 2.0e2 3.0e3 4.0e4 5.0e5 6.0e6 7.0e7";
        let tokens = tokenize(code).unwrap();
        assert_eq!(tokens.len(), 8);
        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::Literal(Lit::Float(n)) => assert_eq!(*n, (i as f64) * 10.0f64.powi(i as i32)),
                _ => panic!("unexpected token: {:?}", token),
            }
        }
    }

    #[test]
    fn test_tokenize_floats_with_exponent_and_sign() {
        let code = "0.0e+0 1.0e-1 2.0e+2 3.0e-3 4.0e+4 5.0e-5 6.0e+6 7.0e-7";
        let tokens = tokenize(code).unwrap();
        assert_eq!(tokens.len(), 8);
        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::Literal(Lit::Float(n)) => assert_eq!(*n, (i as f64) * 10.0f64.powi((i as i32) * (if i % 2 == 0 { 1 } else { -1 }))),
                _ => panic!("unexpected token: {:?}", token),
            }
        }
    }

    #[test]
    fn test_tokenize_hexadecimal_integers() {
        let code = "0x0 0x1 0x2 0x3 0x4 0x5 0x6 0x7";
        let tokens = tokenize(code).unwrap();
        assert_eq!(tokens.len(), 8);
        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::Literal(Lit::Int(n)) => assert_eq!(*n, i as i64),
                _ => panic!("unexpected token: {:?}", token),
            }
        }
    }

    #[test]
    fn test_tokenize_hexadecimal_floats() {
        let code = "0x1.0p0 -0x1.8p-1";
        let tokens = tokenize(code).unwrap();
        assert_eq!(tokens.len(), 2);
        for (mantissa, token) in [1.0, -0.75].iter().zip(tokens.iter()) {
            match token {
                Token::Literal(Lit::Float(n)) => assert_eq!(*n, *mantissa),
                _ => panic!("unexpected token: {:?}", token),
            }
        }
    }

    #[test]
    fn test_tokenize_string_literals() {
        let code = r#""" "a" "ab" "abc" "abcd""#;
        let tokens = tokenize(code).unwrap();
        assert_eq!(tokens.len(), 5);
        let expected = ["", "a", "ab", "abc", "abcd"];
        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::Literal(Lit::String(s)) => assert_eq!(s, expected[i]),
                _ => panic!("unexpected token: {:?}", token),
            }
        }
    }

    #[test]
    fn test_tokenize_string_literals_escape() {
        let code = r#""\"\\\""#;
        let tokens = tokenize(code).unwrap();
        assert_eq!(tokens.len(), 1);
        let expected = r#""\""#;
        for (_, token) in tokens.iter().enumerate() {
            match token {
                Token::Literal(Lit::String(s)) => assert_eq!(s, expected),
                _ => panic!("unexpected token: {:?}", token),
            }
        }
    }

    #[test]
    fn test_tokenize_identifiers() {
        let code = r#"a ab abc abcd abcd1 abcd12 abcd123"#;
        let tokens = tokenize(code).unwrap();
        assert_eq!(tokens.len(), 7);
        let expected = ["a", "ab", "abc", "abcd", "abcd1", "abcd12", "abcd123"];
        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::Ident(s) => assert_eq!(s, expected[i]),
                _ => panic!("unexpected token: {:?}", token),
            }
        }
    }

    #[test]
    fn test_tokenize_comments() {
        let code = r#"; a
; ab
; abc
; abcd
; abcd1
; abcd12
; abcd123"#;
        let tokens = tokenize(code).unwrap();
        assert_eq!(tokens.len(), 13);
        let expected = [" a", " ab", " abc", " abcd", " abcd1", " abcd12", " abcd123"];
        for (i, token) in tokens.iter().enumerate() {
            if i % 2 == 0 {
                match token {
                    Token::Comment(s) => assert_eq!(s, expected[i / 2]),
                    _ => panic!("unexpected token: {:?}", token),
                }
            } else {
                match token {
                    Token::NewLine => {},
                    _ => panic!("unexpected token: {:?}", token),
                }
            }
        }
    }

    #[test]
    fn test_tokenize_idref() {
        let code = r#"%1 %123 %abc %abc123"#;
        let tokens = tokenize(code).unwrap();
        assert_eq!(tokens.len(), 4);
        let expected = ["1", "123", "abc", "abc123"];
        for (i, token) in tokens.iter().enumerate() {
            match token {
                Token::IdRef(s) => assert_eq!(s, expected[i]),
                _ => panic!("unexpected token: {:?}", token),
            }
        }
    }

    #[test]
    fn test_tokenize_eq() {
        let code = r#"="#;
        let tokens = tokenize(code).unwrap();
        assert_eq!(tokens.len(), 1);
        match tokens[0] {
            Token::Eq => {},
            _ => panic!("unexpected token: {:?}", tokens[0]),
        }
    }
}
