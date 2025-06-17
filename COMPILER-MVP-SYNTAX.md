# ASBEL MVP Syntax Specification

This document outlines the core syntax elements for the Minimum Viable Product (MVP) of the ASBEL compiler. This specification will guide the development of the lexer and parser.

## Keywords

The following are reserved keywords and cannot be used as identifiers:

- `fn`: Used to define a function.
- `let`: Used to declare a variable.

## Identifiers

Identifiers are names used for variables, functions, etc. They must follow these rules:

- Start with an alphabet character (a-z, A-Z) or an underscore (_).
- Subsequent characters can be alphanumeric (a-z, A-Z, 0-9) or an underscore (_).
- Regular Expression: `[a-zA-Z_][a-zA-Z0-9_]*`

Examples: `myVariable`, `_internal`, `calculateValue1`

## Literals

### Integer Literals

Integer literals represent whole numbers.

- Examples: `123`, `0`, `42`
- Underscores (`_`) can be used as separators for readability, but are ignored by the lexer.
- Examples with separators: `1_000`, `12_345_678`

### String Literals

String literals represent sequences of characters.

- Enclosed in double quotes (`"`).
- For the MVP, complex escape sequences are not supported. The lexer will treat the content within the quotes literally.
- Examples: `"Hello, World!"`, `"ASBEL MVP"`, `""` (empty string)

## Operators

### Function Call

- `()`: Used to invoke a function. Arguments are placed within the parentheses.
- Example: `myFunction(arg1, arg2)`

### Assignment

- `=`: Used to assign a value to a variable.
- Example: `let x = 10`

### Arithmetic Operators

Basic arithmetic operations:

- `+`: Addition
- `-`: Subtraction
- `*`: Multiplication
- `/`: Division
- Examples: `a + b`, `count - 1`, `price * quantity`

## Punctuation

### Argument Separator

- `,`: Used to separate arguments in function calls and definitions.
- Example: `fn greet(name, age)`

### Type Annotation

- `:`: Used to denote a type for a variable or function parameter/return value.
- Parsing type annotations might be optional for the absolute MVP if type inference is sufficient for a "Hello World" program. However, the syntax should be recognized.
- Example: `let x: i32 = 10`, `fn add(a: i32, b: i32) -> i32`

### Statement Terminator

- Newline: Statements are typically terminated by a newline character. Multiple statements on the same line are not supported in the MVP.

## Structure

### Indentation

- Indentation is significant and used to define code blocks (e.g., function bodies).
- **4 spaces** will be used for each level of indentation. Tabs are not allowed for indentation to ensure consistency.

### Parentheses for Grouping Expressions

- `()`: Used to explicitly group expressions to control the order of operations.
- Example: `(a + b) * c`

## Comments

### Line Comments

- `//`: Any text from `//` to the end of the line is considered a comment and ignored by the lexer.
- Example: `// This is a single-line comment`

## Basic Program Structure Example

Below is an example demonstrating a simple ASBEL program structure:

```asbel
// This is a comment illustrating the basic structure
fn main()
    let greeting = "Hello, ASBEL MVP!"
    print(greeting) // Assuming a built-in 'print' function for MVP output

fn add(a: i32, b: i32) -> i32
    a + b // Implicit return of the last expression in a block
```
