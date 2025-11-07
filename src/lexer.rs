use crate::token::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn peek(&self, n: usize) -> Option<char> {
        self.input.get(self.position + n).copied()
    }

    fn advance(&mut self, n: usize) {
        self.position += n;
    }

    fn advance_until(&mut self, pattern: &str) {
        while self.position + pattern.len() <= self.input.len() {
            let slice: String = self.input[self.position..self.position + pattern.len()]
                .iter()
                .collect();
            if slice == pattern {
                self.advance(pattern.len()); // überspringe das Pattern selbst
                break;
            }
            self.advance(1);
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.peek(0) {
            Some(c) if c.is_ascii_alphabetic() => self.read_identifier_or_keyword(),
            Some(c) if c.is_numeric() => self.read_number(),
            Some('+') => {
                if self.peek(1) == Some('+') {
                    self.advance(2);
                    return Token::Increment;
                } else if self.peek(1) == Some('=') {
                    self.advance(2);
                    return Token::PlusEquals;
                } else {
                    self.advance(1);
                    return Token::Plus;
                }
            },
            Some('-') => {
                if self.peek(1) == Some('-') {
                    self.advance(2);
                    return Token::Decrement;
                } else if self.peek(1) == Some('=') {
                    self.advance(2);
                    return Token::MinusEquals;
                } else if self.peek(1) == Some('>') {
                    self.advance(2);
                    return Token::Arrow;
                } else {
                    self.advance(1);
                    return Token::Minus;
                }
            },
            Some('*') => {
                if self.peek(1) == Some('*') {
                    if self.peek(2) == Some('=') {
                        self.advance(3);
                        return Token::ExpEquals;
                    } else {
                        self.advance(2);
                        return Token::Exp;
                    }
                } else if self.peek(1) == Some('=') {
                    self.advance(2);
                    return Token::StarEquals;
                } else {
                    self.advance(1);
                    return Token::Star;
                }
            }
            Some('/') => {
                // Comments
                if self.peek(1) == Some('/') {
                    self.advance(2);
                    self.advance_until("\n");
                    return self.next_token();
                } else if self.peek(1) == Some('*') {
                    self.advance(2);
                    self.advance_until("*/");
                    return self.next_token();
                } else if self.peek(1) == Some('=') {
                    self.advance(2);
                    return Token::SlashEquals;
                } else {
                    self.advance(1);
                    Token::Slash
                }
            },
            Some('=') => {
                if self.peek(1) == Some('=') {
                    self.advance(2);
                    return Token::EqualsEquals;
                } else if self.peek(1) == Some('>') {
                    self.advance(2);
                    return Token::DArrow;
                } else {
                    self.advance(1);
                    return Token::Equals;
                }
            },
            Some('(') => { self.advance(1); return Token::LParen },
            Some(')') => { self.advance(1); return Token::RParen },
            Some('[') => { self.advance(1); return Token::LBracket },
            Some(']') => { self.advance(1); return Token::RBracket },
            Some('{') => { self.advance(1); return Token::LCurly },
            Some('}') => { self.advance(1); return Token::RCurly },
            Some('<') => {
                if self.peek(1) == Some('=') {
                    self.advance(2);
                    return Token::LessEquals;
                } else {
                    self.advance(1);
                    return Token::LAngle
                }
            },
            Some('>') => {
                if self.peek(1) == Some('=') {
                    self.advance(2);
                    return Token::GreaterEquals;
                } else {
                    self.advance(1);
                    return Token::RAngle;
                }
            }
            Some('.') => {
                if self.peek(1) == Some('.') {
                    if self.peek(2) == Some('.') {
                        self.advance(3);
                        return Token::Ellipsis;
                    } else if self.peek(2) == Some('=') {
                        self.advance(3);
                        return Token::IterIncl;
                    } else {
                        self.advance(2);
                        return Token::Iter;
                    }
                } else {
                    self.advance(1); 
                    return Token::Dot;
                }
            },
            Some(',') => { self.advance(1); return Token::Comma },
            Some(':') => { self.advance(1); return Token::Colon },
            Some(';') => { self.advance(1); return Token::Semicolon },
            Some('\'') => self.read_char(),
            Some('"') => self.read_string(),
            Some(c) => { self.advance(1); return Token::Unknown(c) },
            None => Token::EOF,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek(0) {
            if c.is_whitespace() {
                self.advance(1);
            } else {
                break;
            }
        }
    }

    fn strip_underscores(&self, s: &str) -> String {
        s.replace('_', "")
    }

    fn read_identifier_or_keyword(&mut self) -> Token {
        let start = self.position;

        while let Some(c) = self.peek(0) {
            if c.is_ascii_alphanumeric() || c == '_' {
                self.advance(1);
            } else {
                break;
            }
        }

        let raw: String = self.input[start..self.position].iter().collect();

        // Unterstriche prüfen
        if raw.starts_with('_') || raw.ends_with('_') {
            panic!("Ungültige Unterstrich-Position im Identifier: {}", raw);
        }

        let upper: bool = raw.chars().next().map(|c| c.is_uppercase()).unwrap_or(false);
        
        // Hier könnten wir später weitere Regeln hinzufügen (z.B. keine "__" aufeinanderfolgend)
        match raw.as_str() {
            "let" => Token::Let,
            "print" => Token::Print,
            _ => Token::Identifier(raw, upper),
        }
    }

    fn read_number(&mut self) -> Token {
        // Hex-Zahlen
        if self.peek(0) == Some('0') && matches!(self.peek(1), Some('x') | Some('X')) {
            self.advance(2); // skip 0x
            return self.read_hex_float_or_int();
        }

        self.read_decimal_or_scientific()
    }

    // Unterstriche validieren und entfernen
    fn sanitize_number(s: &str, forbid_start: &[char], forbid_before_after: &[char]) -> Option<String> {
        let mut result = String::new();
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();

        for i in 0..len {
            let c = chars[i];
            if c == '_' {
                // darf nicht am Anfang oder Ende stehen
                if i == 0 || i == len - 1 {
                    return None;
                }
                let prev = chars[i - 1];
                let next = chars[i + 1];
                if forbid_before_after.contains(&prev) || forbid_before_after.contains(&next) {
                    return None;
                }
            } else if forbid_start.contains(&c) && i == 0 {
                return None;
            } else {
                result.push(c);
            }
        }
        Some(result)
    }

    fn read_decimal_or_scientific(&mut self) -> Token {
        let start = self.position;
        let mut has_dot = false;

        while let Some(c) = self.peek(0) {
            if c.is_numeric() || c == '_' {
                self.advance(1);
            } else if c == '.' && !has_dot {
                has_dot = true;
                self.advance(1);
            } else if c == 'e' || c == 'E' {
                self.advance(1);
                if let Some(sign) = self.peek(0) {
                    if sign == '+' || sign == '-' {
                        self.advance(1);
                    }
                }
            } else {
                break;
            }
        }

        let raw: String = self.input[start..self.position].iter().collect();
        let s = match Self::sanitize_number(&raw, &[], &['.', 'e', 'E']) {
            Some(s) => s,
            None => panic!("Ungültige Unterstrich-Position in Zahl: {}", raw),
        };

        if has_dot || s.contains('e') || s.contains('E') {
            Token::Float(s.parse::<f64>().unwrap())
        } else {
            Token::Integer(s.parse::<i64>().unwrap())
        }
    }

    fn read_hex_float_or_int(&mut self) -> Token {
        let start = self.position;

        while let Some(c) = self.peek(0) {
            if c.is_digit(16) || c == '.' || c == 'p' || c == 'P' || c == '+' || c == '-' || c == '_' {
                self.advance(1);
            } else {
                break;
            }
        }

        let raw: String = self.input[start..self.position].iter().collect();
        let s = match Self::sanitize_number(&raw, &[], &['.', 'p', 'P']) {
            Some(s) => s,
            None => panic!("Ungültige Unterstrich-Position in Hex-Zahl: {}", raw),
        };

        // Hex-Float mit 'p'
        if let Some(p_pos) = s.find(|c| c == 'p' || c == 'P') {
            let (mantissa_str, exponent_str) = s.split_at(p_pos);
            let exponent_str = &exponent_str[1..];

            let parts: Vec<&str> = mantissa_str.split('.').collect();
            let int_part = parts[0];
            let frac_part = if parts.len() > 1 { parts[1] } else { "0" };

            let int_value = u64::from_str_radix(int_part, 16).unwrap() as f64;
            let mut frac_value = 0.0;
            for (i, c) in frac_part.chars().enumerate() {
                let digit = c.to_digit(16).unwrap();
                frac_value += digit as f64 / 16f64.powi((i as i32) + 1);
            }
            let mantissa = int_value + frac_value;

            let exponent: i32 = exponent_str.parse().unwrap();
            return Token::Float(mantissa * 2f64.powi(exponent));
        }

        // Hex-Integer
        Token::Integer(i64::from_str_radix(&s, 16).unwrap())
    }

    fn read_char(&mut self) -> Token {
        self.advance(1); // Überspringe das erste `'`
        let c = match self.peek(0) {
            Some('\\') => {
                self.advance(1);
                match self.peek(0) {
                    Some('n') => { self.advance(1); '\n' }
                    Some('t') => { self.advance(1); '\t' }
                    Some('r') => { self.advance(1); '\r' }
                    Some('\\') => { self.advance(1); '\\' }
                    Some('"') => { self.advance(1); '"' }
                    Some('\'') => { self.advance(1); '\'' }
                    Some(other) => { self.advance(1); other } // einfache Escape-Fallback
                    None => panic!("Unvollständiges Escape-Zeichen in Char-Literal"),
                }
            }
            Some(c) => { self.advance(1); c },
            None => panic!("Unvollständiges Char-Literal"),
        };

        // Erwartet abschließendes `'`
        if self.peek(0) != Some('\'') {
            panic!("Char-Literal muss genau ein Zeichen enthalten");
        }
        self.advance(1);

        Token::Char(c)
    }

    fn read_string(&mut self) -> Token {
        self.advance(1); // Überspringe das erste `"`

        let mut result = String::new();
        while let Some(c) = self.peek(0) {
            if c == '"' {
                self.advance(1); // Ende des Strings
                break;
            }

            let ch = if c == '\\' {
                self.advance(1);
                match self.peek(0) {
                    Some('n') => { self.advance(1); '\n' }
                    Some('t') => { self.advance(1); '\t' }
                    Some('r') => { self.advance(1); '\r' }
                    Some('\\') => { self.advance(1); '\\' }
                    Some('"') => { self.advance(1); '"' }
                    Some('\'') => { self.advance(1); '\'' }
                    Some(other) => { self.advance(1); other }
                    None => panic!("Unvollständiges Escape-Zeichen in String-Literal"),
                }
            } else {
                self.advance(1);
                c
            };

            result.push(ch);
        }

        Token::String(result)
    }
}
