use crate::ast::*;
use crate::lexer::{Lexer, Token, TokenKind};
use crate::symbol_table::{SymbolTable, SymbolKind};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
    symbol_table: SymbolTable,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        // Initialize current_token and peek_token by calling next_token() twice.
        // We need placeholder tokens for initialization that are distinct from any valid token.
        // Or, better, initialize them properly.
        // The lexer's new() method already loads the first char, but next_token() hasn't been called.

        // To handle EOF correctly from the start if the input is empty:
        let first_token = lexer.next_token();
        let second_token = lexer.next_token();

        let mut symbol_table = SymbolTable::new();
        // Pre-populate built-in functions
        // Panicking here is acceptable as this is a setup issue for the compiler itself.
        symbol_table.define("print".to_string(), SymbolKind::BuiltInFunction)
            .expect("Failed to define built-in 'print' function. This should not happen.");

        Parser {
            lexer,
            current_token: first_token,
            peek_token: second_token,
            errors: Vec::new(),
            symbol_table, // Use the initialized symbol_table
        }
    }

    // Helper to advance tokens
    fn next_token_internal(&mut self) {
        self.current_token = self.peek_token.clone(); // clone might be expensive, consider alternatives if profiling shows issues
        self.peek_token = self.lexer.next_token();
    }

    // Example of a helper for expecting a certain token kind
    #[allow(dead_code)] // Will be used soon
    fn expect_peek(&mut self, expected_kind: TokenKind) -> bool {
        if self.peek_token.kind == expected_kind {
            self.next_token_internal();
            true
        } else {
            self.peek_error(&expected_kind);
            false
        }
    }

    #[allow(dead_code)] // Will be used soon
    fn peek_error(&mut self, expected_kind: &TokenKind) {
        let msg = format!(
            "Expected next token to be {:?}, got {:?} instead. Line: {}, Col: {}",
            expected_kind, self.peek_token.kind, self.peek_token.line, self.peek_token.col
        );
        self.errors.push(msg);
    }

    #[allow(dead_code)] // Will be used soon
    fn current_token_is(&self, kind: &TokenKind) -> bool {
        self.current_token.kind == *kind
    }

    #[allow(dead_code)] // Will be used soon
    fn peek_token_is(&self, kind: &TokenKind) -> bool {
        self.peek_token.kind == *kind
    }

}

// Precedence for Pratt parser
#[derive(Debug, PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    Equals,      // == (not used for MVP assignment, but for general precedence)
    LessGreater, // > or < (not in MVP)
    Sum,         // + -
    Product,     // * /
    Prefix,      // -X or !X
    Call,        // myFunction(X)
    Index,       // array[index] (not in MVP)
}

impl<'a> Parser<'a> {
    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { body: Vec::new() };

        while self.current_token.kind != TokenKind::Eof {
            // Skip empty newlines and comments between statements
            while self.current_token_is(&TokenKind::Newline) || matches!(self.current_token.kind, TokenKind::Comment(_)) {
                self.next_token_internal();
            }
            if self.current_token.kind == TokenKind::Eof { // Check again after skipping
                break;
            }

            match self.parse_statement() {
                Some(statement) => program.body.push(statement),
                None => {
                    // Error recovery: advance to the next potential statement start (e.g., newline)
                    // For now, just advance one token to avoid infinite loops on errors.
                    // A more robust recovery would skip until a known statement boundary.
                    // self.next_token_internal(); // This might skip too much or too little.
                    // If parse_statement returned None, it should have logged an error.
                    // For now, we'll rely on parse_statement advancing tokens on error.
                    // If parse_statement does NOT advance on error, we MUST advance here.
                    // Let's assume parse_statement advances past the problematic token(s).
                    // If it doesn't, an infinite loop could occur if current_token remains unchanged on error.
                }
            }
            // After a statement, we might expect a newline or EOF
            // parse_statement should consume its terminating newline.
            // If not, we might need to consume it here or ensure it's handled.
        }
        program
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.kind {
            TokenKind::Let => self.parse_let_statement(),
            TokenKind::Fn => self.parse_function_declaration(), // To be implemented
            // TokenKind::Return => self.parse_return_statement(), // If we add explicit returns
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        // Expect Let IDENT Eq Expression Newline
        // current_token is Let
        if !self.expect_peek(TokenKind::Ident("".into())) { // Expect Ident next, content doesn't matter for kind check
            return None;
        }

        let name = match &self.current_token.kind {
            TokenKind::Ident(id_name) => id_name.clone(),
            _ => {
                // Already reported by expect_peek or should be an internal error
                // self.errors.push(format!("Expected identifier after let, got {:?}", self.current_token.kind));
                return None;
            }
        };

        if !self.expect_peek(TokenKind::Eq) {
            return None;
        }
        self.next_token_internal(); // Consume Eq, advance to expression start

        let value = match self.parse_expression(Precedence::Lowest) {
            Some(expr) => expr,
            None => {
                self.errors.push(format!("Expected expression after '=' in let statement for '{}'. Line: {}, Col: {}", name, self.current_token.line, self.current_token.col));
                return None;
            }
        };

        // Expect a newline to terminate the let statement
        if self.peek_token_is(&TokenKind::Newline) {
            self.next_token_internal(); // Consume the expression's last token
            self.next_token_internal(); // Consume the Newline
        } else if self.peek_token_is(&TokenKind::Eof) {
             self.next_token_internal(); // Consume the expression's last token (EOF will be handled by program loop)
        }
        else {
            // Error: Expected newline after let statement expression
            self.errors.push(format!(
                "Expected newline after expression in let statement, got {:?}. Line: {}, Col: {}",
                self.peek_token.kind, self.peek_token.line, self.peek_token.col
            ));
            // We don't return None here, as the core let statement is parsed.
            // This is more of a syntax convention.
            // However, for strictness, one might return None.
            // Let's try to consume the current expression's last token and see what happens.
            self.next_token_internal();
        }

        // Define the variable in the symbol table
        if let Err(e) = self.symbol_table.define(name.clone(), SymbolKind::Variable) {
            self.errors.push(e);
            // We might choose to return None here if a symbol error is critical
            // For now, we'll record the error and return the statement.
        }

        Some(Statement::Let { name, value })
    }

    fn parse_function_declaration(&mut self) -> Option<Statement> {
        // current_token is Fn
        if !self.expect_peek(TokenKind::Ident("".into())) { // Expect Ident (function name)
            return None;
        }
        let name = match &self.current_token.kind {
            TokenKind::Ident(id_name) => id_name.clone(),
            _ => return None, // Error already logged by expect_peek
        };

        if !self.expect_peek(TokenKind::LParen) { // Expect LParen
            return None;
        }
        // current_token is now LParen
        // The next_token_internal() to move past LParen will be handled by parse_parameters or its first check.

        let params = match self.parse_parameters() {
            Some(p) => p,
            None => return None, // Error in parameter parsing
        };
        // parse_parameters consumes up to and including the RParen

        let return_type = if self.current_token_is(&TokenKind::Colon) { // current_token is now Colon if present, because parse_params consumed RParen
            self.next_token_internal(); // Consume Colon (representing '->')

            // Check if next token is an identifier for the type
            if let TokenKind::Ident(type_name) = &self.current_token.kind {
                let type_ident = type_name.clone();
                self.next_token_internal(); // Consume type identifier
                Some(type_ident)
            } else {
                self.errors.push(format!("Expected return type identifier after ':', got {:?}. Line: {}, Col: {}", self.current_token.kind, self.current_token.line, self.current_token.col));
                return None;
            }
        } else {
            None
        };

        // After function signature (name, params, optional return type), expect a Newline then Indent for block
        // current_token is now at the token that followed the signature (e.g., Newline, or whatever is next if no return type)

        if !self.current_token_is(&TokenKind::Newline) {
             self.errors.push(format!("Expected newline before function body block, got {:?}. Line {}, Col {}", self.current_token.kind, self.current_token.line, self.current_token.col));
            return None;
        }
        // current_token is Newline

        if !self.expect_peek(TokenKind::Indent) { // Consumes Newline, current_token is Newline, expects Indent. After this, current_token is Indent.
             self.errors.push(format!("Expected indent for function body, got {:?}. Line {}, Col {}", self.peek_token.kind, self.peek_token.line, self.peek_token.col)); // peek_token because expect_peek checks peek
            return None;
        }
        // current_token is Indent

        let body = match self.parse_block_statement() { // parse_block_statement expects current_token to be Indent
            Some(b) => b,
            None => return None, // Error in block parsing
        };

        // Define the function in the symbol table
        // Note: Parameters are not added to this global symbol table in this MVP step.
        // A more advanced implementation would handle scopes for parameters.
        if let Err(e) = self.symbol_table.define(name.clone(), SymbolKind::Function) {
            self.errors.push(e);
            // Similar to let, decide if this error should prevent returning the statement.
            // For now, record error and return statement.
        }

        Some(Statement::FunctionDeclaration { name, params, return_type, body })
    }

