use crate::errors::FqError;

/// Represents a token in a query.
#[derive(Debug)]
pub enum Token {
    // keywords
    SELECT,
    FROM,
    LIMIT,
    WHERE,
    INTK,
    FLOATK,
    STRK,
    BOOLK,
    TRUE,
    FALSE,
    //
    LPAREN,
    RPAREN,
    STAR,
    COMMA,
    ID(String),
    PATH(String),
    STRING(String),
    INT(i32),
    FLOAT(f32),
    COLUMN(u32),
    GT,
    LT,
    EQ,
    GE,
    LE,
    NE,
    PLUS,
    MINUS,
    DIV,
}

const COLUMN_PREFIX: char = '@';
const COLUMN_SEP: char = ',';

/// Main tokenization loop. It splits the given string into a sequence
/// of tokens.
///
/// # Errors
///
/// Returns an error if there is an invalid token.
pub fn tokenize(query: &str) -> Result<Vec<Token>, FqError> {
    let mut tokens = Vec::new();

    let chars: Vec<char> = query.chars().collect();
    let mut index: usize = 0;

    let len = chars.len();
    while index < len {
        match chars[index] {
            '(' => {
                index += 1;
                tokens.push(Token::LPAREN);
            }
            ')' => {
                index += 1;
                tokens.push(Token::RPAREN);
            }
            '>' => {
                index += 1;
                if index < len && chars[index] == '=' {
                    index += 1;
                    tokens.push(Token::GE);
                } else {
                    tokens.push(Token::GT);
                }
            }
            '<' => {
                index += 1;
                if index < len && chars[index] == '=' {
                    index += 1;
                    tokens.push(Token::LE);
                } else {
                    tokens.push(Token::LT);
                }
            }
            '!' => {
                index += 1;
                if index < len && chars[index] == '=' {
                    index += 1;
                    tokens.push(Token::NE);
                } else {
                    return Err(FqError::syntax("Unknown char '!'"));
                }
            }
            '=' => {
                index += 1;
                tokens.push(Token::EQ);
            }
            '*' => {
                index += 1;
                tokens.push(Token::STAR);
            }
            '+' => {
                index += 1;
                tokens.push(Token::PLUS);
            }
            '-' => {
                index += 1;
                if index < len && matches!(chars[index], '0'..='9') {
                    tokens.push(eat_negative_number(&chars, &mut index)?);
                } else {
                    tokens.push(Token::MINUS);
                }
            }
            '/' => {
                index += 1;
                tokens.push(Token::DIV);
            }
            COLUMN_SEP => {
                index += 1;
                tokens.push(Token::COMMA);
            }
            ' ' | '\t' | '\n' | '\r' => {
                index += 1;
            }
            '\'' => {
                index += 1;
                tokens.push(eat_string_literal(&chars, &mut index)?);
            }
            'a'..='z' => {
                match tokens.last() {
                    // way better to push this into parser
                    Some(Token::FROM) => tokens.push(eat_path(&chars, &mut index)?),
                    _ => tokens.push(eat_identifier(&chars, &mut index)?),
                }
            }
            '0'..='9' => {
                tokens.push(eat_number(&chars, &mut index)?);
            }
            COLUMN_PREFIX => {
                tokens.push(eat_column_ref(&chars, &mut index)?);
            }
            _ => {
                return Err(FqError::syntax("Unknown char"));
            }
        }
    }

    Ok(tokens)
}

fn eat_string_literal(chars: &Vec<char>, index: &mut usize) -> Result<Token, FqError> {
    let mut word = String::new();
    let len = chars.len();
    while *index < chars.len() {
        match chars[*index] {
            // ' starts a new string literal
            '\'' => {
                *index += 1;
                // we check if it is escaped or it is an end
                if *index < len && chars[*index] == '\'' {
                    *index += 1;
                    word.push('\'');
                } else {
                    return Ok(Token::STRING(word));
                }
            }
            c => {
                *index += 1;
                word.push(c);
            }
        }
    }
    Err(FqError::syntax("Incomplete string literal"))
}

