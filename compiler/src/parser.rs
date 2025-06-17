use crate::ast::*;
use crate::lexer::{Lexer, Token, TokenKind};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
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

        Parser {
            lexer,
            current_token: first_token,
            peek_token: second_token,
            errors: Vec::new(),
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
            // Skip empty newlines between statements if any
            while self.current_token.kind == TokenKind::Newline {
                self.next_token_internal();
            }
            if self.current_token.kind == TokenKind::Eof { // Check again after skipping newlines
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
            // Skip any leading newlines within the block (e.g. empty lines)
            while self.current_token_is(&TokenKind::Newline) {
                self.next_token_internal();
            }
            // Check again after skipping newlines
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
        println!("Parser errors for 'let x 5': {:?}", parser.errors);
    }
}
