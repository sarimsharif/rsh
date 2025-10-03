const DOUBLE_SYMBOLS: [&str; 8] = [">>", "<<", ">&", "<&", "`>", ">|", "&&", "||"];
const SINGLE_SYMBOLS: [char; 19] = [
    '!', ';', '&', '|', '<', '>', '(', ')', '{', '}', '\'', '"', '`', '\\', '#', '*', '?', '[', ']',
];
const KEYWORDS: [&str; 13] = [
    "case", "do", "done", "elif", "else", "esac", "fi", "for", "if", "in", "then", "while",
    "function",
];

/// A [`Token`] enum containing all possible tokens.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    /// Includes: ! ; & && || | ( ) { } > >> < <<
    Symbol(String),

    /// Includes: case do done elif else esac fi for if in then while function return export local readonly
    /// unset
    Keyword(String),

    /// Any other word.
    Word(String),
}

/// Errors that can occur while lexing source code.
#[derive(Debug)]
pub enum LexError {
    /// An unexpected character was encountered (e.g. `@` in plain source).
    UnexpectedCharacter(char),

    /// The input ended before a complete token could be formed.
    UnexpectedEOF,
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexError::UnexpectedCharacter(c) => write!(f, "Unexpected character: {}", c),
            LexError::UnexpectedEOF => write!(f, "Unexpected End Of File"),
        }
    }
}

impl std::error::Error for LexError {}

type LexResult<T> = Result<T, LexError>;

/// A simple lexer for a posix-compliant shell language.
pub struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    /// Creates a new lexer from some source code.
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.to_string(),
            position: 0,
        }
    }

    /// Peeks at the current position and returns a reference to the character.
    fn peek(&self) -> LexResult<char> {
        self.peek_ahead(0)
    }

    /// Peeks ahead of the position by an offset
    fn peek_ahead(&self, offset: usize) -> LexResult<char> {
        self.input
            .chars()
            .nth(self.position + offset)
            .ok_or(LexError::UnexpectedEOF)
    }

    /// Peeks two characters and puts them into a string.
    fn peek2(&mut self) -> LexResult<String> {
        let peek1 = self.peek()?;
        let peek2 = self.peek_ahead(1)?;
        Ok(format!("{}{}", peek1, peek2))
    }

    /// Advances the lexer.
    fn advance(&mut self) {
        self.position += 1;
    }

    /// Skips whitespace.
    fn skip_whitespace(&mut self) {
        while let Ok(ch) = self.peek() {
            if !ch.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    /// Gets a word.
    fn get_word(&mut self) -> String {
        let mut word = String::new();
        while let Ok(ch) = self.peek() {
            if !ch.is_alphanumeric() && ch != '_' {
                break;
            }
            word.push(ch);
            self.advance();
        }
        word
    }

    /// Gets the next token.
    fn next(&mut self) -> LexResult<Token> {
        self.skip_whitespace();

        let peek2ed = self.peek2()?;
        if DOUBLE_SYMBOLS.contains(&peek2ed.as_str()) {
            self.advance();
            self.advance();
            self.skip_whitespace();
            return Ok(Token::Symbol(peek2ed));
        }

        let peeked = self.peek()?;
        if SINGLE_SYMBOLS.contains(&peeked) {
            self.advance();
            self.skip_whitespace();
            return Ok(Token::Symbol(format!("{}", peeked)));
        }

        if peeked.is_alphanumeric() || peeked == '_' {
            let word = self.get_word();
            self.skip_whitespace();
            if KEYWORDS.contains(&word.as_str()) {
                return Ok(Token::Keyword(word));
            }
            return Ok(Token::Word(word));
        }
        Err(LexError::UnexpectedCharacter(peeked))
    }

    /// Gets all the tokens out of the source with the lexer.
    pub fn tokens(&mut self) -> LexResult<Vec<Token>> {
        let mut tokens = Vec::new();
        while self.position < self.input.len() {
            tokens.push(self.next()?);
            dbg!(&self.input[self.position..]);
        }
        Ok(tokens)
    }
}
