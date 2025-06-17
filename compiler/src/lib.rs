pub mod lexer;
pub mod ast;
pub mod parser;

pub use lexer::{Lexer, Token, TokenKind};
pub use ast::*; // Export all AST nodes
pub use parser::Parser;

// Minimal top-level tests, or integration tests that cover more than one unit.
// Specific unit tests for lexer are in lexer.rs, for parser in parser.rs.
#[cfg(test)]
mod tests {
    // Example of a very basic integration test (optional)
    // Most detailed tests are now in their respective modules.
    #[test]
    fn it_works() {
        // A simple test to ensure things link and run.
        // Could parse a very simple "program" here if desired.
        // For example:
        // use super::*;
        // let input = "let x = 5\n";
        // let lexer = Lexer::new(input);
        // let mut parser = Parser::new(lexer);
        // let program = parser.parse_program();
        // assert!(parser.errors.is_empty());
        // assert_eq!(program.body.len(), 1);
        assert_eq!(2 + 2, 4); // Placeholder
    }
}