/// Eats an identifier, path, or a keyword. This function is invoked
/// only if we know that the sequence starts with a letter.
///
/// #Errors
///
/// Returns an error if an identifier cannot be taken from the
/// beginning of the given sequence.
fn eat_identifier(chars: &Vec<char>, index: &mut usize) -> Result<Token, FqError> {
    let mut word = String::new();
    while *index < chars.len() {
        match chars[*index] {
            'a'..='z' => {
                word.push(chars[*index]);
                *index += 1;
            }
            '0'..='9' => {
                word.push(chars[*index]);
                *index += 1;
            }
            _ => {
                break;
            }
        }
    }

    match word.as_str() {
        "select" => Ok(Token::SELECT),
        "from" => Ok(Token::FROM),
        "limit" => Ok(Token::LIMIT),
        "where" => Ok(Token::WHERE),
        "int" => Ok(Token::INTK),
        "float" => Ok(Token::FLOATK),
        "str" => Ok(Token::STRK),
        "bool" => Ok(Token::BOOLK),
        "true" => Ok(Token::TRUE),
        "false" => Ok(Token::FALSE),
        _ => Ok(Token::ID(word)),
    }
}

fn eat_path(chars: &Vec<char>, index: &mut usize) -> Result<Token, FqError> {
    let mut word = String::new();
    let len = chars.len();
    while *index < chars.len() {
        match chars[*index] {
            ' ' | '\t' | '\n' | '\r' => {
                break;
            }
            '\\' => {
                *index += 1;
                if *index < len {
                    word.push(chars[*index]);
                    *index += 1
                } else {
                    return Err(FqError::syntax("Incorrect escape sequence"));
                }
            }
            c => {
                word.push(c);
                *index += 1;
            }
        }
    }
    Ok(Token::PATH(word))
}

/// Creates a number token from a sequence of chars. This function is
/// invoked only if we know that the sequence stars with a number.
///
/// # Errors
///
/// Returns an error if a number cannot be taken from the beginning of
/// the given character sequence.
fn eat_number(chars: &Vec<char>, index: &mut usize) -> Result<Token, FqError> {
    let mut number = String::new();
    while *index < chars.len() {
        match chars[*index] {
            '0'..='9' | '.' => {
                number.push(chars[*index]);
                *index += 1;
            }
            _ => {
                break;
            }
        }
    }

    // Check if number and create either INT or FLOAT.
    match number.parse::<i32>() {
        Ok(num) => Ok(Token::INT(num)),
        Err(_) => {
            // try to parse as float
            match number.parse::<f32>() {
                Ok(num) => Ok(Token::FLOAT(num)),
                Err(_) => Err(FqError::syntax("Not a number")),
            }
        }
    }
}

fn eat_negative_number(chars: &Vec<char>, index: &mut usize) -> Result<Token, FqError> {
    match eat_number(chars, index)? {
        Token::INT(val) => Ok(Token::INT(-val)),
        Token::FLOAT(val) => Ok(Token::FLOAT(-val)),
        _ => Err(FqError::internal(
            "Not a number when tokenizing a negative number",
        )),
    }
}

fn eat_column_ref(chars: &Vec<char>, index: &mut usize) -> Result<Token, FqError> {
    // eat COLUMN_PREFIX
    *index += 1;
    let mut number = String::new();
    while *index < chars.len() {
        match chars[*index] {
            '0'..='9' => {
                number.push(chars[*index]);
                *index += 1;
            }
            _ => {
                break;
            }
        }
    }

    if number.len() == 0 {
        return Err(FqError::syntax(
            "Column prefix has to be followed by an integer",
        ));
    }

    let number: u32 = number.parse().expect("could not parse column number");
    Ok(Token::COLUMN(number))
}

pub struct Lexer {
    tokens: Vec<Token>,
    index: usize,
}

impl Lexer {
    pub fn from(query: &str) -> Result<Lexer, FqError> {
        let tokens = tokenize(query)?;
        Ok(Lexer { tokens, index: 0 })
    }

