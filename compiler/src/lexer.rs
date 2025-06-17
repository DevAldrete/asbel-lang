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

    // Helper to collect all TokenKinds from the lexer
    fn get_all_token_kinds(input: &str) -> Vec<TokenKind> {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        loop {
            let token = lexer.next_token();
            tokens.push(token.kind.clone()); // Clone kind before token is dropped or its kind is Eof
            if token.kind == TokenKind::Eof {
                break;
            }
        }
        tokens
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