    fn parse_parameters(&mut self) -> Option<Vec<Parameter>> {
        // Assumes current_token is LParen upon entry by parse_function_declaration calling expect_peek(LParen)
        // then parse_parameters. So current_token is LParen.
        let mut params = Vec::new();

        if self.peek_token_is(&TokenKind::RParen) {
            self.next_token_internal(); // Consume LParen
            self.next_token_internal(); // Consume RParen
            return Some(params);
        }

        self.next_token_internal(); // Consume LParen, current_token is now first param's Ident if any

        while !self.current_token_is(&TokenKind::RParen) && !self.current_token_is(&TokenKind::Eof) {
            let param_name = match &self.current_token.kind {
                TokenKind::Ident(name) => name.clone(),
                _ => {
                    self.errors.push(format!("Expected identifier for parameter name, got {:?}. Line {}, Col {}", self.current_token.kind, self.current_token.line, self.current_token.col));
                    return None;
                }
            };
            self.next_token_internal(); // Consume param name Ident

            let type_ann = if self.current_token_is(&TokenKind::Colon) {
                self.next_token_internal(); // Consume Colon
                match &self.current_token.kind {
                    TokenKind::Ident(type_name) => {
                        let type_ident = type_name.clone();
                        self.next_token_internal(); // Consume type Ident
                        Some(type_ident)
                    }
                    _ => {
                        self.errors.push(format!("Expected type identifier after ':' in parameter, got {:?}. Line {}, Col {}", self.current_token.kind, self.current_token.line, self.current_token.col));
                        return None;
                    }
                }
            } else {
                None
            };
            params.push(Parameter { name: param_name, type_ann });

            if self.current_token_is(&TokenKind::Comma) {
                self.next_token_internal(); // Consume Comma
                if self.current_token_is(&TokenKind::RParen) { // Trailing comma before RParen
                    self.errors.push(format!("Trailing comma in parameter list not allowed. Line {}, Col {}", self.current_token.line, self.current_token.col));
                    return None;
                }
                 if self.current_token_is(&TokenKind::Eof) { // Comma then EOF
                    self.errors.push(format!("Unexpected EOF after comma in parameter list. Line {}, Col {}", self.current_token.line, self.current_token.col));
                    return None;
                }
            } else if !self.current_token_is(&TokenKind::RParen) {
                self.errors.push(format!("Expected comma or ')' after parameter, got {:?}. Line {}, Col {}", self.current_token.kind, self.current_token.line, self.current_token.col));
                return None;
            }
        }

        if !self.current_token_is(&TokenKind::RParen) {
            self.errors.push(format!("Expected ')' to close parameter list, got {:?}. Line {}, Col {}", self.current_token.kind, self.current_token.line, self.current_token.col));
            return None;
        }
        self.next_token_internal(); // Consume RParen. current_token is now RParen.
                                    // The caller (parse_function_declaration) will advance past this.
        Some(params)
    }

    fn parse_block_statement(&mut self) -> Option<BlockStatement> {
        // Assumes current_token is Indent upon entry.
        let mut statements = Vec::new();
        self.next_token_internal(); // Consume Indent, current_token is now the first token of the first statement in block

        while !self.current_token_is(&TokenKind::Dedent) && !self.current_token_is(&TokenKind::Eof) {
            // Skip any leading newlines or comments within the block
            while self.current_token_is(&TokenKind::Newline) || matches!(self.current_token.kind, TokenKind::Comment(_)) {
                self.next_token_internal();
            }
            // Check again after skipping
            if self.current_token_is(&TokenKind::Dedent) || self.current_token_is(&TokenKind::Eof) {
                break;
            }

            match self.parse_statement() {
                Some(stmt) => statements.push(stmt),
                None => {
                    // Error in parsing statement within block.
                    // parse_statement should have logged an error and advanced tokens to prevent loops.
                    // We return None for the block because it's malformed.
                    return None;
                }
            }
        }

        if !self.current_token_is(&TokenKind::Dedent) {
            self.errors.push(format!("Expected Dedent to end block, got {:?}. Line {}, Col {}", self.current_token.kind, self.current_token.line, self.current_token.col));
            return None;
        }
        self.next_token_internal(); // Consume Dedent. current_token is now Dedent.
                                    // The caller (e.g. parse_program or parse_fn_decl) will advance past this.
                                    // Or rather, the next call to parse_statement will see Dedent and handle it, or main loop.
                                    // For function blocks, the fn parsing logic is done after this.
                                    // For other blocks (if any), similar logic.
        Some(BlockStatement { statements })
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expression = match self.parse_expression(Precedence::Lowest) {
            Some(expr) => expr,
            None => {
                // error already logged by parse_expression or its children
                // self.errors.push(format!("Failed to parse expression statement. Line: {}, Col: {}", self.current_token.line, self.current_token.col));
                return None;
            }
        };

        // Expression statements must be followed by a newline or EOF
        if self.peek_token_is(&TokenKind::Newline) {
            self.next_token_internal(); // Consume expression's last token
            self.next_token_internal(); // Consume newline
        } else if self.peek_token_is(&TokenKind::Eof) {
            self.next_token_internal(); // Consume expression's last token
        } else {
            self.errors.push(format!(
                "Expected newline after expression statement, got {:?}. Line: {}, Col: {}",
                self.peek_token.kind, self.peek_token.line, self.peek_token.col
            ));
            // Attempt to recover by consuming the current token that formed the expression.
            // This might not be enough if the expression parsing didn't consume appropriately.
            self.next_token_internal();
            // Return None because the statement is malformed.
            return None;
        }
        Some(Statement::ExpressionStatement { expression })
    }

    // Pratt parser: parse_expression
    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        // Determine the prefix parsing function based on the current token
        // and parse the prefix expression.
        let mut left_opt = match self.current_token.kind {
            TokenKind::Ident(_) => Self::parse_identifier(self),
            TokenKind::Integer(_) => Self::parse_integer_literal(self),
            TokenKind::String(_) => Self::parse_string_literal(self),
            TokenKind::LParen => Self::parse_grouped_expression(self),
            // TODO: Add prefix operators like Minus (e.g., -5)
            _ => {
                self.errors.push(format!(
                    "No prefix parse function for token {:?} at Line {}, Col {}",
                    self.current_token.kind, self.current_token.line, self.current_token.col
                ));
                return None;
            }
        };

        // Loop for infix expressions as long as the next token has higher precedence.
        while !self.peek_token_is(&TokenKind::Newline) && precedence < self.peek_precedence() {
            // Check if the peek token is an infix operator we handle for expressions.
            match self.peek_token.kind {
                TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash => {
                    self.next_token_internal(); // Consume the prefix expression's last token.
                                             // current_token is now the infix operator.
                    left_opt = Self::parse_infix_expression(self, left_opt?); // Pass self and the left expression.
                                                                          // parse_infix_expression consumes the operator
                                                                          // and the right operand.
                    if left_opt.is_none() { return None; } // Error in infix parsing, already logged.
                }
                // TODO: Add TokenKind::LParen for call expressions here later
                _ => {
                    // Not an infix operator we handle for expressions, or lower precedence.
                    return left_opt;
                }
            }
        }