    #[allow(dead_code)]
    pub fn peek(&self) -> Option<&Token> {
        if self.index < self.tokens.len() {
            return Some(&self.tokens[self.index]);
        }
        None
    }

    pub fn next(&mut self) -> Option<&Token> {
        if self.index < self.tokens.len() {
            let ix = self.index;
            self.index += 1;
            return Some(&self.tokens[ix]);
        }
        None
    }

    pub fn is_empty(&self) -> bool {
        self.index == self.tokens.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_lparen() {
        let query = "(";
        let tokens = tokenize(query).expect("Tokenization failed");

        assert_eq!(tokens.len(), 1, "Expected one token, got {}", tokens.len());
        assert!(
            matches!(tokens.first(), Some(Token::LPAREN)),
            "The token should be LPAREN"
        );
    }

    #[test]
    fn tokenize_int() {
        let tokens = tokenize("123").expect("Tokenization failed");
        assert_eq!(tokens.len(), 1, "Expected one token, got {}", tokens.len());
        assert!(
            matches!(tokens.first(), Some(Token::INT(s)) if *s == 123),
            "The token should be INT"
        );

        let tokens = tokenize("0123").expect("Tokenization failed");
        assert_eq!(tokens.len(), 1, "Expected one token, got {}", tokens.len());
        assert!(
            matches!(tokens.first(), Some(Token::INT(s)) if *s == 123),
            "The token should be INT"
        );
    }

    #[test]
    fn tokenize_float() {
        let tokens = tokenize("123.33").expect("Tokenization failed");
        assert_eq!(tokens.len(), 1, "Expected one token, got {}", tokens.len());
        assert!(
            matches!(tokens.first(), Some(Token::FLOAT(s)) if (*s - 123.33).abs() < f32::EPSILON),
            "The token should be FLOAT"
        );
    }

    #[test]
    fn tokenize_zero() {
        let tokens = tokenize("0").expect("Tokenization failed");
        assert_eq!(tokens.len(), 1, "Expected one token, got {}", tokens.len());
        assert!(
            matches!(tokens.first(), Some(Token::INT(s)) if *s == 0),
            "The toke should be INT(0)"
        );
    }

    #[test]
    fn tokenize_column_ref() {
        let query = format!("{}123", COLUMN_PREFIX);
        let tokens = tokenize(&query).expect("Tokenization failed");

        assert_eq!(tokens.len(), 1, "Expected one token, got {}", tokens.len());
        assert!(
            matches!(tokens.first(), Some(Token::COLUMN(s)) if *s == 123),
            "The token should be COLUMN"
        )
    }

    #[test]
    fn tokenize_string_literal() {
        let tokens = tokenize("'this is a string'").expect("Tokenization of a string failed");
        assert_eq!(tokens.len(), 1, "Expected one token, got {}", tokens.len());
        assert!(
            matches!(tokens.first(), Some(Token::STRING(s)) if s == "this is a string"),
            "The token should be STRING"
        );

        let tokens = tokenize("'this is a string with '' escaped '''")
            .expect("Tokenization of a string failed");
        assert_eq!(tokens.len(), 1, "Expected one token, got {}", tokens.len());
        assert!(
            matches!(tokens.first(), Some(Token::STRING(s)) if s == "this is a string with ' escaped '"),
            "The token should be STRING"
        );
    }

    #[test]
    fn tokenize_identifier() {
        let tokens = tokenize("abc").expect("Tokenization of an identifier failed");
        assert_eq!(tokens.len(), 1);
        assert!(
            matches!(tokens.first(), Some(Token::ID(s)) if s == "abc"),
            "The token should be ID"
        );
    }

    #[test]
    fn tokenize_path() {
        let tokens = tokenize("from a-b_c.txt").unwrap();
        assert_eq!(tokens.len(), 2);
        if let Token::PATH(s) = &tokens[1] {
            assert_eq!(s, "a-b_c.txt")
        } else {
            panic!("The token should be PATH");
        }
    }

    #[test]
    fn tokenize_keywords() {
        let tokens = tokenize("select").unwrap();
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens.first(), Some(Token::SELECT)));
    }
}
