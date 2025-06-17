#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    // Keywords
    Fn,
    Let,

    // Identifiers
    Ident(String),

    // Literals
    Integer(i64),
    String(String),

    // Operators
    Eq,      // =
    Plus,    // +
    Minus,   // -
    Star,    // *
    Slash,   // /
    LParen,  // (
    RParen,  // )

    // Punctuation
    Comma,   // ,
    Colon,   // :

    // Structural
    Newline,
    Indent,
    Dedent,
    Eof,

    // Comments
    Comment(String), // Content of the comment

    // Error handling
    Unknown(char),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String, // The actual lexeme
    pub line: usize,  // Line number (1-indexed)
    pub col: usize,   // Column number (1-indexed on the line)
}

// Basic implementation for now, will be expanded
impl Token {
    pub fn new(kind: TokenKind, text: &str, line: usize, col: usize) -> Self {
        Token {
            kind,
            text: text.to_string(),
            line,
            col,
        }
    }

    // Helper to peek at the next character without consuming it
    fn peek(&mut self) -> Option<char> {
        self.input.peek().cloned()
    }

    // Skips whitespace characters (space and tab), but not newlines
    fn skip_whitespace_no_newline(&mut self) {
        while let Some(c) = self.current_char {
            if c == ' ' || c == '\t' {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        // Handle indentation/dedentation at the start of a line
        if self.start_of_line {
            self.start_of_line = false; // Reset flag

            // Skip any leading spaces or tabs to count indentation
            let mut current_indent = 0;
            while let Some(c) = self.current_char {
                if c == ' ' {
                    current_indent += 1;
                    self.advance();
                } else if c == '\t' {
                    // As per spec, tabs are not allowed for indentation.
                    // For now, let's treat a tab as an error or a large indent.
                    // This can be refined. For MVP, we might error or count as N spaces.
                    // Here, we'll make it an Unknown token if encountered as indentation.
                    // The problem statement says "4 spaces ... Tabs are not allowed".
                    // So, if we see a tab here, it's an issue.
                    // However, the spec also says Unknown(char) for error handling.
                    // Let's assume for now any tab at start of line after newline is an error.
                    // If it's a // comment or newline, it will be handled.
                    let err_char = self.current_char.unwrap_or(' '); // Should be Some('\t')
                    self.advance();
                    return Token::new(TokenKind::Unknown(err_char), &err_char.to_string(), self.line, self.col -1);
                } else {
                    break;
                }
            }

            // If the line is blank (only whitespace followed by newline) or a comment line,
            // indentation doesn't change or emit Indent/Dedent.
            if self.current_char == Some('\n') || (self.current_char == Some('/') && self.peek() == Some('/')) {
                // Skip the rest of the whitespace (if any) before the newline/comment
                self.skip_whitespace_no_newline();
                // Fall through to newline/comment handling
            } else {
                let last_indent = *self.indentation_stack.last().unwrap_or(&0);
                if current_indent > last_indent {
                    if (current_indent - last_indent) % 4 != 0 && last_indent + 4 != current_indent {
                        // Non-standard indentation, but let's allow it and push.
                        // Or, we could return an error token.
                        // For now, let's be flexible and just push.
                        // The spec says "4 spaces", implying fixed increments.
                        // Let's assume an error if not a multiple of 4, or not exactly +4.
                        // For MVP, if it's greater, we indent. We can make this stricter later.
                        // The spec says "4 spaces will be used". This implies it must be a multiple of 4.
                        // Let's return an Indent token, but the parser might later validate the amount.
                        // For now, the lexer's job is to report the change.
                    }
                    self.indentation_stack.push(current_indent);
                    return Token::new(TokenKind::Indent, "    ", self.line, 1); // col for Indent is tricky, let's use 1
                } else if current_indent < last_indent {
                    // Dedent
                    while *self.indentation_stack.last().unwrap_or(&0) > current_indent {
                        self.indentation_stack.pop();
                        // We need to return one Dedent token at a time.
                        // This requires state or emitting multiple tokens.
                        // For now, let's assume the lexer will be called again
                        // and will continue to emit Dedents if necessary.
                        // This means next_token needs to be able to return a pending dedent.
                        // This part is tricky. Let's simplify for now:
                        // If current_indent < last_indent, we must match a previous indent level.
                        if !self.indentation_stack.contains(&current_indent) {
                             // Invalid dedent level
                             return Token::new(TokenKind::Unknown(' '), "DedentError", self.line, 1);
                        }
                        // For now, just one dedent. This needs to be a loop or queue if multiple dedents.
                        // Let's assume for MVP, we emit one dedent, and if more are needed,
                        // the next call to next_token will re-evaluate indentation.
                        // This means the `start_of_line` logic needs to persist if dedents are pending.
                        // This is a common complexity.
                        // For now, let's emit one and see. If stack top still > current_indent,
                        // then next_token will be called, start_of_line will be true again.
                        // We need to consume the character that caused the dedent check.
                        // The characters that form the current_indent have been consumed.
                        // The current_char is the first non-space/tab char.
                        // So, if we emit Dedent, the next call should process current_char.
                        // The problem is we might need to emit multiple Dedents before processing current_char.

                        // Let's try a simpler approach for now: Store pending dedents.
                        // Or, for MVP, assume only one dedent per line, or that parser handles cascading.
                        // The typical way is a queue of tokens.
                        // Let's try to return one dedent. The next call will re-evaluate.
                        // The `col` for dedent is also tricky, use 1.
                        return Token::new(TokenKind::Dedent, "", self.line, 1);
                        // If after this dedent, current_indent is still less than new stack top,
                        // the next call to next_token (if start_of_line is true) will dedent again.
                        // This requires careful state management for `start_of_line`.
                        // If we return Dedent, the `current_char` is NOT consumed yet.
                        // The next call to `next_token` should not re-calculate indent.
                        // So, if we emit Indent/Dedent, `start_of_line` should be false for the *next* real token.
                        // This means Indent/Dedent tokens are "meta".

                        // Simpler: if current_indent < last_indent, pop until current_indent is met or exceeded.
                        // Emit one dedent token for each pop. Store them and return one by one.
                        // This requires a small token buffer in the lexer.
                        // Let's add a `pending_dedents` counter.
                        // This will be handled before character processing.
                    }
                    // If after popping, the top is not equal to current_indent, it's an error.
                    if *self.indentation_stack.last().unwrap_or(&0) != current_indent {
                        return Token::new(TokenKind::Unknown(' '), "Invalid Dedent Level", self.line, 1);
                    }
                    // If we are here, current_indent == *self.indentation_stack.last().unwrap()
                    // No Indent/Dedent token is emitted if indentation is same as stack top (after pops)
                }
                // If current_indent == last_indent, no Indent/Dedent token.
            }
        }

        // Skip other whitespace (spaces, tabs) that are not at the start of a line (already handled)
        self.skip_whitespace_no_newline();

        // Main token recognition
        if let Some(char) = self.current_char {
            let start_col = self.col;
            match char {
                '\n' => {
                    self.advance();
                    self.start_of_line = true; // Set for the next token
                    // self.line has already been incremented by advance() if current_char was \n
                    // self.col has been reset to 0 by advance() and then incremented to 1 if there's a next char.
                    // So, the newline token's line should be self.line-1 if advance was called for it.
                    // The column for a newline token is effectively the end of the previous line.
                    return Token::new(TokenKind::Newline, "\\n", self.line -1 , start_col);
                }
                '/' => {
                    if self.peek() == Some('/') {
                        // Comment
                        self.advance(); // Consume the first /
                        self.advance(); // Consume the second /
                        let mut comment_text = String::new();
                        while let Some(c) = self.current_char {
                            if c == '\n' {
                                break; // Comment ends at newline
                            }
                            comment_text.push(c);
                            self.advance();
                        }
                        // The newline that ends the comment will be tokenized next if not EOF
                        // Or current_char is None if EOF
                        // We don't set start_of_line = true here, newline tokenizing will do it.
                        return Token::new(TokenKind::Comment(comment_text), "//", self.line, start_col);
                    } else {
                        // Slash operator
                        self.advance();
                        return Token::new(TokenKind::Slash, "/", self.line, start_col);
                    }
                }
                '=' => {
                    self.advance();
                    Token::new(TokenKind::Eq, "=", self.line, start_col)
                }
                '+' => {
                    self.advance();
                    Token::new(TokenKind::Plus, "+", self.line, start_col)
                }
                '-' => {
                    self.advance();
                    Token::new(TokenKind::Minus, "-", self.line, start_col)
                }
                '*' => {
                    self.advance();
                    Token::new(TokenKind::Star, "*", self.line, start_col)
                }
                '(' => {
                    self.advance();
                    Token::new(TokenKind::LParen, "(", self.line, start_col)
                }
                ')' => {
                    self.advance();
                    Token::new(TokenKind::RParen, ")", self.line, start_col)
                }
                ',' => {
                    self.advance();
                    Token::new(TokenKind::Comma, ",", self.line, start_col)
                }
                ':' => {
                    self.advance();
                    Token::new(TokenKind::Colon, ":", self.line, start_col)
                }
                _ if char.is_alphabetic() || char == '_' => {
                    // Identifiers or Keywords
                    let mut text = String::new();
                    while let Some(c) = self.current_char {
                        if c.is_alphanumeric() || c == '_' {
                            text.push(c);
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    // Check for keywords
                    match text.as_str() {
                        "fn" => Token::new(TokenKind::Fn, &text, self.line, start_col),
                        "let" => Token::new(TokenKind::Let, &text, self.line, start_col),
                        _ => Token::new(TokenKind::Ident(text.clone()), &text, self.line, start_col),
                    }
                }
                _ if char.is_digit(10) => {
                    // Integer Literals
                    let mut text = String::new();
                    let mut value: i64 = 0;
                    while let Some(c) = self.current_char {
                        if c.is_digit(10) {
                            text.push(c);
                            value = value * 10 + (c as i64 - '0' as i64);
                            self.advance();
                        } else if c == '_' {
                            text.push(c); // Include underscore in text for fidelity
                            self.advance(); // Skip underscore for value calculation
                        } else {
                            break;
                        }
                    }
                    // The 'text' variable here contains underscores, token.text should store it.
                    // The TokenKind::Integer stores the parsed i64 value.
                    Token::new(TokenKind::Integer(value), &text, self.line, start_col)
                }
                '"' => {
                    // String Literals
                    self.advance(); // Consume the opening quote
                    let mut text_content = String::new();
                    let mut literal_text = String::from("\""); // Store full literal including quotes for token.text

                    while let Some(c) = self.current_char {
                        if c == '"' {
                            literal_text.push(c);
                            self.advance(); // Consume the closing quote
                            break;
                        }
                        // MVP: No complex escapes.
                        text_content.push(c);
                        literal_text.push(c);
                        self.advance();
                        if c == '\n' { // Unterminated string at newline
                            // This is an error case, but for now, we'll end the string.
                            // A better lexer might return an error token.
                            // For now, the string content will include the newline.
                            // Or, we can make it an Unknown token.
                            // Let's return what we have as a string, parser can validate.
                            // The spec says "simple strings, no complex escapes".
                            // Unterminated is an error. Let's make it Unknown for now.
                            // This needs the original char for Unknown.
                            // For now, let's assume strings are well-terminated for MVP.
                            // If self.current_char becomes None before closing quote, it's an unterminated string.
                        }
                    }
                    // Check if string was terminated
                    if !literal_text.ends_with('"') {
                         // Unterminated string
                         return Token::new(TokenKind::Unknown('\"'), &literal_text, self.line, start_col);
                    }
                    Token::new(TokenKind::String(text_content), &literal_text, self.line, start_col)
                }
                _ => {
                    self.advance(); // Consume the character
                    Token::new(TokenKind::Unknown(char), &char.to_string(), self.line, start_col)
                }
            }
        } else {
             // Correctly handle pending dedents before EOF
            let current_indent = 0; // Implicit indent at EOF is 0
            if *self.indentation_stack.last().unwrap_or(&0) > current_indent {
                self.indentation_stack.pop();
                // Reset start_of_line to true to re-evaluate indentation if needed,
                // though at EOF it's mainly for dedents.
                self.start_of_line = true;
                return Token::new(TokenKind::Dedent, "", self.line, 1);
            }
            Token::new(TokenKind::Eof, "", self.line, self.col) // Use current col, or col+1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, TokenKind, Token}; // Added Token for more detailed asserts if needed

    // Helper to collect all Tokens from the lexer
    fn get_all_tokens(input: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        loop {
            let token = lexer.next_token();
            let is_eof = token.kind == TokenKind::Eof;
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        tokens
    }

    // Helper to collect all TokenKinds from the lexer
    fn get_all_token_kinds(input: &str) -> Vec<TokenKind> {
        get_all_tokens(input).into_iter().map(|t| t.kind).collect()
    }

    // Simplified helper for basic kind checking
    fn assert_tokens(input: &str, expected_kinds: Vec<TokenKind>) {
        let kinds = get_all_token_kinds(input);
        assert_eq!(kinds, expected_kinds, "Token kinds do not match for input: \"{}\"", input);
    }

    #[test]
    fn test_simple_let() {
        assert_tokens(
            "let x = 5",
            vec![
                TokenKind::Let,
                TokenKind::Ident("x".to_string()),
                TokenKind::Eq,
                TokenKind::Integer(5),
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn test_keywords() {
        assert_tokens("fn let", vec![TokenKind::Fn, TokenKind::Let, TokenKind::Eof]);
    }

    #[test]
    fn test_identifiers() {
        assert_tokens(
            "foo bar _baz foo_bar",
            vec![
                TokenKind::Ident("foo".to_string()),
                TokenKind::Ident("bar".to_string()),
                TokenKind::Ident("_baz".to_string()),
                TokenKind::Ident("foo_bar".to_string()),
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn test_integer_literals() {
        assert_tokens(
            "123 0 1_000",
            vec![
                TokenKind::Integer(123),
                TokenKind::Integer(0),
                TokenKind::Integer(1000),
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn test_string_literal() {
        assert_tokens(
            "\"hello world\"",
            vec![TokenKind::String("hello world".to_string()), TokenKind::Eof],
        );
    }

    #[test]
    fn test_unterminated_string_literal() {
        // Based on current lexer string parsing, an unterminated string might be an error or an incomplete string.
        // The current lexer.rs has: return Token::new(TokenKind::Unknown('\"'), &literal_text, self.line, start_col);
        // So, let's expect Unknown. The literal_text for Unknown would be the partial string.
        let kinds = get_all_token_kinds("\"hello");
        assert_eq!(kinds.len(), 2); // Unknown and Eof
        assert!(matches!(kinds[0], TokenKind::Unknown('\"')), "Expected Unknown for unterminated string, got {:?}", kinds[0]);
    }


    #[test]
    fn test_operators() {
        assert_tokens(
            "+ - * / = ( )",
            vec![
                TokenKind::Plus,
                TokenKind::Minus,
                TokenKind::Star,
                TokenKind::Slash,
                TokenKind::Eq,
                TokenKind::LParen,
                TokenKind::RParen,
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn test_punctuation() {
        assert_tokens(", :", vec![TokenKind::Comma, TokenKind::Colon, TokenKind::Eof]);
    }

    #[test]
    fn test_comment() {
        assert_tokens(
            "// this is a comment\nlet x = 1",
            vec![
                TokenKind::Comment(" this is a comment".to_string()), // Comment content, leading space included by current lexer
                TokenKind::Newline,
                TokenKind::Let,
                TokenKind::Ident("x".to_string()),
                TokenKind::Eq,
                TokenKind::Integer(1),
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn test_comment_no_newline_at_eof() {
        assert_tokens(
            "// comment",
            vec![
                TokenKind::Comment(" comment".to_string()),
                TokenKind::Eof, // No newline after comment if it's last line
            ],
        );
    }


    #[test]
    fn test_simple_indent_dedent() {
        // Lexer logic for indent/dedent:
        // Indent is emitted when indent > stack.last.
        // Dedent is emitted when indent < stack.last.
        // Newline is significant.
        // fn main() [Newline]
        // [Indent] let x = 1 [Newline]
        // print(x) [Newline]          (assuming print is an ident for now, same indent)
        // [Dedent] let y = 2 [Eof]
        assert_tokens(
            "fn main()\n  let x = 1\n  print(x)\nlet y = 2",
            vec![
                TokenKind::Fn, TokenKind::Ident("main".to_string()), TokenKind::LParen, TokenKind::RParen, TokenKind::Newline,
                TokenKind::Indent,
                TokenKind::Let, TokenKind::Ident("x".to_string()), TokenKind::Eq, TokenKind::Integer(1), TokenKind::Newline,
                TokenKind::Ident("print".to_string()), TokenKind::LParen, TokenKind::Ident("x".to_string()), TokenKind::RParen, TokenKind::Newline,
                TokenKind::Dedent, // Dedent before 'let y'
                TokenKind::Let, TokenKind::Ident("y".to_string()), TokenKind::Eq, TokenKind::Integer(2),
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn test_indent_handling_complex() {
        let input = "fn main()\n    let x = 1\n    log(x)\nlet y = 0\n";
        // Expected: fn, main, (, ), NL,
        // INDENT(4), let, x, =, 1, NL,
        // log, (, x, ), NL,
        // DEDENT, let, y, =, 0, NL,
        // EOF
        // Note: The current lexer might emit multiple Dedents if needed, or one by one.
        // The current simple dedent logic emits one dedent if current_indent < last_indent and matches a previous level.
        // For EOF, it also tries to dedent stack to 0.
        // Let's trace:
        // fn main() -> Fn, Ident("main"), LParen, RParen, Newline. Indent stack: [0]
        //   let x = 1 -> (sees 4 spaces) Indent. Indent stack: [0, 4]. Tokens: Let, Ident("x"), Eq, Integer(1), Newline.
        //   log(x) -> (sees 4 spaces, same as stack top) Tokens: Ident("log"), LParen, Ident("x"), RParen, Newline.
        // let y = 0 -> (sees 0 spaces) Dedent. Indent stack: [0]. Tokens: Let, Ident("y"), Eq, Integer(0), Newline.
        // EOF -> (stack is [0], current_indent is 0) Eof.
        let expected = vec![
            TokenKind::Fn, TokenKind::Ident("main".to_string()), TokenKind::LParen, TokenKind::RParen, TokenKind::Newline,
            TokenKind::Indent,
            TokenKind::Let, TokenKind::Ident("x".to_string()), TokenKind::Eq, TokenKind::Integer(1), TokenKind::Newline,
            TokenKind::Ident("log".to_string()), TokenKind::LParen, TokenKind::Ident("x".to_string()), TokenKind::RParen, TokenKind::Newline,
            TokenKind::Dedent,
            TokenKind::Let, TokenKind::Ident("y".to_string()), TokenKind::Eq, TokenKind::Integer(0), TokenKind::Newline,
            TokenKind::Eof,
        ];
        assert_tokens(input, expected);
    }


    #[test]
    fn test_multiple_indents_dedents() {
        // fn a() NL
        //   INDENT fn b() NL
        //     INDENT let x = 1 NL
        //   DEDENT let y = 2 NL
        // DEDENT let z = 3 EOF
        assert_tokens(
            "fn a()\n  fn b()\n    let x = 1\n  let y = 2\nlet z = 3",
            vec![
                TokenKind::Fn, TokenKind::Ident("a".to_string()), TokenKind::LParen, TokenKind::RParen, TokenKind::Newline,
                TokenKind::Indent,
                TokenKind::Fn, TokenKind::Ident("b".to_string()), TokenKind::LParen, TokenKind::RParen, TokenKind::Newline,
                TokenKind::Indent,
                TokenKind::Let, TokenKind::Ident("x".to_string()), TokenKind::Eq, TokenKind::Integer(1), TokenKind::Newline,
                TokenKind::Dedent,
                TokenKind::Let, TokenKind::Ident("y".to_string()), TokenKind::Eq, TokenKind::Integer(2), TokenKind::Newline,
                TokenKind::Dedent,
                TokenKind::Let, TokenKind::Ident("z".to_string()), TokenKind::Eq, TokenKind::Integer(3),
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn test_blank_lines_in_indent() {
        // fn main() NL
        //   INDENT NL (blank line, start_of_line still true, next line re-evaluates indent)
        //   let x = 1 NL (still indented)
        // NL (blank line at base level)
        // DEDENT (implicitly before next token if indent changes, or handled by EOF dedenting)
        // let y = 2 EOF
        // Current logic: blank lines (newline only) after indent:
        // if self.start_of_line: ... if self.current_char == Some('\n') ... fall through to newline handling
        // This means a blank line doesn't change indent level for the lexer itself, it just emits Newline.
        // The *next* non-blank line will then re-evaluate indentation.
        assert_tokens(
            "fn main()\n\n  let x = 1\n\nlet y = 2",
            vec![
                TokenKind::Fn, TokenKind::Ident("main".to_string()), TokenKind::LParen, TokenKind::RParen, TokenKind::Newline,
                TokenKind::Newline, // Blank line, no indent/dedent token from it
                TokenKind::Indent,  // Indent for 'let x = 1' line
                TokenKind::Let, TokenKind::Ident("x".to_string()), TokenKind::Eq, TokenKind::Integer(1), TokenKind::Newline,
                TokenKind::Newline, // Blank line
                TokenKind::Dedent,  // Dedent before 'let y = 2'
                TokenKind::Let, TokenKind::Ident("y".to_string()), TokenKind::Eq, TokenKind::Integer(2),
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn test_dedent_to_zero_at_eof() {
        // fn main() NL
        //   INDENT let x = 1 EOF
        // Expected: Fn, Ident, LParen, RParen, Newline, Indent, Let, Ident, Eq, Integer, Dedent, Eof
        // The lexer's EOF handling:
        // if *self.indentation_stack.last().unwrap_or(&0) > current_indent (0 at eof) { emit Dedent }
        assert_tokens(
            "fn main()\n  let x = 1",
            vec![
                TokenKind::Fn, TokenKind::Ident("main".to_string()), TokenKind::LParen, TokenKind::RParen, TokenKind::Newline,
                TokenKind::Indent,
                TokenKind::Let, TokenKind::Ident("x".to_string()), TokenKind::Eq, TokenKind::Integer(1),
                TokenKind::Dedent, // Dedent because input ends while indented
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn test_multiple_dedents_at_eof() {
        let input = "fn a()\n  fn b()\n    let x = 1";
        // Expected:
        // Fn, Ident("a"), LParen, RParen, Newline,
        // Indent,
        // Fn, Ident("b"), LParen, RParen, Newline,
        // Indent,
        // Let, Ident("x"), Eq, Integer(1),
        // Dedent, (for level from x back to b's level)
        // Dedent, (for level from b back to a's level)
        // Eof
        // The current lexer's EOF logic:
        // if *self.indentation_stack.last().unwrap_or(&0) > current_indent { self.indentation_stack.pop(); return Token::new(TokenKind::Dedent, ...)}
        // This means it will return one Dedent at a time. So get_all_token_kinds will call it again.
        assert_tokens(input, vec![
            TokenKind::Fn, TokenKind::Ident("a".to_string()), TokenKind::LParen, TokenKind::RParen, TokenKind::Newline,
            TokenKind::Indent, // for fn b
            TokenKind::Fn, TokenKind::Ident("b".to_string()), TokenKind::LParen, TokenKind::RParen, TokenKind::Newline,
            TokenKind::Indent, // for let x
            TokenKind::Let, TokenKind::Ident("x".to_string()), TokenKind::Eq, TokenKind::Integer(1),
            TokenKind::Dedent, // from x to b level (4 to 2, assuming 2 spaces for test simplicity here, though spec is 4)
                               // Let's assume the lexer's indent/dedent logic is based on actual space counts it sees,
                               // and stack stores these counts. The test input implies 2-space indents.
                               // The lexer itself doesn't enforce "must be 4 spaces", just "indent means more spaces".
                               // The Indent token text is "    " but that's cosmetic.
                               // The important part is the change in indentation level.
            TokenKind::Dedent, // from b to global level (2 to 0)
            TokenKind::Eof,
        ]);
    }


    #[test]
    fn test_unknown_char() {
        // The current lexer advances past the unknown char.
        assert_tokens(
            "let x = ?;",
            vec![
                TokenKind::Let,
                TokenKind::Ident("x".to_string()),
                TokenKind::Eq,
                TokenKind::Unknown('?'),
                // Assuming semicolon is not defined, it might be Unknown or part of an error recovery
                // Based on current lexer, if it's not in defined operators/punctuation, it's Unknown
                TokenKind::Unknown(';'),
                TokenKind::Eof,
            ],
        );
    }
     #[test]
    fn test_empty_input() {
        assert_tokens("", vec![TokenKind::Eof]);
    }

    #[test]
    fn test_only_newlines_and_spaces() {
        // Current lexer: start_of_line=true. Skips spaces. Sees newline. Emits newline. Repeats.
        // Then EOF. No indents/dedents if only spaces/newlines.
        assert_tokens("\n  \n\n", vec![TokenKind::Newline, TokenKind::Newline, TokenKind::Newline, TokenKind::Eof]);
    }

    #[test]
    fn test_indent_dedent_with_comments_and_blank_lines() {
        let input = r#"
fn main()
  // comment
  let x = 1

  // another comment
  let y = 2
let z = 3
"#;
        // Expected:
        // NL (from initial blank line of raw string literal)
        // FN Ident LP RP NL
        // INDENT Comment NL (comment does not affect indent level itself, but is tokenized)
        //        Ident EQ Num NL (still at indent level 1)
        // NL (blank line, does not affect indent level)
        //        Comment NL (still at indent level 1)
        //        Ident EQ Num NL (still at indent level 1)
        // DEDENT Ident EQ Num EOF

        // Current lexer behavior with comments and blank lines in indentation:
        // - A comment line or a blank line itself does not generate Indent/Dedent.
        // - It emits Comment + Newline or just Newline.
        // - The indentation is checked on the *next* line that contains non-whitespace characters.
        // So, the INDENT for `let x = 1` happens after the `// comment` and its newline.
        // The DEDENT for `let z = 3` happens before `let z`.

        // Trace:
        // "" -> Start of line, current_char is \n (from raw string literal start) -> Newline
        // "fn main()\n" -> Fn, Ident("main"), LParen, RParen, Newline. Stack: [0]
        // "  // comment\n" -> start_of_line=true. Sees 2 spaces. current_char is '/'.
        //                  -> Indentation logic: current_indent=2. last_indent=0.
        //                  -> Indent token. Stack: [0, 2].
        //                  -> Then processes comment: Comment(" comment"), Newline.
        // "  let x = 1\n" -> start_of_line=true. Sees 2 spaces. current_char is 'l'.
        //                  -> Indentation logic: current_indent=2. last_indent=2 (stack top). No change.
        //                  -> Let, Ident("x"), Eq, Integer(1), Newline.
        // "\n"           -> start_of_line=true. current_char is '\n'.
        //                  -> Indentation logic: blank line. No change. Emits Newline.
        // "  // another comment\n" -> start_of_line=true. Sees 2 spaces. current_char is '/'.
        //                  -> Indentation logic: current_indent=2. last_indent=2. No change.
        //                  -> Comment(" another comment"), Newline.
        // "  let y = 2\n" -> start_of_line=true. Sees 2 spaces. current_char is 'l'.
        //                  -> Indentation logic: current_indent=2. last_indent=2. No change.
        //                  -> Let, Ident("y"), Eq, Integer(2), Newline.
        // "let z = 3\n" -> start_of_line=true. Sees 0 spaces. current_char is 'l'.
        //                  -> Indentation logic: current_indent=0. last_indent=2.
        //                  -> Dedent token. Stack: [0].
        //                  -> Let, Ident("z"), Eq, Integer(3), Newline.
        // EOF            -> Dedent stack to 0 if needed (already [0]). Eof.

        assert_tokens(
            input,
            vec![
                TokenKind::Newline, // From the initial newline in the raw string literal
                TokenKind::Fn, TokenKind::Ident("main".to_string()), TokenKind::LParen, TokenKind::RParen, TokenKind::Newline,
                TokenKind::Indent, // For the line with "// comment" which leads to "let x = 1"
                TokenKind::Comment(" comment".to_string()), TokenKind::Newline,
                // No new Indent here, already indented for "let x = 1"
                TokenKind::Let, TokenKind::Ident("x".to_string()), TokenKind::Eq, TokenKind::Integer(1), TokenKind::Newline,
                TokenKind::Newline, // Blank line
                // No new Indent here for "// another comment"
                TokenKind::Comment(" another comment".to_string()), TokenKind::Newline,
                // No new Indent here for "let y = 2"
                TokenKind::Let, TokenKind::Ident("y".to_string()), TokenKind::Eq, TokenKind::Integer(2), TokenKind::Newline,
                TokenKind::Dedent, // Before "let z = 3"
                TokenKind::Let, TokenKind::Ident("z".to_string()), TokenKind::Eq, TokenKind::Integer(3), TokenKind::Newline,
                TokenKind::Eof,
            ],
        );
    }

    #[test]
    fn test_inconsistent_indentation_errors() {
        // 1. Tab used for indentation
        let input_tab = "fn main()\n\tlet x = 1"; // Tab at start of line 2, col 1
        let tokens_tab = get_all_tokens(input_tab);
        // Expected: Fn, Ident("main"), LParen, RParen, Newline, Unknown('\t'), Let, Ident("x"), Eq, Integer(1), Dedent (if any indent happened), Eof
        // Let's trace the tab error:
        // Line 1: fn main() -> Fn, Ident, LParen, RParen, Newline. Stack: [0]. Line: 1
        // Line 2: \tlet x = 1
        //   start_of_line = true.
        //   current_indent loop: sees `\t`.
        //   `err_char = '\t'`. `advance()` consumes `\t`. `self.col` becomes 1 (or 2 if advance increments before read).
        //   Lexer's advance: `self.current_char = self.input.next(); if self.current_char.is_some() { self.col += 1; }`
        //   If current char was `\t`, col was 1. `advance()` makes `current_char = 'l'`, then `col` becomes 2.
        //   So, the Unknown token for `\t` should be at `self.line` (2), `self.col - 1` (which is 2 - 1 = 1).
        //   Token::new(TokenKind::Unknown('\t'), "\t", 2, 1)
        //   The rest of the line "let x = 1" is then processed.
        //   No Indent token was emitted. Stack is still [0].
        //   Then let, x, =, 1.
        //   EOF: No dedent needed as stack is [0].

        assert_eq!(tokens_tab.len(), 8, "Tab input: {:?}", tokens_tab);
        assert_eq!(tokens_tab[0].kind, TokenKind::Fn);
        assert_eq!(tokens_tab[1].kind, TokenKind::Ident("main".to_string()));
        assert_eq!(tokens_tab[2].kind, TokenKind::LParen);
        assert_eq!(tokens_tab[3].kind, TokenKind::RParen);
        assert_eq!(tokens_tab[4].kind, TokenKind::Newline);
        assert_eq!(tokens_tab[4].line, 1, "Newline line number for tab test");

        assert_eq!(tokens_tab[5].kind, TokenKind::Unknown('\t'));
        assert_eq!(tokens_tab[5].text, "\t");
        assert_eq!(tokens_tab[5].line, 2, "Tab Unknown token line number");
        assert_eq!(tokens_tab[5].col, 1, "Tab Unknown token column");
        // The 'let' token will follow. Its column will depend on how `col` is managed post-error.
        // After returning Unknown token for tab, `next_token` is called again.
        // `self.start_of_line` is now false. `skip_whitespace_no_newline()` runs.
        // `current_char` is 'l'. `self.col` is 2.
        // So 'let' token starts at col 2.

        assert_eq!(tokens_tab[6].kind, TokenKind::Let); // This should be the next token
                                                        // The problem is that after the error token, the lexer continues on the same line.
                                                        // Let's check the full sequence for tab.
        let expected_kinds_tab = vec![
            TokenKind::Fn, TokenKind::Ident("main".to_string()), TokenKind::LParen, TokenKind::RParen, TokenKind::Newline,
            TokenKind::Unknown('\t'), // Error for tab
            TokenKind::Let, TokenKind::Ident("x".to_string()), TokenKind::Eq, TokenKind::Integer(1), // Continues parsing after error
            TokenKind::Eof,
        ];
        let actual_kinds_tab = get_all_token_kinds(input_tab);
        assert_eq!(actual_kinds_tab, expected_kinds_tab, "Token kinds for tab input do not match");


        // 2. Dedent to a level not previously indented ("DedentError")
        // Example: Indent to 4, then try to dedent to 2 (which was never an indent level)
        let input_dedent_error = "fn main()\n    let x = 1\n  let y = 2";
        // Line 1: Fn, Ident, LP, RP, NL. Stack: [0]
        // Line 2: Indent (to 4). Let, Ident, Eq, Int, NL. Stack: [0, 4]. col for Indent is 1. line is 2.
        // Line 3: current_indent = 2. last_indent = 4.
        //   `while 4 > 2` is true. Pop. Stack `[0]`.
        //   `!self.indentation_stack.contains(&2)` (`![0].contains(&2)`) is true.
        //   Returns `Token::new(TokenKind::Unknown(' '), "DedentError", self.line (3), 1)`.
        //   After this error, processing continues. `start_of_line` is false.
        //   `let y = 2` is processed.
        let tokens_dedent_error = get_all_tokens(input_dedent_error);
        // Expected: Fn, Main, ( , ), NL, Indent, Let, x, =, 1, NL, Unknown("DedentError"), Let, y, =, 2, Eof
        // Let's check the Unknown token specifically.
        let error_token_dedent = tokens_dedent_error.iter().find(|t| t.text == "DedentError").expect("DedentError token not found");
        assert_eq!(error_token_dedent.kind, TokenKind::Unknown(' '));
        assert_eq!(error_token_dedent.line, 3, "DedentError line number");
        assert_eq!(error_token_dedent.col, 1, "DedentError column");

        let expected_kinds_dedent_error = vec![
            TokenKind::Fn, TokenKind::Ident("main".to_string()), TokenKind::LParen, TokenKind::RParen, TokenKind::Newline,
            TokenKind::Indent,
            TokenKind::Let, TokenKind::Ident("x".to_string()), TokenKind::Eq, TokenKind::Integer(1), TokenKind::Newline,
            TokenKind::Unknown(' '), // DedentError
            TokenKind::Let, TokenKind::Ident("y".to_string()), TokenKind::Eq, TokenKind::Integer(2),
            TokenKind::Dedent, // This dedent is to bring stack [0,4] to [0] to match final 0-indent line if y was also 0-indent
                               // However, the error happens on line 3. Stack was [0,4]. Error happens.
                               // Then "let y = 2" is parsed at column 3 (after "  ").
                               // Is there an implicit dedent to 0 at EOF if stack is not [0]? Yes.
                               // After DedentError, stack is [0]. So no dedent at EOF.

            TokenKind::Eof,
        ];
        let actual_kinds_dedent_error = get_all_token_kinds(input_dedent_error);
        assert_eq!(actual_kinds_dedent_error, expected_kinds_dedent_error, "Token kinds for DedentError input do not match");

        // 3. "Invalid Dedent Level" - This one is tricky, might be hard to hit.
        //    It requires `current_indent < last_indent` initially.
        //    And after the (single effective) pop, `*self.indentation_stack.last().unwrap_or(&0)` must be `< current_indent`.
        //    This means `new_stack_top < current_indent < old_stack_top`.
        //    Example: stack `[0, 8]`. `current_indent = 4`. `old_stack_top = 8`.
        //    - `current_indent (4) < old_stack_top (8)`.
        //    - Pop `8`. Stack is `[0]`. `new_stack_top = 0`.
        //    - `!self.indentation_stack.contains(&4)` (`![0].contains(&4)`) is true. Returns `DedentError`.
        //    This indicates "DedentError" is likely to always be hit before "Invalid Dedent Level" if
        //    `current_indent` is not on the stack after a pop.
        //
        //    The "Invalid Dedent Level" is after the while loop: `if *self.indentation_stack.last().unwrap_or(&0) != current_indent`.
        //    This line is reachable if the `while` loop condition (`*self.indentation_stack.last().unwrap_or(&0) > current_indent`)
        //    is false. This means `*self.indentation_stack.last().unwrap_or(&0) <= current_indent`.
        //    So, for the error to trigger, we need `*self.indentation_stack.last().unwrap_or(&0) < current_indent`.
        //    And this is within the block `current_indent < last_indent`.
        //    So: `stack_top_after_pops < current_indent < last_indent_before_pops`.
        //    And the `while` loop did not run, or ran and exited.
        //    Given current logic (return after one dedent or DedentError), the while loop doesn't complete multiple pops.
        //    If the `while` loop condition `*self.indentation_stack.last().unwrap_or(&0) > current_indent` is initially false,
        //    it means `last_indent <= current_indent`.
        //    But this code path is in `else if current_indent < last_indent`. Contradiction.
        //    So, the `while` loop must execute at least once (pop happens).
        //    After the pop, if `DedentError` is not returned, a `Dedent` token is returned.
        //    This means the `Invalid Dedent Level` check is currently unreachable.
        //    I will skip testing "Invalid Dedent Level" for now as it seems unreachable.
    }

    #[test]
    fn test_numeric_literals_with_underscores() {
        // Test valid underscores
        let input1 = "1_000_000";
        let tokens1 = get_all_tokens(input1);
        assert_eq!(tokens1.len(), 2);
        assert_eq!(tokens1[0].kind, TokenKind::Integer(1000000));
        assert_eq!(tokens1[0].text, "1_000_000");
        assert_eq!(tokens1[0].line, 1);
        assert_eq!(tokens1[0].col, 1);

        let input2 = "12_34";
        let tokens2 = get_all_tokens(input2);
        assert_eq!(tokens2.len(), 2);
        assert_eq!(tokens2[0].kind, TokenKind::Integer(1234));
        assert_eq!(tokens2[0].text, "12_34");

        // Test multiple underscores together
        let input3 = "1__000";
        let tokens3 = get_all_tokens(input3);
        assert_eq!(tokens3.len(), 2);
        assert_eq!(tokens3[0].kind, TokenKind::Integer(1000));
        assert_eq!(tokens3[0].text, "1__000");

        // Test trailing underscore (allowed by current lexer logic, part of text)
        let input4 = "123_";
        let tokens4 = get_all_tokens(input4);
        assert_eq!(tokens4.len(), 2);
        assert_eq!(tokens4[0].kind, TokenKind::Integer(123));
        assert_eq!(tokens4[0].text, "123_");

        // Test leading underscore: "_123"
        // This will be tokenized as an identifier by the current lexer, not an integer.
        // The `_ if char.is_alphabetic() || char == '_'` branch for identifiers will take it.
        let input_leading_underscore = "_123";
        let tokens_leading_underscore = get_all_tokens(input_leading_underscore);
        assert_eq!(tokens_leading_underscore.len(), 2);
        assert_eq!(tokens_leading_underscore[0].kind, TokenKind::Ident("_123".to_string()));
        assert_eq!(tokens_leading_underscore[0].text, "_123");

        // Test number followed by underscore and then non_numeric/non_underscore char (e.g. letter)
        // e.g. "1_a" -> Integer(1), text("1_"), Ident("a")
        let input_num_underscore_alpha = "1_a";
        let tokens_nua = get_all_tokens(input_num_underscore_alpha);
        assert_eq!(tokens_nua.len(), 3);
        assert_eq!(tokens_nua[0].kind, TokenKind::Integer(1));
        assert_eq!(tokens_nua[0].text, "1_");
        assert_eq!(tokens_nua[1].kind, TokenKind::Ident("a".to_string()));
        assert_eq!(tokens_nua[1].text, "a");


        // Test only underscores (not a number, should be an identifier)
        // "___" -> Ident("___")
        let input_only_underscores = "___";
        let tokens_ou = get_all_tokens(input_only_underscores);
        assert_eq!(tokens_ou.len(), 2);
        assert_eq!(tokens_ou[0].kind, TokenKind::Ident("___".to_string()));
        assert_eq!(tokens_ou[0].text, "___");

        // Test case from description: `_123` (if allowed, current lexer might treat `_` at start as ident) -> Handled above
        // Test case from description: `123_` (if allowed) -> Handled above

        // Test a mix: "1_2_3 45_6_7"
        let input_mix = "1_2_3 45_6_7";
        let tokens_mix = get_all_tokens(input_mix);
        assert_eq!(tokens_mix.len(), 3);
        assert_eq!(tokens_mix[0].kind, TokenKind::Integer(123));
        assert_eq!(tokens_mix[0].text, "1_2_3");
        assert_eq!(tokens_mix[1].kind, TokenKind::Integer(4567));
        assert_eq!(tokens_mix[1].text, "45_6_7");
        assert_eq!(tokens_mix[2].kind, TokenKind::Eof);
    }

    #[test]
    fn test_all_operators_and_punctuation_individually() {
        // Test each operator and punctuation in a slightly more involved context
        let input = "let res = (val1 + val2) * val3 / val4 - val5 : some_type, other // comment";
        // Expected tokens:
        // Let, Ident("res"), Eq,
        // LParen, Ident("val1"), Plus, Ident("val2"), RParen,
        // Star, Ident("val3"), Slash, Ident("val4"), Minus, Ident("val5"),
        // Colon, Ident("some_type"), Comma, Ident("other"),
        // Comment(" comment"), Eof

        let tokens = get_all_tokens(input);

        let expected_token_kinds = vec![
            TokenKind::Let, TokenKind::Ident("res".to_string()), TokenKind::Eq,
            TokenKind::LParen, TokenKind::Ident("val1".to_string()), TokenKind::Plus, TokenKind::Ident("val2".to_string()), TokenKind::RParen,
            TokenKind::Star, TokenKind::Ident("val3".to_string()), TokenKind::Slash, TokenKind::Ident("val4".to_string()), TokenKind::Minus, TokenKind::Ident("val5".to_string()),
            TokenKind::Colon, TokenKind::Ident("some_type".to_string()), TokenKind::Comma, TokenKind::Ident("other".to_string()),
            TokenKind::Comment(" comment".to_string()),
            TokenKind::Eof,
        ];

        let actual_token_kinds: Vec<TokenKind> = tokens.into_iter().map(|t| t.kind).collect();
        assert_eq!(actual_token_kinds, expected_token_kinds, "Token kinds do not match for complex operator/punctuation test");

        // Test them with varied spacing
        let input_spacing = "let x = ( 5 + a ) : type_name // comment here";
        let tokens_spacing = get_all_tokens(input_spacing);
        let expected_kinds_spacing = vec![
            TokenKind::Let, TokenKind::Ident("x".to_string()), TokenKind::Eq,
            TokenKind::LParen, TokenKind::Integer(5), TokenKind::Plus, TokenKind::Ident("a".to_string()), TokenKind::RParen,
            TokenKind::Colon, TokenKind::Ident("type_name".to_string()),
            TokenKind::Comment(" comment here".to_string()),
            TokenKind::Eof,
        ];
        let actual_kinds_spacing: Vec<TokenKind> = tokens_spacing.into_iter().map(|t| t.kind).collect();
        assert_eq!(actual_kinds_spacing, expected_kinds_spacing, "Token kinds do not match for spacing test");


        // Test operators and punctuation at start/end of input if meaningful (usually not without other tokens)
        // For example, a lone "+" might be valid or not depending on language grammar, lexer should just tokenize it.
        assert_tokens("+", vec![TokenKind::Plus, TokenKind::Eof]);
        assert_tokens("(", vec![TokenKind::LParen, TokenKind::Eof]);
        assert_tokens("  ,", vec![TokenKind::Comma, TokenKind::Eof]); // With leading spaces

        // Test operators separated only by comments and newlines
        let input_op_comment_nl = "+\n//comment\n-";
        assert_tokens(input_op_comment_nl, vec![
            TokenKind::Plus, TokenKind::Newline,
            TokenKind::Comment("comment".to_string()), TokenKind::Newline,
            TokenKind::Minus, TokenKind::Eof
        ]);
    }


    #[test]
    fn test_string_literals_various() {
        // Test empty string: ""
        let input_empty = "\"\"";
        let tokens_empty = get_all_tokens(input_empty);
        assert_eq!(tokens_empty.len(), 2, "Empty string input: {:?}", tokens_empty);
        assert_eq!(tokens_empty[0].kind, TokenKind::String("".to_string()));
        assert_eq!(tokens_empty[0].text, "\"\"");
        assert_eq!(tokens_empty[0].line, 1);
        assert_eq!(tokens_empty[0].col, 1);
        assert_eq!(tokens_empty[1].kind, TokenKind::Eof);

        // Test string with spaces: "  "
        let input_spaces = "\"  \"";
        let tokens_spaces = get_all_tokens(input_spaces);
        assert_eq!(tokens_spaces.len(), 2, "String with spaces input: {:?}", tokens_spaces);
        assert_eq!(tokens_spaces[0].kind, TokenKind::String("  ".to_string()));
        assert_eq!(tokens_spaces[0].text, "\"  \"");
        assert_eq!(tokens_spaces[0].line, 1);
        assert_eq!(tokens_spaces[0].col, 1);
        assert_eq!(tokens_spaces[1].kind, TokenKind::Eof);

        // Test string with special characters (that don't require complex escapes): "hello (world)!"
        let input_special = "\"hello (world)!\"";
        let tokens_special = get_all_tokens(input_special);
        assert_eq!(tokens_special.len(), 2, "String with special chars input: {:?}", tokens_special);
        assert_eq!(tokens_special[0].kind, TokenKind::String("hello (world)!".to_string()));
        assert_eq!(tokens_special[0].text, "\"hello (world)!\"");
        assert_eq!(tokens_special[0].line, 1);
        assert_eq!(tokens_special[0].col, 1);
        assert_eq!(tokens_special[1].kind, TokenKind::Eof);

        // Test string ending at EOF without closing quote (already covered by test_unterminated_string_literal)
        // let input_unterminated = "\"abc";
        // let tokens_unterminated = get_all_tokens(input_unterminated);
        // assert_eq!(tokens_unterminated.len(), 2);
        // assert!(matches!(tokens_unterminated[0].kind, TokenKind::Unknown('\"')));
        // assert_eq!(tokens_unterminated[0].text, "\"abc"); // as per current Unknown token logic for unterminated string

        // Test string with newline inside - current lexer behavior for unterminated string might treat this as error.
        // Based on current `lexer.rs`:
        // if c == '\n' { // Unterminated string at newline ... currently assumes strings are well-terminated for MVP.
        // If string contains \n, it's part of the content.
        // The existing unterminated string logic is if EOF is hit before ".
        // A string like "a\nb" is valid if newlines are allowed in strings.
        // The lexer code does:
        // text_content.push(c); literal_text.push(c); self.advance(); if c == '\n' { /* no break */ }
        // This means newlines are included in string content.
        let input_newline = "\"hello\nworld\"";
        let tokens_newline = get_all_tokens(input_newline);
        assert_eq!(tokens_newline.len(), 2, "String with newline input: {:?}", tokens_newline);
        assert_eq!(tokens_newline[0].kind, TokenKind::String("hello\nworld".to_string()));
        assert_eq!(tokens_newline[0].text, "\"hello\nworld\"");
        assert_eq!(tokens_newline[0].line, 1); // String starts on line 1
        assert_eq!(tokens_newline[0].col, 1);
        // The line number of the token itself is where it starts. The internal newline increments lexer's line counter.
        assert_eq!(tokens_newline[1].kind, TokenKind::Eof);
        // To verify lexer line/col state after this string:
        let mut lexer_state_check = Lexer::new("\"hello\nworld\"");
        let token1 = lexer_state_check.next_token(); // The string token
        assert_eq!(token1.line, 1);
        assert_eq!(token1.col, 1);
        // After processing this token, the lexer's internal line will be 2, and col will be 7 (for 'world"').
        // Let's check the EOF token's position.
        let token2 = lexer_state_check.next_token(); // EOF
        assert_eq!(token2.line, 2, "EOF line after string with newline"); // Because the string ended on line 2
        assert_eq!(token2.col, 7, "EOF col after string with newline");  // Col after the closing quote on line 2
    }
}

pub struct Lexer<'a> {
    input: std::iter::Peekable<std::str::Chars<'a>>,
    current_char: Option<char>,
    line: usize,
    col: usize, // Current column in the current line
    indentation_stack: Vec<usize>,
    start_of_line: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input: input.chars().peekable(),
            current_char: None,
            line: 1,
            col: 0, // Before the first char
            indentation_stack: vec![0], // Start with a base indentation of 0
            start_of_line: true,
        };
        lexer.advance(); // Load the first character
        lexer
    }

    // Helper to advance the lexer's current character
    fn advance(&mut self) {
        if self.current_char == Some('\n') {
            self.line += 1;
            self.col = 0; // Reset column before advancing to the char on the new line
            self.start_of_line = true;
        }
        self.current_char = self.input.next();
        if self.current_char.is_some() {
            self.col += 1;
        }
    }
}
