use std::iter::Peekable;
use std::rc::Rc;
use std::str::Chars;
use std::str::FromStr;

use anyhow::{anyhow, bail, Error, Result};
use spirq_core::parse::{Instr, SpirvBinary, Instrs, bin::SpirvHeader};
use spirq_core::ty::TypeRegistry;

#[derive(Debug)]
enum Lit {
    Int(i64),
    // Base numeric and the exponent bias. The effect of the bias depends on the
    // actual floating-point type it casts to.
    Float(f64, i32),
    String(String),
}

#[derive(Debug)]
enum Token {
    Comment(String),
    Literal(Lit),
    Ident(String),
    Eq,
}

struct Tokenizer<'a> {
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
                Lit::Float(f64::from_str(buf.as_str())?, 0)
            },
            _ => {
                // Integer.
                Lit::Int(i64::from_str(buf.as_str())?)
            },
        };
        Ok(lit)
    }

    pub fn tokenize_numeric_literal_hexadecimal(&mut self) -> Result<Lit> {
        let mut buf = String::new();
        while let Some(c) = self.chars.peek() {
            if c.is_ascii_hexdigit() {
                buf.push(*c);
                self.chars.next();
            } else {
                break;
            }
        }
        // In form of `0x1p0`.
        let lit = if Some(&'.') == self.chars.peek() && buf == "1" {
            // Float.
            buf.push('.');

            // In form of `0x1.2p0`.
            while let Some(c) = self.chars.peek() {
                if c.is_ascii_digit() {
                    buf.push(*c);
                    self.chars.next();
                } else {
                    break;
                }
            }

            let mantissa = f64::from_str(buf.as_str())?;
            let mut exponent_bias = 0;

            if let Some(c) = self.chars.peek() {
                if c == &'p' || c == &'P' {
                    self.chars.next(); // Consume the 'p' or 'P'.

                    // Float with exponent.
                    let mut exponent_bias_buf = String::new();
                    match self.chars.peek() {
                        Some('+') => {
                            self.chars.next();
                        },
                        Some('-') => {
                            exponent_bias_buf.push('-');
                            self.chars.next();
                        },
                        _ => {},
                    }
                    loop {
                        if let Some(c) = self.chars.peek() {
                            if c.is_ascii_digit() {
                                exponent_bias_buf.push(*c);
                                self.chars.next();
                            } else {
                                break;
                            }
                        }
                    }
                    exponent_bias = i32::from_str(&exponent_bias_buf)?;
                }
            }

            Lit::Float(mantissa, exponent_bias)
        } else {
            // Integer.
            Lit::Int(i64::from_str_radix(buf.as_str(), 16)?)
        };

        Ok(lit)
    }

    /// Tokenize a SPIR-V assembly numeric literal that can be decimal, hexadecimal,
    /// decimal and hexadecimal numbers.
    pub fn tokenize_numeric_literal(&mut self) -> Result<Token> {
        let c = *self.chars.peek()
            .ok_or_else(|| anyhow!("unexpected end of input"))?;

        let mantissa_sign = match c {
            '+' => {
                self.chars.next();
                1
            },
            '-' => {
                self.chars.next();
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
        let lit = match lit {
            Lit::Int(i) => Lit::Int(i * mantissa_sign),
            Lit::Float(f, e) => Lit::Float(f * mantissa_sign as f64, e),
            Lit::String(_) => unreachable!(),
        };

        let token = Token::Literal(lit);
        Ok(token)
    }

    pub fn tokenize_string_literal(&mut self) -> Result<Token> {
        self.chars.next(); // Consume the initial '"'.

        let mut string = String::new();
        let mut escape = false;

        loop {
            let c = self.chars.next()
                .ok_or_else(|| anyhow!("unexpected end of input"))?;
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
        let mut c = self.chars.next()
            .ok_or_else(|| anyhow!("unexpected end of input"))?;
        while c.is_ascii_alphanumeric() || c == '_' {
            ident.push(c);
            c = self.chars.next()
                .ok_or_else(|| anyhow!("unexpected end of input"))?;
        }
        return Ok(Token::Ident(ident));
    }

    pub fn tokenize(&mut self) -> Result<Option<Token>> {
        if let Some(c) = self.chars.peek() {
            // Ignore LWS.
            if c.is_ascii_whitespace() {
                self.chars.next();
                while let Some(c) = self.chars.peek() {
                    if c.is_ascii_whitespace() {
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
                let token = Token::Eq;
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



fn tokenize(code: &str) -> Result<Vec<Token>> {
    let tokenizer = Tokenizer::new(code);
    let tokens = tokenizer.collect::<Result<Vec<_>>>();
    tokens
}


#[cfg(test)]
mod test {
    use super::*;

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
                Token::Literal(Lit::Float(n, _)) => assert_eq!(*n, i as f64),
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
                Token::Literal(Lit::Float(n, _)) => assert_eq!(*n, (i as f64) * 10.0f64.powi(i as i32)),
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
                Token::Literal(Lit::Float(n, _)) => assert_eq!(*n, (i as f64) * 10.0f64.powi((i as i32) * (if i % 2 == 0 { 1 } else { -1 }))),
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
}