        left_opt // Return the expression parsed so far.
    }

    fn get_token_precedence(token_kind: &TokenKind) -> Precedence {
        match token_kind {
            TokenKind::Plus | TokenKind::Minus => Precedence::Sum,
            TokenKind::Star | TokenKind::Slash => Precedence::Product,
            TokenKind::LParen => Precedence::Call, // For function calls like identifier(
            _ => Precedence::Lowest,
        }
    }

    #[allow(dead_code)] // Will be used soon
    fn current_precedence(&self) -> Precedence {
        Self::get_token_precedence(&self.current_token.kind)
    }

    fn peek_precedence(&self) -> Precedence {
        Self::get_token_precedence(&self.peek_token.kind)
    }

    // Prefix parsing functions
    fn parse_identifier(parser: &mut Parser<'a>) -> Option<Expression> {
        let name = match &parser.current_token.kind {
            TokenKind::Ident(id_name) => id_name.clone(),
            _ => {
                // This case should ideally not be reached if dispatch is correct.
                parser.errors.push(format!("Expected Identifier, got {:?}", parser.current_token.kind));
                return None;
            }
        };
        parser.next_token_internal(); // Consume the identifier token
        Some(Expression::Identifier(name))
    }

    fn parse_integer_literal(parser: &mut Parser<'a>) -> Option<Expression> {
        let value = match parser.current_token.kind {
            TokenKind::Integer(val) => val,
            _ => {
                parser.errors.push(format!("Expected Integer literal, got {:?}", parser.current_token.kind));
                return None;
            }
        };
        parser.next_token_internal(); // Consume the integer literal token
        Some(Expression::LiteralInteger(value))
    }

    fn parse_string_literal(parser: &mut Parser<'a>) -> Option<Expression> {
        let value = match &parser.current_token.kind {
            TokenKind::String(val) => val.clone(),
            _ => {
                parser.errors.push(format!("Expected String literal, got {:?}", parser.current_token.kind));
                return None;
            }
        };
        parser.next_token_internal(); // Consume the string literal token
        Some(Expression::LiteralString(value))
    }

    fn parse_grouped_expression(parser: &mut Parser<'a>) -> Option<Expression> {
        // current_token is LParen when called
        parser.next_token_internal(); // Consume LParen. current_token is now the start of the inner expression.

        let expression = parser.parse_expression(Precedence::Lowest);
        // parse_expression for the inner content will consume its tokens.
        // current_token will be the last token of that inner expression.

        // We now expect peek_token to be RParen.
        if !parser.expect_peek(TokenKind::RParen) {
            // expect_peek logs an error if peek_token is not RParen.
            // it also calls next_token_internal, so current_token becomes what peek_token was.
            return None;
        }
        // If expect_peek was true, current_token is now RParen.
        parser.next_token_internal(); // Consume the RParen.

        expression.map(|e| Expression::GroupedExpression(Box::new(e)))
    }

    fn parse_call_expression(parser: &mut Parser<'a>, function: Expression) -> Option<Expression> {
        // current_token is LParen when this is called by parse_expression.
        // LParen has already been consumed by the loop in parse_expression before calling this.
        // So, current_token is LParen.

        let arguments = match parser.parse_expression_list(TokenKind::RParen) {
            Some(args) => args,
            None => {
                // Error already logged by parse_expression_list
                return None;
            }
        };
        // parse_expression_list consumes tokens up to and including the RParen.
        // current_token is now RParen.
        parser.next_token_internal(); // Consume the RParen to move to the token after the call expression.

        Some(Expression::FunctionCall {
            function: Box::new(function),
            arguments,
        })
    }

    // Parses a list of expressions, e.g., arguments in a function call.
    // Expects current_token to be the token that starts the list (e.g. LParen for first call, or Comma).
    // Consumes tokens until (and including) the `end_token_kind`.
    fn parse_expression_list(&mut self, end_token_kind: TokenKind) -> Option<Vec<Expression>> {
        let mut list = Vec<Expression>::new();

        // Check for an empty list: e.g. my_func() where current_token is LParen and peek_token is RParen.
        if self.peek_token_is(&end_token_kind) {
            self.next_token_internal(); // Consume LParen (current_token)
            // self.next_token_internal(); // Consume RParen (peek_token) - This is done by the caller or expect_peek in caller
            return Some(list);
        }

        self.next_token_internal(); // Consume LParen (or Comma), current_token is now start of first/next expression.

        // Parse the first expression in the list.
        match self.parse_expression(Precedence::Lowest) {
            Some(expr) => list.push(expr),
            None => {
                 self.errors.push(format!("Expected expression in list. Line {}, Col {}", self.current_token.line, self.current_token.col));
                return None; // Error logged by parse_expression or here.
            }
        }
        // parse_expression consumes tokens for the expression it parsed.
        // current_token is now the last token of that parsed expression.

        // Parse subsequent expressions separated by commas.
        while self.peek_token_is(&TokenKind::Comma) {
            self.next_token_internal(); // Consume the last token of the previous expression.
            self.next_token_internal(); // Consume the Comma. current_token is now Comma.
                                     // current_token should be the start of the next expression.

            if self.current_token_is(&end_token_kind) { // Check for trailing comma: e.g. (arg1, )
                self.errors.push(format!("Trailing comma before {:?} in expression list. Line {}, Col {}", end_token_kind, self.current_token.line, self.current_token.col));
                return None;
            }

            // Now parse the next expression.
            match self.parse_expression(Precedence::Lowest) {
                Some(expr) => list.push(expr),
                None => {
                    self.errors.push(format!("Expected expression after comma in list. Line {}, Col {}", self.current_token.line, self.current_token.col));
                    return None; // Error logged by parse_expression or here.
                }
            }
        }

        // After all expressions (or the first one if no commas), expect the end token.
        if !self.expect_peek(end_token_kind.clone()) {
             self.errors.push(format!("Expected {:?} to end expression list, but got {:?}. Line {}, Col {}", end_token_kind, self.peek_token.kind, self.peek_token.line, self.peek_token.col));
            return None; // Error logged by expect_peek.
        }
        // expect_peek consumes the end_token. current_token is now the end_token.

        Some(list)
    }

    fn parse_infix_expression(parser: &mut Parser<'a>, left: Expression) -> Option<Expression> {
        // current_token is the operator token when this is called by parse_expression.
        let operator = match parser.current_token.kind {
            TokenKind::Plus => InfixOperator::Plus,
            TokenKind::Minus => InfixOperator::Minus,
            TokenKind::Star => InfixOperator::Star,
            TokenKind::Slash => InfixOperator::Slash,
            _ => {
                // This case should not be reached if called correctly.
                parser.errors.push(format!(
                    "Token {:?} is not a valid infix operator. Line {}, Col {}",
                    parser.current_token.kind, parser.current_token.line, parser.current_token.col
                ));
                return None;
            }
        };

        let op_precedence = parser.current_precedence(); // Get precedence of the current operator.
        parser.next_token_internal(); // Consume the operator. current_token is now the start of the right-hand expression.

        // Parse the right-hand side of the expression, using the operator's precedence.
        // This ensures correct associativity (e.g. for left-associative ops).
        match parser.parse_expression(op_precedence) {
            Some(right) => {
                // parse_expression (for right) has consumed its tokens.
                // current_token is the last token of the right expression.
                Some(Expression::InfixExpression {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                })
            }
            None => {
                // Error parsing right-hand side, already logged by inner parse_expression.
                parser.errors.push(format!(
                    "Expected expression on the right side of {:?} operator. Line {}, Col {}",
                    operator, parser.current_token.line, parser.current_token.col // current_token is where parsing failed for RHS
                ));
                None
            }
        }
    }

    #[test]
    fn test_parameter_edge_cases() {
        // Valid cases
        let valid_cases = vec![
            ("fn noParams()\n  \n", "noParams", vec![]),
            ("fn oneParam(a)\n  \n", "oneParam", vec![Parameter { name: "a".to_string(), type_ann: None }]),
            ("fn typedParam(a: MyType)\n  \n", "typedParam", vec![Parameter { name: "a".to_string(), type_ann: Some("MyType".to_string()) }]),
            ("fn multipleParams(a: TypeA, b, c: TypeC)\n  \n", "multipleParams", vec![
                Parameter { name: "a".to_string(), type_ann: Some("TypeA".to_string()) },
                Parameter { name: "b".to_string(), type_ann: None },
                Parameter { name: "c".to_string(), type_ann: Some("TypeC".to_string()) },
            ]),
        ];

        for (input, expected_fn_name, expected_params) in valid_cases {
            let program = parse_input_to_program(input);
            assert_eq!(program.body.len(), 1, "Program body should have 1 statement for input: {}", input);
            match &program.body[0] {
                Statement::FunctionDeclaration { name, params, .. } => {
                    assert_eq!(name, expected_fn_name, "Function name mismatch for input: {}", input);
                    assert_eq!(params.len(), expected_params.len(), "Parameter count mismatch for input: {}", input);
                    for (i, expected_param) in expected_params.iter().enumerate() {
                        assert_eq!(&params[i], expected_param, "Parameter mismatch at index {} for input: {}", i, input);
                    }
                }
                _ => panic!("Expected FunctionDeclaration for input: {}", input),
            }
        }

        // Error case: Trailing comma in parameters
        let input_trailing_comma = "fn paramsWithTrailingCommaError(a, b,)\n  \n";
        let lexer_tc = Lexer::new(input_trailing_comma);
        let mut parser_tc = Parser::new(lexer_tc);
        parser_tc.parse_program();
        assert!(!parser_tc.errors.is_empty(), "Expected errors for trailing comma input: '{}'", input_trailing_comma);
        assert!(parser_tc.errors.iter().any(|e| e.contains("Trailing comma in parameter list not allowed")),
            "Specific error for trailing comma not found. Errors: {:?}", parser_tc.errors);

        // Error case: Missing parameter name after comma
        let input_missing_param_name = "fn missingParamName(a, :TypeB)\n \n";
        let lexer_mpn = Lexer::new(input_missing_param_name);
        let mut parser_mpn = Parser::new(lexer_mpn);
        parser_mpn.parse_program();
        assert!(!parser_mpn.errors.is_empty(), "Expected errors for missing param name: '{}'", input_missing_param_name);
        assert!(parser_mpn.errors.iter().any(|e| e.contains("Expected identifier for parameter name, got Colon instead")),
            "Specific error for missing param name not found. Errors: {:?}", parser_mpn.errors);

    }

    #[test]
    fn test_program_multiple_statements_and_comments() {
        let input = r#"
// Program start
let x = 10 // variable x

// Function definition
fn myFunc(p1)
  // Function body comment
  let y = p1 + x
  y // implicit return or expression statement in function body

let z = myFunc(5) // call function
// End of program
"#;
        // Expected AST structure:
        // Program:
        // 1. LetStatement { name: "x", value: LiteralInteger(10) }
        // 2. FunctionDeclaration { name: "myFunc", params: [Parameter("p1", None)], return_type: None,
        //      body: BlockStatement {
        //          1. LetStatement { name: "y", value: Infix(Ident("p1"), Plus, Ident("x")) }
        //          2. ExpressionStatement { expression: Ident("y") }
        //      }}
        // 3. LetStatement { name: "z", value: FunctionCall { function: Ident("myFunc"), args: [LiteralInteger(5)] } }

        let program = parse_input_to_program(input);

        assert_eq!(program.body.len(), 3, "Program should have 3 top-level statements. Found: {:?}", program.body);

        // Check statement 1: let x = 10
        match &program.body[0] {
            Statement::Let { name, value } => {
                assert_eq!(name, "x");
                assert_eq!(*value, Expression::LiteralInteger(10));
            }
            _ => panic!("Expected Let statement for x, got {:?}", program.body[0]),
        }

        // Check statement 2: fn myFunc(p1)...
        match &program.body[1] {
            Statement::FunctionDeclaration { name, params, return_type, body } => {
                assert_eq!(name, "myFunc");
                assert_eq!(params.len(), 1);
                assert_eq!(params[0], Parameter { name: "p1".to_string(), type_ann: None });
                assert!(return_type.is_none());

                assert_eq!(body.statements.len(), 2, "myFunc body should have 2 statements.");
                // Check myFunc body statement 1: let y = p1 + x
                match &body.statements[0] {
                    Statement::Let { name: name_y, value: value_y } => {
                        assert_eq!(name_y, "y");
                        if let Expression::InfixExpression { left, operator, right } = value_y {
                            assert_eq!(**left, Expression::Identifier("p1".to_string()));
                            assert_eq!(*operator, InfixOperator::Plus);
                            assert_eq!(**right, Expression::Identifier("x".to_string()));
                        } else {
                            panic!("Expected infix expression for let y value, got {:?}", value_y);
                        }
                    }
                    _ => panic!("Expected Let statement for y in myFunc, got {:?}", body.statements[0]),
                }
                // Check myFunc body statement 2: y
                match &body.statements[1] {
                    Statement::ExpressionStatement { expression } => {
                        assert_eq!(*expression, Expression::Identifier("y".to_string()));
                    }
                    _ => panic!("Expected ExpressionStatement for y in myFunc, got {:?}", body.statements[1]),
                }
            }
            _ => panic!("Expected FunctionDeclaration for myFunc, got {:?}", program.body[1]),
        }

        // Check statement 3: let z = myFunc(5)
        match &program.body[2] {
            Statement::Let { name, value } => {
                assert_eq!(name, "z");
                if let Expression::FunctionCall { function, arguments } = value {
                    assert_eq!(**function, Expression::Identifier("myFunc".to_string()));
                    assert_eq!(arguments.len(), 1);
                    assert_eq!(arguments[0], Expression::LiteralInteger(5));
                } else {
                    panic!("Expected FunctionCall for let z value, got {:?}", value);
                }
            }
            _ => panic!("Expected Let statement for z, got {:?}", program.body[2]),
        }
    }


    #[test]
    fn test_functions_empty_or_comment_only_body() {
        // Test 1: Function with an effectively empty body (just newline after indent)
        // Lexer: Fn, Ident, LParen, RParen, Newline, Indent, Newline, Dedent, Eof
        let input_empty = "fn emptyBody()\n  \n";
        let program_empty = parse_input_to_program(input_empty);
        assert_eq!(program_empty.body.len(), 1, "Empty body: program should have 1 statement.");
        match &program_empty.body[0] {
            Statement::FunctionDeclaration { name, body, .. } => {
                assert_eq!(name, "emptyBody");
                assert!(body.statements.is_empty(), "Empty body: function body should have 0 statements. Got: {:?}", body.statements);
            }
            _ => panic!("Expected FunctionDeclaration for empty body test."),
        }

        // Test 2: Function with only comments in the body
        // Lexer: Fn, Ident, LParen, RParen, Newline, Indent, Comment, Newline, Dedent, Eof
        // This test currently expects failure due to how parse_statement handles comments.
        let input_comment_only = "fn commentBody()\n  // only a comment\n  \n";
        let lexer_comment = Lexer::new(input_comment_only);
        let mut parser_comment = Parser::new(lexer_comment);
        let program_comment_result = parser_comment.parse_program();

        // Based on current parser logic, this should produce an error.
        // "No prefix parse function for token Comment..."
        if parser_comment.errors.is_empty() {
            // If no errors, it means the parser was modified to handle comments.
            // In that case, the body should be empty.
            assert_eq!(program_comment_result.body.len(), 1, "Comment only body (no error): program should have 1 statement.");
            match &program_comment_result.body[0] {
                Statement::FunctionDeclaration { name, body, .. } => {
                    assert_eq!(name, "commentBody");
                    assert!(body.statements.is_empty(), "Comment only body (no error): function body should have 0 statements. Got: {:?}", body.statements);
                }
                _ => panic!("Expected FunctionDeclaration for comment only body test (no error case)."),
            }
            println!("Successfully parsed comment-only body (parser likely modified). Errors: {:?}", parser_comment.errors);
        } else {
            // This is the path if the parser has NOT been modified yet.
            println!("Detected errors for comment-only body (as expected for unmodified parser): {:?}", parser_comment.errors);
            assert!(parser_comment.errors.iter().any(|e| e.contains("No prefix parse function for token Comment")),
                "Expected error 'No prefix parse function for token Comment' not found. Errors: {:?}", parser_comment.errors);
            // The function itself might not be added to the program body if parse_block_statement returns None.
            // So, program_comment_result.body might be empty or the FunctionDeclaration might be incomplete.
        }

        // Test 3: Function with multiple comments and blank lines
        let input_multi_comment = "fn multiCommentBody()\n  // comment 1\n\n  // comment 2\n  \n";
        let lexer_multi_comment = Lexer::new(input_multi_comment);
        let mut parser_multi_comment = Parser::new(lexer_multi_comment);
        let program_multi_comment_result = parser_multi_comment.parse_program();
        if parser_multi_comment.errors.is_empty() {
             assert_eq!(program_multi_comment_result.body.len(), 1, "Multi comment body (no error): program should have 1 statement.");
            match &program_multi_comment_result.body[0] {
                Statement::FunctionDeclaration { name, body, .. } => {
                    assert_eq!(name, "multiCommentBody");
                    assert!(body.statements.is_empty(), "Multi comment body (no error): function body should have 0 statements. Got: {:?}", body.statements);
                }
                _ => panic!("Expected FunctionDeclaration for multi comment body test (no error case)."),
            }
            println!("Successfully parsed multi-comment body (parser likely modified). Errors: {:?}", parser_multi_comment.errors);
        } else {
            println!("Detected errors for multi-comment body (as expected for unmodified parser): {:?}", parser_multi_comment.errors);
            assert!(parser_multi_comment.errors.iter().any(|e| e.contains("No prefix parse function for token Comment")),
                "Expected error 'No prefix parse function for token Comment' not found for multi-comment. Errors: {:?}", parser_multi_comment.errors);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{BlockStatement, Expression, InfixOperator, Parameter, Program, Statement};
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    fn parse_input_to_program(input: &str) -> Program {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        if !parser.errors.is_empty() {
            panic!("Parser errors occurred: {:?}", parser.errors);
        }
        program
    }

    #[test]
    fn test_parse_let_statement() {
        let input = "let x = 5\n"; // Parser expects newline termination for statements
        let program = parse_input_to_program(input);

        assert_eq!(program.body.len(), 1, "Program should have 1 statement.");
        match &program.body[0] {
            Statement::Let { name, value } => {
                assert_eq!(name, "x", "Let statement name is incorrect.");
                match value {
                    Expression::LiteralInteger(val) => {
                        assert_eq!(*val, 5, "Integer literal value is incorrect.");
                    }
                    _ => panic!("Expected LiteralInteger, got {:?}", value),
                }
            }
            _ => panic!("Expected Let statement, got {:?}", program.body[0]),
        }
    }

    #[test]
    fn test_parse_simple_function_declaration() {
        let input = "fn main()\n  let greeting = \"hello\"\n";
        let program = parse_input_to_program(input);

        assert_eq!(program.body.len(), 1);
        match &program.body[0] {
            Statement::FunctionDeclaration { name, params, return_type, body } => {
                assert_eq!(name, "main");
                assert!(params.is_empty(), "Expected no parameters.");
                assert!(return_type.is_none(), "Expected no return type.");
                assert_eq!(body.statements.len(), 1, "Function body should have one statement.");
                match &body.statements[0] {
                    Statement::Let { name: let_name, value } => {
                        assert_eq!(let_name, "greeting");
                        match value {
                            Expression::LiteralString(s_val) => assert_eq!(s_val, "hello"),
                            _ => panic!("Expected LiteralString for let value"),
                        }
                    }
                    _ => panic!("Expected Let statement in function body."),
                }
            }
            _ => panic!("Expected FunctionDeclaration statement."),
        }
    }

    #[test]
    fn test_parse_function_with_params_and_return() {
        let input = "fn add(a: i32, b: i32) -> i32\n  a + b\n";
        let program = parse_input_to_program(input);

        assert_eq!(program.body.len(), 1);
        match &program.body[0] {
            Statement::FunctionDeclaration { name, params, return_type, body } => {
                assert_eq!(name, "add");
                assert_eq!(params.len(), 2);
                assert_eq!(params[0], Parameter { name: "a".to_string(), type_ann: Some("i32".to_string()) });
                assert_eq!(params[1], Parameter { name: "b".to_string(), type_ann: Some("i32".to_string()) });
                assert_eq!(return_type, &Some("i32".to_string()));
                assert_eq!(body.statements.len(), 1);
                match &body.statements[0] {
                    Statement::ExpressionStatement { expression } => {
                        match expression {
                            Expression::InfixExpression { operator, .. } => { // Basic check
                                assert_eq!(*operator, InfixOperator::Plus);
                            }
                            _ => panic!("Expected InfixExpression in function body."),
                        }
                    }
                    _ => panic!("Expected ExpressionStatement in function body."),
                }
            }
            _ => panic!("Expected FunctionDeclaration statement."),
        }
    }

    #[test]
    fn test_parse_integer_literal_expression() {
        let input = "5\n"; // Expression statements also need newline
        let program = parse_input_to_program(input);
        assert_eq!(program.body.len(), 1);
        match &program.body[0] {
            Statement::ExpressionStatement { expression } => match expression {
                Expression::LiteralInteger(val) => assert_eq!(*val, 5),
                _ => panic!("Expected LiteralInteger expression."),
            },
            _ => panic!("Expected ExpressionStatement."),
        }
    }

    #[test]
    fn test_parse_identifier_expression() {
        let input = "my_var\n";
        let program = parse_input_to_program(input);
        assert_eq!(program.body.len(), 1);
        match &program.body[0] {
            Statement::ExpressionStatement { expression } => match expression {
                Expression::Identifier(name) => assert_eq!(name, "my_var"),
                _ => panic!("Expected Identifier expression."),
            },
            _ => panic!("Expected ExpressionStatement."),
        }
    }

    #[test]
    fn test_parse_infix_expressions() {
        let inputs_results = vec![
            ("5 + 3\n", Expression::InfixExpression {
                left: Box::new(Expression::LiteralInteger(5)),
                operator: InfixOperator::Plus,
                right: Box::new(Expression::LiteralInteger(3))
            }),
            ("10 * 2\n", Expression::InfixExpression {
                left: Box::new(Expression::LiteralInteger(10)),
                operator: InfixOperator::Star,
                right: Box::new(Expression::LiteralInteger(2))
            }),
            ("1 + 2 * 3\n", Expression::InfixExpression { // Tests precedence 1 + (2 * 3)
                left: Box::new(Expression::LiteralInteger(1)),
                operator: InfixOperator::Plus,
                right: Box::new(Expression::InfixExpression {
                    left: Box::new(Expression::LiteralInteger(2)),
                    operator: InfixOperator::Star,
                    right: Box::new(Expression::LiteralInteger(3)),
                }),
            }),
            ("(1 + 2) * 3\n", Expression::InfixExpression { // Tests grouping (1 + 2) * 3
                left: Box::new(Expression::GroupedExpression(Box::new(Expression::InfixExpression {
                    left: Box::new(Expression::LiteralInteger(1)),
                    operator: InfixOperator::Plus,
                    right: Box::new(Expression::LiteralInteger(2)),
                }))),
                operator: InfixOperator::Star,
                right: Box::new(Expression::LiteralInteger(3)),
            }),
        ];

        for (input, expected_expr) in inputs_results {
            let program = parse_input_to_program(input);
            assert_eq!(program.body.len(), 1, "Failed for input: {}", input);
            match &program.body[0] {
                Statement::ExpressionStatement { expression } => {
                    assert_eq!(expression, &expected_expr, "AST mismatch for input: {}", input);
                }
                _ => panic!("Expected ExpressionStatement for input: {}", input),
            }
        }
    }

    #[test]
    fn test_parse_call_expression() {
        let input = "add(1, 2 * 3)\n";
        let program = parse_input_to_program(input);
        assert_eq!(program.body.len(), 1);
        match &program.body[0] {
            Statement::ExpressionStatement { expression } => match expression {
                Expression::FunctionCall { function, arguments } => {
                    match &**function {
                        Expression::Identifier(name) => assert_eq!(name, "add"),
                        _ => panic!("Expected Identifier for function name."),
                    }
                    assert_eq!(arguments.len(), 2);
                    match &arguments[0] {
                        Expression::LiteralInteger(val) => assert_eq!(*val, 1),
                        _ => panic!("Expected LiteralInteger for first argument."),
                    }
                    match &arguments[1] {
                        Expression::InfixExpression { .. } => { /* Correct type */ }
                        _ => panic!("Expected InfixExpression for second argument."),
                    }
                }
                _ => panic!("Expected FunctionCall expression."),
            },
            _ => panic!("Expected ExpressionStatement."),
        }
    }

    #[test]
    fn test_parser_errors() {
        let input = "let x 5\n"; // Missing '='
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse_program(); // We don't check the program, just the errors.
        assert!(!parser.errors.is_empty(), "Expected parser errors for input: {}", input);
        // Optionally, check for specific error messages
        // e.g., assert!(parser.errors[0].contains("Expected next token to be Eq"));
        println!("Parser errors for 'let x 5': {:?}", parser.errors); // Keep for debugging if needed
    }

    #[test]
    fn test_parser_errors_let_statement() {
        let test_cases = vec![
            ("let x 5\n", "Expected next token to be Eq, got Integer(5) instead"), // Missing =
            ("let = 5\n", "Expected next token to be Ident(\"\"), got Eq instead"),   // Missing identifier
            ("let x =\n", "Expected expression after '=' in let statement for 'x'."), // Missing expression
            ("let x = 5 y\n", "Expected newline after expression in let statement, got Ident(\"y\")"), // Unexpected token
        ];

        for (input, expected_error_fragment) in test_cases {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            parser.parse_program();
            assert!(!parser.errors.is_empty(), "No errors found for input: '{}'", input);
            // Check if at least one error message contains the expected fragment.
            let found_error = parser.errors.iter().any(|e| e.contains(expected_error_fragment));
            assert!(found_error, "Expected error fragment '{}' not found in errors {:?} for input '{}'", expected_error_fragment, parser.errors, input);
        }

        // Test case: `let x =` (EOF instead of expression)
        let input_eof = "let x =";
        let lexer_eof = Lexer::new(input_eof);
        let mut parser_eof = Parser::new(lexer_eof);
        parser_eof.parse_program();
        assert!(!parser_eof.errors.is_empty(), "No errors found for input: '{}'", input_eof);
        let found_eof_error = parser_eof.errors.iter().any(|e| e.contains("Expected expression after '='"));
        assert!(found_eof_error, "Expected error for EOF after '=' not found in errors {:?} for input '{}'", parser_eof.errors, input_eof);

    }

    #[test]
    fn test_parser_errors_function_declaration() {
        let test_cases = vec![
            ("fn ()\n", "Expected next token to be Ident(\"\"), got LParen instead"), // Missing function name
            ("fn myFunction\n", "Expected next token to be LParen, got Newline instead"), // Missing ()
            ("fn myFunction(\n", "Expected ')' to close parameter list, got Eof instead"), // Missing )
            ("fn myFunction(a b)\n", "Expected comma or ')' after parameter, got Ident(\"b\") instead"), // Missing comma or type
            ("fn myFunction(a:)\n", "Expected type identifier after ':' in parameter, got RParen instead"), // Missing type after colon
            ("fn myFunction(a: num\n", "Expected ')' to close parameter list, got Eof instead"), // Missing ) after param type
            ("fn myFunction() -> \n", "Expected return type identifier after ':', got Newline instead"), // Missing return type
            ("fn myFunction() -> num\n", "Expected newline before function body block, got Eof instead"), // Missing body (Newline/Indent)
            // For "missing Dedent at end of block", the parser currently expects a Dedent token.
            // If the input is "fn myFunction()\n  let x = 1\n" (and then EOF)
            // parse_block_statement will consume Indent, then 'let x = 1\n'.
            // Then it will look for Dedent. If current_token is EOF, it will report "Expected Dedent... got Eof".
            ("fn myFunction()\n  let x = 1\n", "Expected Dedent to end block, got Eof instead"),
        ];

        for (input, expected_error_fragment) in test_cases {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            parser.parse_program();
            assert!(!parser.errors.is_empty(), "No errors found for input: '{}'", input);
            let found_error = parser.errors.iter().any(|e| e.contains(expected_error_fragment));
            assert!(found_error, "Expected error fragment '{}' not found in errors {:?} for input '{}'", expected_error_fragment, parser.errors, input);
        }

        // Specific test for `fn myFunction(a: num` (missing `)` but has content after type)
        let input_specific = "fn myFunction(a: num b)\n"; // `b` is unexpected after type
        let lexer_specific = Lexer::new(input_specific);
        let mut parser_specific = Parser::new(lexer_specific);
        parser_specific.parse_program();
        assert!(!parser_specific.errors.is_empty(), "No errors for specific case: '{}'", input_specific);
        let found_specific_error = parser_specific.errors.iter().any(|e| e.contains("Expected comma or ')' after parameter, got Ident(\"b\")"));
        assert!(found_specific_error, "Specific error for 'fn myFunction(a: num b)' not found, errors: {:?}", parser_specific.errors);


        // Test for missing Indent after function signature + Newline
        let input_missing_indent = "fn myFunc()\nlet x = 1\n";
        let lexer_missing_indent = Lexer::new(input_missing_indent);
        let mut parser_missing_indent = Parser::new(lexer_missing_indent);
        parser_missing_indent.parse_program();
        assert!(!parser_missing_indent.errors.is_empty(), "No errors for missing indent: '{}'", input_missing_indent);
        // expect_peek for Indent is called. peek_token would be 'Let'.
        let found_missing_indent_error = parser_missing_indent.errors.iter().any(|e| e.contains("Expected indent for function body, got Let instead"));
        assert!(found_missing_indent_error, "Error for missing indent not found, errors: {:?}", parser_missing_indent.errors);
    }

    #[test]
    fn test_parse_print_function_call_literal() {
        let input = "print(\"Hello, World!\")\n";
        let program = parse_input_to_program(input);

        assert_eq!(program.body.len(), 1, "Program should have 1 statement.");
        match &program.body[0] {
            Statement::ExpressionStatement { expression } => {
                match expression {
                    Expression::FunctionCall { function, arguments } => {
                        match &**function {
                            Expression::Identifier(name) => {
                                assert_eq!(name, "print", "Function name should be 'print'.");
                            }
                            _ => panic!("Expected Identifier for function name, got {:?}", function),
                        }
                        assert_eq!(arguments.len(), 1, "Print call should have 1 argument.");
                        match &arguments[0] {
                            Expression::LiteralString(value) => {
                                assert_eq!(value, "Hello, World!", "Argument value mismatch.");
                            }
                            _ => panic!("Expected LiteralString for print argument, got {:?}", arguments[0]),
                        }
                    }
                    _ => panic!("Expected FunctionCall expression, got {:?}", expression),
                }
            }
            _ => panic!("Expected ExpressionStatement, got {:?}", program.body[0]),
        }
    }

    #[test]
    fn test_parse_print_function_call_identifier() {
        let input = r#"
let my_message = "test"
print(my_message)
"#;
        let program = parse_input_to_program(input);

        assert_eq!(program.body.len(), 2, "Program should have 2 statements.");

        // Check the second statement: print(my_message)
        match &program.body[1] {
            Statement::ExpressionStatement { expression } => {
                match expression {
                    Expression::FunctionCall { function, arguments } => {
                        match &**function {
                            Expression::Identifier(name) => {
                                assert_eq!(name, "print", "Function name should be 'print'.");
                            }
                            _ => panic!("Expected Identifier for function name, got {:?}", function),
                        }
                        assert_eq!(arguments.len(), 1, "Print call should have 1 argument.");
                        match &arguments[0] {
                            Expression::Identifier(name) => {
                                assert_eq!(name, "my_message", "Argument should be identifier 'my_message'.");
                            }
                            _ => panic!("Expected Identifier for print argument, got {:?}", arguments[0]),
                        }
                    }
                    _ => panic!("Expected FunctionCall expression for print statement, got {:?}", expression),
                }
            }
            _ => panic!("Expected ExpressionStatement for print statement, got {:?}", program.body[1]),
        }
    }

    #[test]
    fn test_print_is_builtin_in_symbol_table() {
        // Empty input is fine, we just need the parser's initial state
        let lexer = Lexer::new("");
        let parser = Parser::new(lexer); // Parser::new() should define "print"

        match parser.symbol_table.resolve(&"print".to_string()) {
            Some(symbol) => {
                assert_eq!(symbol.name, "print");
                assert_eq!(symbol.kind, SymbolKind::BuiltInFunction, "'print' should be registered as a BuiltInFunction.");
            }
            None => panic!("'print' built-in function not found in symbol table after parser initialization."),
        }
    }

    #[test]
    fn test_user_cannot_redefine_print_variable() {
        let input = "let print = 123\n";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse_program();

        assert!(!parser.errors.is_empty(), "Expected parser errors when redefining 'print' as a variable.");
        assert!(
            parser.errors.iter().any(|e| e.contains("Symbol 'print' is already defined")),
            "Error message for redefining 'print' as variable not found. Errors: {:?}", parser.errors
        );
    }

    #[test]
    fn test_user_cannot_redefine_print_function() {
        let input = r#"
fn print(s: String) // Attempt to redefine print
  // some other implementation
  let x = 1
"#;
        // Note: The body needs to be valid enough to not cause other errors before the symbol definition.
        // Adding a newline and proper indent/dedent if the parser expects it.
        // The current parser expects Newline, Indent, (statements), Dedent for a function body.
        // So, the input should be:
        // fn print(s: String)
        //   let x = 1
        //
        // Let's make the input for the test robust for function body parsing:
        let full_input = "fn print(s: String)\n  let x = 1\n";

        let lexer = Lexer::new(full_input);
        let mut parser = Parser::new(lexer);
        parser.parse_program();

        assert!(!parser.errors.is_empty(), "Expected parser errors when redefining 'print' as a function.");
        assert!(
            parser.errors.iter().any(|e| e.contains("Symbol 'print' is already defined")),
            "Error message for redefining 'print' as function not found. Errors: {:?}", parser.errors
        );
    }

    #[test]
    fn test_symbol_registration_let_and_function() {
        let input = r#"
let x = 10
fn my_func()
  let y = 20 // y is local, should not be in global symbol table
"#;
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse_program();

        assert!(parser.errors.is_empty(), "Expected no parser errors, got: {:?}", parser.errors);

        // Check global 'x'
        match parser.symbol_table.resolve(&"x".to_string()) {
            Some(symbol) => {
                assert_eq!(symbol.name, "x");
                assert_eq!(symbol.kind, SymbolKind::Variable);
            }
            None => panic!("Global variable 'x' not found in symbol table."),
        }

        // Check global 'my_func'
        match parser.symbol_table.resolve(&"my_func".to_string()) {
            Some(symbol) => {
                assert_eq!(symbol.name, "my_func");
                assert_eq!(symbol.kind, SymbolKind::Function);
            }
            None => panic!("Global function 'my_func' not found in symbol table."),
        }

        // Check that local 'y' is NOT in the global symbol table
        assert!(
            parser.symbol_table.resolve(&"y".to_string()).is_none(),
            "Local variable 'y' should not be in the global symbol table."
        );
    }

    #[test]
    fn test_duplicate_variable_declaration_error() {
        let input = r#"
let x = 10
let x = 20
"#;
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse_program();

        assert!(!parser.errors.is_empty(), "Expected parser errors for duplicate variable declaration.");
        // The error comes from symbol_table.define()
        assert!(
            parser.errors.iter().any(|e| e.contains("Symbol 'x' is already defined")),
            "Error message for duplicate variable 'x' not found. Errors: {:?}", parser.errors
        );
    }

    #[test]
    fn test_duplicate_function_declaration_error() {
        let input = r#"
fn my_func()
  let a = 1
fn my_func()
  let b = 2
"#;
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse_program();

        assert!(!parser.errors.is_empty(), "Expected parser errors for duplicate function declaration.");
        assert!(
            parser.errors.iter().any(|e| e.contains("Symbol 'my_func' is already defined")),
            "Error message for duplicate function 'my_func' not found. Errors: {:?}", parser.errors
        );
    }

    #[test]
    fn test_duplicate_variable_and_function_name_error() {
        let test_cases = vec![
            (r#"
let x = 10
fn x()
  let a = 1
"#, "x", "variable then function"),
            (r#"
fn y()
  let a = 1
let y = 20
"#, "y", "function then variable"),
        ];

        for (input, name, scenario) in test_cases {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            parser.parse_program();

            assert!(!parser.errors.is_empty(), "Expected parser errors for duplicate var/fn name scenario: {}", scenario);
            let expected_error_msg = format!("Symbol '{}' is already defined", name);
            assert!(
                parser.errors.iter().any(|e| e.contains(&expected_error_msg)),
                "Error message for duplicate var/fn name '{}' (scenario: {}) not found. Errors: {:?}", name, scenario, parser.errors
            );
        }
    }

    #[test]
    fn test_no_error_on_shadowing_in_different_scope() {
        let input = r#"
let x = 10
fn my_func()
  let x = 20 // Shadowing x, but in a local scope (not registered globally by current parser)
"#;
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        parser.parse_program();

        assert!(parser.errors.is_empty(), "Expected no parser errors for shadowing in different scope (current ST is global only), got: {:?}", parser.errors);

        // Verify the global 'x' is still the original one.
        match parser.symbol_table.resolve(&"x".to_string()) {
            Some(symbol) => {
                assert_eq!(symbol.name, "x");
                assert_eq!(symbol.kind, SymbolKind::Variable, "Global 'x' should be a Variable.");
                // We can't directly check the value from the symbol table as it's not stored there,
                // but its presence and kind confirm it's the global 'let x = 10'.
            }
            None => panic!("Global variable 'x' not found in symbol table after parsing shadowing case."),
        }

        // Verify 'my_func' is registered
        assert!(
            parser.symbol_table.resolve(&"my_func".to_string()).is_some(),
            "Function 'my_func' should be registered in the global symbol table."
        );
    }

    #[test]
    fn test_parser_errors_expressions() {
        let test_cases = vec![
            // Missing right operand
            ("5 + \n", "Expected expression on the right side of Plus operator."),
            // Missing left operand for infix (current parser sees '*' as unexpected prefix)
            ("* 5\n", "No prefix parse function for token Star"),
            // Trailing comma in function call argument list
            // parse_expression_list: current_token is Comma. next_token_internal() consumes Comma.
            // parse_expression is called. current_token is now RParen.
            // parse_expression fails as RParen has no prefix parse fn.
            // Error: "No prefix parse function for token RParen"
            // The "Trailing comma" error in parse_expression_list is specific to `(arg1, )` where current_token becomes RParen *after* comma.
            // If input is "myFunc(a,)\n":
            // parse_call_expression -> parse_expression_list(RParen)
            //  - current_token is LParen. peek_token is Ident("a")
            //  - consumes LParen. current_token is Ident("a")
            //  - parse_expression for "a". list = [Ident("a")]. current_token is Ident("a")
            //  - peek_token is Comma. Consume "a", Consume Comma. current_token is Comma.
            //  - *Now, check for trailing comma*: if current_token (Comma) is RParen -> false.
            //  - *Then, parse_expression for thing after comma*: current_token is Comma.
            //    The code has `self.next_token_internal(); // Consume the Comma. current_token is now Comma.` which seems wrong.
            //    It should be `self.next_token_internal(); // Consume previous expression token`
            //    `self.next_token_internal(); // Consume Comma. current_token is now start of next expr or RParen.`
            //    Let's re-check parse_expression_list logic for "myFunc(a,)\n"
            //    After parsing "a", current_token is "a". peek_token is ",".
            //    Loop `while self.peek_token_is(&TokenKind::Comma)`: true
            //      `self.next_token_internal();` // current_token becomes "," (from "a")
            //      `self.next_token_internal();` // current_token becomes ")" (from ",")
            //      Now, `if self.current_token_is(&end_token_kind)` (i.e. current_token is RParen): true
            //        This triggers "Trailing comma before RParen..."
            ("myFunc(a,)\n", "Trailing comma before RParen in expression list."),
            // Missing closing parenthesis for grouped expression
            ("(5 + 2\n", "Expected next token to be RParen, got Newline instead"),
        ];

        for (input, expected_error_fragment) in test_cases {
            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            parser.parse_program(); // Attempt to parse the program
            assert!(!parser.errors.is_empty(), "No errors found for input: '{}'", input);
            let found_error = parser.errors.iter().any(|e| e.contains(expected_error_fragment));
            assert!(found_error, "Expected error fragment '{}' not found in errors {:?} for input '{}'", expected_error_fragment, parser.errors, input);
        }

        // Test case: `5 +` (EOF instead of right operand)
        let input_eof = "5 +";
        let lexer_eof = Lexer::new(input_eof);
        let mut parser_eof = Parser::new(lexer_eof);
        parser_eof.parse_program();
        assert!(!parser_eof.errors.is_empty(), "No errors for input: '{}'", input_eof);
        let found_eof_error = parser_eof.errors.iter().any(|e| e.contains("Expected expression on the right side of Plus operator."));
        assert!(found_eof_error, "Specific EOF error for '5 +' not found, errors: {:?}", parser_eof.errors);
    }

    #[test]
    fn test_complex_expressions_nested_calls_precedence() {
        // Test 1: Precedence a + b * c - d / e  => (a + (b * c)) - (d / e)
        let input1 = "a + b * c - d / e\n";
        let program1 = parse_input_to_program(input1);
        assert_eq!(program1.body.len(), 1);
        match &program1.body[0] {
            Statement::ExpressionStatement { expression } => {
                // Expected: ((a + (b * c)) - (d / e))
                // Or, due to left-associativity of + and - at same level: (a + (b*c)) - (d/e)
                // The parser should build it as: Infix(Infix(Ident(a), Plus, Infix(Ident(b), Star, Ident(c))), Minus, Infix(Ident(d), Slash, Ident(e)))
                // Or: Infix(left: Infix(left: Ident(a), op: Plus, right: Infix(left: Ident(b), op: Star, right: Ident(c))), op: Minus, right: Infix(left: Ident(d), op: Slash, right: Ident(e)))
                // This is how a typical Pratt parser handles it.
                // Let's verify the top-level Minus, then its left and right.
                if let Expression::InfixExpression { left: l1, operator: op1, right: r1 } = expression { // (a + (b*c))  MINUS  (d/e)
                    assert_eq!(*op1, InfixOperator::Minus);
                    // Check left side of Minus: a + (b*c)
                    if let Expression::InfixExpression { left: l2, operator: op2, right: r2 } = &**l1 { // a PLUS (b*c)
                        assert_eq!(*op2, InfixOperator::Plus);
                        assert_eq!(**l2, Expression::Identifier("a".to_string())); // a
                        // Check right side of Plus: b*c
                        if let Expression::InfixExpression { left: l3, operator: op3, right: r3 } = &**r2 { // b STAR c
                            assert_eq!(*op3, InfixOperator::Star);
                            assert_eq!(**l3, Expression::Identifier("b".to_string())); // b
                            assert_eq!(**r3, Expression::Identifier("c".to_string())); // c
                        } else { panic!("Expected infix b*c, got {:?}", r2); }
                    } else { panic!("Expected infix a+(b*c), got {:?}", l1); }
                    // Check right side of Minus: d/e
                    if let Expression::InfixExpression { left: l4, operator: op4, right: r4 } = &**r1 { // d SLASH e
                        assert_eq!(*op4, InfixOperator::Slash);
                        assert_eq!(**l4, Expression::Identifier("d".to_string())); // d
                        assert_eq!(**r4, Expression::Identifier("e".to_string())); // e
                    } else { panic!("Expected infix d/e, got {:?}", r1); }
                } else { panic!("Expected top-level InfixExpression (Minus), got {:?}", expression); }
            }
            _ => panic!("Expected ExpressionStatement"),
        }

        // Test 2: func1(func2(a), b + c)
        let input2 = "func1(func2(a), b + c)\n";
        let program2 = parse_input_to_program(input2);
        assert_eq!(program2.body.len(), 1);
        match &program2.body[0] {
            Statement::ExpressionStatement { expression } => {
                if let Expression::FunctionCall { function: f1_name, arguments: args1 } = expression {
                    assert_eq!(**f1_name, Expression::Identifier("func1".to_string()));
                    assert_eq!(args1.len(), 2);
                    // Arg 1: func2(a)
                    if let Expression::FunctionCall { function: f2_name, arguments: args2 } = &args1[0] {
                        assert_eq!(**f2_name, Expression::Identifier("func2".to_string()));
                        assert_eq!(args2.len(), 1);
                        assert_eq!(args2[0], Expression::Identifier("a".to_string()));
                    } else { panic!("Expected func2(a) as first arg of func1, got {:?}", args1[0]); }
                    // Arg 2: b + c
                    if let Expression::InfixExpression { left: l_bc, operator: op_bc, right: r_bc } = &args1[1] {
                        assert_eq!(*op_bc, InfixOperator::Plus);
                        assert_eq!(**l_bc, Expression::Identifier("b".to_string()));
                        assert_eq!(**r_bc, Expression::Identifier("c".to_string()));
                    } else { panic!("Expected b+c as second arg of func1, got {:?}", args1[1]); }
                } else { panic!("Expected func1 call, got {:?}", expression); }
            }
            _ => panic!("Expected ExpressionStatement"),
        }

        // Test 3: (a + b) * (c - d)
        let input3 = "(a + b) * (c - d)\n";
        let program3 = parse_input_to_program(input3);
        assert_eq!(program3.body.len(), 1);
        match &program3.body[0] {
            Statement::ExpressionStatement { expression } => {
                if let Expression::InfixExpression { left: group1_expr, operator: op_mul, right: group2_expr } = expression {
                    assert_eq!(*op_mul, InfixOperator::Star);
                    // Left side: (a + b)
                    if let Expression::GroupedExpression(inner_g1) = &**group1_expr {
                        if let Expression::InfixExpression {left: l_ab, operator: op_ab, right: r_ab } = &**inner_g1 {
                            assert_eq!(*op_ab, InfixOperator::Plus);
                            assert_eq!(**l_ab, Expression::Identifier("a".to_string()));
                            assert_eq!(**r_ab, Expression::Identifier("b".to_string()));
                        } else { panic!("Expected a+b inside group1, got {:?}", inner_g1); }
                    } else { panic!("Expected GroupedExpression for (a+b), got {:?}", group1_expr); }
                    // Right side: (c - d)
                    if let Expression::GroupedExpression(inner_g2) = &**group2_expr {
                        if let Expression::InfixExpression {left: l_cd, operator: op_cd, right: r_cd } = &**inner_g2 {
                            assert_eq!(*op_cd, InfixOperator::Minus);
                            assert_eq!(**l_cd, Expression::Identifier("c".to_string()));
                            assert_eq!(**r_cd, Expression::Identifier("d".to_string()));
                        } else { panic!("Expected c-d inside group2, got {:?}", inner_g2); }
                    } else { panic!("Expected GroupedExpression for (c-d), got {:?}", group2_expr); }
                } else { panic!("Expected top-level InfixExpression (Star), got {:?}", expression); }
            }
            _ => panic!("Expected ExpressionStatement"),
        }
    }
}
