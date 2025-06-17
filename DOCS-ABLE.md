# ASBEL (A Simple But Efficient Language) - Technical Documentation

## Introduction

ASBEL (A Simple But Efficient Language) is a programming language designed with the ambitious goal of bridging the gap between the readability and productivity of high-level languages like Python and the safety and performance of low-level systems languages like C and Rust. Its core philosophy is **"transparent abstraction,"** where the behavior of the source code maps directly and predictably to the compiled binary, minimizing surprises and making performance analysis intuitive.

To achieve this, ASBEL is implemented in Rust and **transpiles to highly optimized C code**. This strategic choice leverages mature and ubiquitous C compilers like GCC and Clang, ensuring maximum performance and broad platform compatibility from day one.

This document provides a deep dive into the ASBEL language specification, detailing its syntax, semantics, and the key features that set it apart. It is intended for developers, language designers, and anyone interested in understanding the foundational principles of ASBEL and its place in the modern programming landscape.

ASBEL's mission is to empower developers to write robust, efficient systems software without the cognitive overhead of manual memory management or the syntactic complexity of explicit lifetime annotations. Through a carefully designed system of inferred ownership and borrowing, ASBEL aims to eliminate entire classes of common programming errors while maintaining a clean, expressive, and productive development experience.

## 1. Variables and Types

ASBEL provides an intuitive and flexible variable declaration system with a strong emphasis on type inference and safety. The language clearly distinguishes between immutable and mutable variables, promoting a programming style that reduces side effects and improves code predictability.

### 1.1. Variable Declaration

ASBEL uses two primary keywords for variable declaration:

*   `let`: Declares an **immutable** variable. Once a value is assigned to a `let` binding, it cannot be modified. This encourages a functional style and helps prevent bugs by ensuring data does not change unexpectedly.
*   `var`: Declares a **mutable** variable. `var` bindings can be reassigned to new values after their initial declaration. This is intended for state that must change over time, such as counters or accumulators.

Type inference is a cornerstone of ASBEL. The compiler automatically deduces a variable's type from its assigned value, reducing verbosity and allowing developers to focus on logic rather than type annotations.

**Examples:**

```asbel
let name = "ASBEL"         // 'name' is an immutable String
var counter = 0             // 'counter' is a mutable i32 (default integer type)
let pi = 3.14159            // 'pi' is an immutable f64 (default float type)
var is_active = true        // 'is_active' is a mutable Bool
```

While inference is the default, ASBEL supports explicit type annotations for clarity, API compliance, or when the inferred type is not the one desired.

**Example of Explicit Annotation:**

```asbel
let count: i64 = 100_000_000_000 // 'count' is an immutable i64
var temperature: f32 = 25.5      // 'temperature' is a mutable f32
```

### 1.2. Refined Types and Ranges

One of ASBEL's most powerful safety features is its support for **refined types**, which allow value constraints to be specified directly within the type declaration. This is achieved by appending an optional range or predicate to a base type. The ASBEL compiler uses this information to perform compile-time checks and, where necessary, insert runtime checks to guarantee that variable values remain within their defined bounds. This moves data validation from procedural code into the type system itself.

The syntax for range-based refined types is `Type(min..=max)` for inclusive ranges and `Type(min..max)` for exclusive ranges. Open-ended ranges using `..` or `..=` are also supported.

**Examples:**

```asbel
// 'pct' is an 8-bit unsigned integer, guaranteed to be between 0 and 100.
let pct: u8(0..=100) = 42

// 'angle' is a 32-bit float, guaranteed to be between 0.0 and 360.0.
var angle: f32(0..=360) = 270.0

// Attempting to assign an out-of-range value results in a compile-time error
// or a controlled runtime panic.
// let invalid_pct: u8(0..=100) = 101 // Error: Value out of range

// The function signature itself enforces the contract.
fn set_volume(level: u8(0..=10))
    // ...

// 'positive_value' must be an integer greater than or equal to 1.
let positive_value: i32(1..) = 5

// 'negative_value' must be an integer less than 0.
let negative_value: i32(..0) = -10
```

Refined types are a cornerstone of Design by Contract and data validation in ASBEL. They empower the compiler to catch logical errors early, reducing the need for manual runtime checks and dramatically improving software reliability, all while minimizing performance impact in line with the "transparent abstraction" philosophy.

### 1.3. Primitive Data Types

ASBEL supports a comprehensive set of primitive data types:

*   **Integers:**
    *   Signed: `i8`, `i16`, `i32`, `i64`, `i128`
    *   Unsigned: `u8`, `u16`, `u32`, `u64`, `u128`
    *   The default inferred integer type is `i32`.
*   **Floating-Point:**
    *   `f32` (single-precision)
    *   `f64` (double-precision)
    *   The default inferred float type is `f64`.
*   **Boolean:**
    *   `Bool`: Represents truth values, `true` or `false`.
*   **Character:**
    *   `Char`: Represents a single Unicode scalar value. Defined with single quotes (e.g., `'A'`, `'ñ'`, `'😊'`).
*   **String:**
    *   `String`: Represents a UTF-8 encoded sequence of characters. Strings are immutable by default and defined with double quotes (e.g., `"Hello, World!"`).

### 1.4. Literals

ASBEL supports various literal formats for improved readability:

*   **Integer Literals:** `10`, `1_000_000` (underscores can be used as visual separators).
*   **Floating-Point Literals:** `3.14`, `1.2e-5`.
*   **Boolean Literals:** `true`, `false`.
*   **Character Literals:** `'a'`, `'\n'`, `'\u{1F600}'` (Unicode escape).
*   **String Literals:** `"Hello"`, `"A multi-line
    string"`.

1.5. Context-Aware Strings (New)
To eliminate entire classes of injection vulnerabilities, ASBEL introduces context-aware string literals. These are special string types that instruct the compiler to automatically apply the correct escaping for a given context, such as SQL, HTML, or JSON.
Syntax:
asbel

### 1.5. Context-Aware Strings (New)

To eliminate entire classes of injection vulnerabilities, ASBEL introduces context-aware string literals. These are special string types that instruct the compiler to automatically apply the correct escaping for a given context, such as SQL, HTML, or JSON.

**Syntax:**

```asbel
let user_input = "Robert'); DROP TABLE Students;--" // A classic injection attack
let safe_name = "O'Malley"

// The compiler understands the context and applies the correct escaping.
let query = sql"SELECT * FROM users WHERE name = \(user_input)"
// Transpiles to something like: "SELECT * FROM users WHERE name = 'Robert''); DROP TABLE Students;--'"

let html_output = html"<h1>Welcome, \(safe_name)!</h1>"
// Transpiles to: "<h1>Welcome, O&#39;Malley!</h1>"
```

This feature moves security-critical escaping from a manual, error-prone developer task into a guaranteed, compile-time transformation.

### 1.6. Resource-Bound Types (New)

Extending the concept of ownership beyond just memory, ASBEL supports **resource-bound types**. These are types annotated to tie a system resource (like a file handle, network socket, or GPU buffer) to a variable's scope, guaranteeing its release when the variable goes out of scope.

**Syntax:**

```asbel
// The @auto_close annotation guarantees the file is closed.
let file: File@auto_close = open("data.txt")!

// The @auto_release annotation guarantees the GPU memory is freed.
let buffer: GpuBuffer@auto_release(device) = allocate_gpu(1024)!

// The @timeout annotation could enforce a network timeout.
let socket: Socket@timeout(5s) = connect("server.com")!

// When 'file', 'buffer', or 'socket' go out of scope, their respective
// release functions are automatically called. No manual 'close()' is needed.
```

## 2. Functions

Functions are the fundamental building blocks for organizing code into reusable, logical units. ASBEL provides a clean, expressive syntax for function definitions, enhanced with advanced features like explicit error handling and Design by Contract to promote robust and verifiable software.

### 2.1. Function Definition

A function definition begins with the `fn` keyword, followed by the function name, a parameter list in parentheses, and an optional return type indicated by `->`. The function body is an indented block.

**General Syntax:**

```asbel
fn function_name(param1: Type1, param2: Type2) -> ReturnType
    // Function body
    // ...
```

If a function does not return a value, the return type can be omitted, implicitly returning a `Void` (or `()`) unit type. In ASBEL, the last expression in a function body is implicitly returned.

**Examples:**

```asbel
fn greet(name: String)
    println "Hello, \(name)!"

fn add(a: i32, b: i32) -> i32
    a + b // The result of this expression is returned

fn get_pi() -> f64
    3.14159
```

### 2.2. Positional and Named Arguments

ASBEL enforces a flexible rule for function call arguments designed to maximize clarity:

*   **One or two arguments:** May be passed positionally. This is convenient for common functions where the argument order is unambiguous.
*   **More than two arguments:** Must be passed using named arguments. This makes function calls self-documenting, eliminating ambiguity and improving maintainability, especially for functions with many configuration options.

**Examples:**

```asbel
// Function with one argument (positional)
fn increment(x: i32) -> i32
    x + 1

let result1 = increment(5)

// Function with two arguments (positional)
fn multiply(a: i32, b: i32) -> i32
    a * b

let result2 = multiply(3, 4)

// Function with more than two arguments (named)
fn create_user(name: String, email: String, is_admin: Bool) -> User
    // ... implementation ...

// Call with named arguments is clear and safe
let new_user = create_user(name: "Alice", email: "alice@example.com", is_admin: false)

// This would be a compile-time error:
// let bad_user = create_user("Bob", "bob@example.com", true) // Error: Too many positional arguments. Use named arguments.
```

This convention encourages clear, maintainable code and prevents a common source of bugs: passing arguments in the wrong order.

### 2.3. Error Handling with `Result`

ASBEL adopts an explicit and safe approach to error handling using the `Result` type. A function that can fail is marked with a `!` (exclamation mark) after its signature. This is a clear signal to the caller that the operation is fallible and its result must be handled.

The `Result<T, E>` type is an enum that can be either `Ok(T)` (indicating success with a value of type `T`) or `Err(E)` (indicating failure with an error of type `E`).

**Syntax:**

```asbel
fn fallible_function(param: Type) -> ReturnType!
    // ...
```

**Examples:**

```asbel
// Definition of a function that can fail
fn parse_int(s: String) -> i32!
    // Implementation that attempts to convert a string to an integer.
    // On success, returns Ok(integer)
    // On failure (e.g., "abc"), returns Err(ParseError)

// Using the function and handling the result
let num_str = "123"
let parsed_num = parse_int(num_str)

match parsed_num
    Ok(value) : println "Parsed number: \(value)"
    Err(error) : println "Parsing error: \(error)"

// The `?` operator for ergonomic error propagation (similar to Rust)
// If `parse_int` returns `Err`, `process_input` immediately returns that `Err`.
fn process_input(input: String) -> i32!
    let num = parse_int(input)? // Propagates the error if `parse_int` fails
    num * 2 // Continues with the unwrapped value on success
```

The `?` operator is a key ergonomic feature that simplifies error propagation. It unwraps `Ok` values and returns `Err` values from the current function, eliminating verbose `match` blocks for error handling and promoting cleaner code.

### 2.4. Contracts with `requires` and `ensures`

ASBEL integrates **Design by Contract** directly into the language with the `requires` and `ensures` keywords. This allows developers to specify preconditions and postconditions for functions.

*   **`requires` (Preconditions):** Specify conditions that must be true *before* the function executes. A failed precondition indicates a bug in the *caller's* code.
*   **`ensures` (Postconditions):** Specify conditions that must be true *after* the function has executed successfully. The return value can be referenced with the `result` keyword. A failed postcondition indicates a bug in the *function's implementation*.

**Syntax:**

```asbel
fn function_name(param: Type) -> ReturnType
    requires precondition1
    requires precondition2
    ensures postcondition_on_result(result)
    ensures postcondition_on_state
    // ... implementation ...
```

**Examples:**

```asbel
fn sqrt(x: f64) -> f64
    requires x >= 0.0 // Precondition: Square root is only defined for non-negative numbers.
    ensures (result * result - x).abs() < 1e-9 // Postcondition: The result squared must be close to x.
    // ... implementation of square root (e.g., Newton-Raphson) ...
    let mut guess = x / 2.0
    for _ in 0..10 // Fixed iterations for example simplicity
        guess = (guess + x / guess) / 2.0
    guess

fn divide(numerator: i32, denominator: i32) -> i32!
    requires denominator != 0 // Precondition: The divisor cannot be zero.
    // ... implementation ...
    if denominator == 0
        return Err(DivisionByZeroError) // This path is unreachable if the contract is enforced before execution
    Ok(numerator / denominator)

fn process_data(data: List<i32>) -> List<i32>
    requires data.len() > 0 // Precondition: Input list must not be empty.
    ensures result.len() == data.len() // Postcondition: Output list must have the same size.
    ensures result.all(|x| x >= 0) // Postcondition: All elements in the output must be non-negative.
    // ... implementation ...
```

Contracts are checked by the compiler or at runtime (depending on build settings and condition complexity). They provide a powerful way to document expected behavior, catch logical errors early, and improve the overall reliability of the software. When combined with refined types, contracts elevate the level of safety and correctness ASBEL can guarantee.

### 2.5. Advanced Error Handling and Context (New)

Beyond the `?` operator, ASBEL provides more powerful tools for annotating and handling errors.

**Error Context:** The `context` keyword allows developers to add descriptive information to an error as it propagates up the call stack, creating a clear chain of events without manual string formatting.

```asbel
fn load_user_data(user_id: u32) -> User!
    context "Failed to load data for user \(user_id)"

    let config_path = find_config_file()?
    let config = load_config(config_path).context("Configuration could not be parsed")?
    let db_conn = connect_to_db(config.db_url)?

    db_conn.fetch_user(user_id)?
```
If `fetch_user` fails with `NotFound`, the final error would be a rich, chained error like: `Failed to load data for user 123: Database query failed: NotFound`.

**Fluent Error Chaining:** For more complex recovery logic, errors can be handled with a fluent, chainable interface.

```asbel
let result = primary_operation()
    .or_else(|e| fallback_operation().context("Primary failed: \(e)"))
    .or_else(|e| last_resort().context("Fallback also failed: \(e)"))?
```

## 3. Lambdas and UFCS

ASBEL provides powerful tools for functional programming and code expressivity through lambdas (anonymous functions) and Universal Function Call Syntax (UFCS).

### 3.1. Lambdas (Anonymous Functions)

Lambdas, also known as anonymous functions or closures, are functions that can be defined inline without a name. They are especially useful for short operations passed as arguments to higher-order functions (like `map`, `filter`, `reduce`).

*   **Full Syntax:** `|| { ... }` for multi-line lambdas.
*   **Shorthand Syntax:** `|param1, param2| expression` for concise single-expression lambdas.

**Examples:**

```asbel
// Full syntax lambda
let greet_func = || {
    println "Hello from a lambda!"
}
greet_func() // Calling the lambda

// Shorthand syntax (one parameter)
let square = |x| x * x
println "The square of 5 is: \(square(5))" // Output: The square of 5 is: 25

// Using lambdas with higher-order functions
let numbers = [1, 2, 3, 4, 5]
let squares = numbers.map(|n| n * n) // [1, 4, 9, 16, 25]
println "Squares: \(squares)"

let even_numbers = numbers.filter(|n| n % 2 == 0) // [2, 4]
println "Even numbers: \(even_numbers)"

// Lambdas can capture their environment (closures)
let factor = 10
let multiply_by_factor = |x| x * factor // 'factor' is captured from the environment
println "5 multiplied by the factor: \(multiply_by_factor(5))" // Output: 50
```

### 3.2. UFCS (Universal Function Call Syntax)

ASBEL implements Universal Function Call Syntax (UFCS), which allows any free function that takes an instance of a type as its first argument to be called using method syntax (`.`). This unifies the calling convention for free functions and methods, improving code consistency and readability.

With UFCS, `object.method(arg1, arg2)` is syntactically equivalent to `method(object, arg1, arg2)`.

**Examples:**

```asbel
// A free function that calculates the length of a vector
fn length(vec: Vec2) -> f32
    (vec.x^2 + vec.y^2).sqrt()

type Vec2
    x: f32
    y: f32

let my_vec = Vec2(x: 3.0, y: 4.0)

// Traditional free function call
let len1 = length(my_vec)
println "Length (free function): \(len1)"

// Method-style call using UFCS
let len2 = my_vec.length()
println "Length (UFCS): \(len2)"

// Another example with a list
fn get_first_element<T>(list: List<T>) -> T!
    if list.is_empty()
        return Err(EmptyListError)
    Ok(list[0])

let my_list = [10, 20, 30]

// Calling as a free function
let first1 = get_first_element(my_list)?
println "First element (free function): \(first1)"

// Calling as a method using UFCS is more idiomatic and readable
let first2 = my_list.get_first_element()?
println "First element (UFCS): \(first2)"
```

UFCS contributes to ASBEL's goal of reducing cognitive load by providing a consistent, familiar way to interact with data and the functions that operate on it.

## 4. Control Flow

ASBEL provides modern, expressive control flow constructs designed to be powerful, safe, and highly readable.

### 4.1. `if/else` as an Expression

In ASBEL, `if/else` is an **expression**, meaning it evaluates to a value. This allows for more concise, functional-style code, eliminating the need for temporary mutable variables. Both branches of an `if/else` expression must evaluate to a value of the same type.

*   **Single-line expression:** `if condition: value_if_true else: value_if_false`
*   **Indented block expression:**
    ```asbel
    if condition:
        // Block of code for true case
        value_if_true
    else:
        // Block of code for false case
        value_if_false
    ```

**Examples:**

```asbel
// Single-line if/else expression
let label = if score > 90: "A" else: "B"
println "The grade is: \(label)"

// Block-based if/else expression
let message = if user.is_logged_in():
    "Welcome back, \(user.name)!"
else:
    "Please log in to continue."
println(message)

// Chained conditional assignment
let response_message = if status_code == 200:
    "Success"
else if status_code == 404:
    "Not Found"
else:
    "Unknown Error"
println "Response: \(response_message)"
```

### 4.2. `guard` for Early Exit

The `guard` statement is a control flow construct for validating conditions at the beginning of a scope (like a function) and exiting early if a condition is not met. This improves readability by avoiding deeply nested `if` statements and keeping the "happy path" of execution at the top level of indentation.

The syntax is `guard condition else: expression_or_block_on_failure`. If `condition` is `false`, the `else` block is executed, which must exit the current scope (e.g., via `return`, `break`, `continue`).

**Examples:**

```asbel
fn process_request(request: Request)
    // Validate that a user is present and an admin
    guard let user = request.user else:
        println "Error: No user in request."
        return // Exits the function

    guard user.is_admin else:
        println "Error: User is not an administrator."
        return // Exits the function

    // If we reach here, 'user' is guaranteed to be a valid admin.
    println "Processing request for admin: \(user.name)"
    // ... continue processing the request ...

fn divide_numbers(a: i32, b: i32) -> i32!
    guard b != 0 else:
        return Err(DivisionByZeroError)

    Ok(a / b)
```
### 4.3. `match` with Patterns and Ranges 

The `match` expression is a powerful pattern-matching construct. The compiler enforces **exhaustiveness**, ensuring all possible cases are handled.

*(...previous examples...)*

**Pattern Guards with Bindings:** For more complex conditions, `match` arms can include `if` and `if let` guards to conditionally execute a branch and bind variables simultaneously, avoiding nested logic.

```asbel
match request
    // Only matches if the method is POST and the body can be parsed as JSON.
    HttpRequest { method: "POST", body: Some(data) } if let Ok(json) = parse_json(data):
        handle_json_post(json)

    // Matches GET requests to the API path.
    HttpRequest { method: "GET", path } if path.starts_with("/api/"):
        handle_api_get(path)

    _ :
        handle_default(request)
```

The ability to use ranges directly in `match` patterns dramatically simplifies complex conditional logic, making code more readable and less error-prone than long chains of `if/else if` statements.

## 5. Loops and Ranges

ASBEL provides flexible and safe looping constructs for iteration and repetition, complemented by an expressive syntax for defining ranges.

### 5.1. Iteration Loops (`for`)

The `for` loop is designed for safe and convenient iteration over collections. It iterates directly over the elements of an iterator, eliminating the possibility of off-by-one or out-of-bounds errors common with manual index management.

**General Syntax:**

```asbel
for item in collection:
    // Code to execute for each item
    // ...
```

**Examples:**

```asbel
let numbers = [10, 20, 30, 40, 50]
var total = 0
for num in numbers:
    total += num
println "Sum of numbers: \(total)" // Output: Sum of numbers: 150

// Iterating over a range (see section 5.4)
for i in 0..5:
    println "Iteration: \(i)" // Prints 0, 1, 2, 3, 4

// Iterating with an index using an `enumerate` method
for (index, value) in names.enumerate():
    println "Item at index \(index): \(value)"
```

### 5.2. Conditional Loops (`while`)

The `while` loop executes a block of code repeatedly as long as a boolean condition remains true. It is suitable for situations where the number of iterations is not known in advance.

**Syntax:**

```asbel
while condition:
    // Code to execute while condition is true
    // ...
```

**Example:**

```asbel
var count = 5
while count > 0:
    println "Countdown: \(count)"
    count -= 1
println "Liftoff!"
```

### 5.3. Infinite Loops (`loop`)

The `loop` keyword creates an infinite loop that executes indefinitely until explicitly terminated with a `break` or `return` statement. It is useful for services, event loops, or any process that must run continuously.

**Syntax:**

```asbel
loop:
    // Code to execute indefinitely
    // ...
```

**Example:**

```asbel
var i = 0
loop:
    println "Looping forever, iteration: \(i)"
    i += 1
    if i >= 3:
        break // Exits the loop when i reaches 3
```

### 5.4. Ranges

Ranges are a concise way to represent a sequence of values. They are widely used in `for` loops and `match` expressions.

*   **Exclusive Range (`start..end`):** Includes `start` but excludes `end`.
*   **Inclusive Range (`start..=end`):** Includes both `start` and `end`.
*   **Step (`by step`):** An optional modifier to specify an increment or decrement.

**Syntax and Examples:**

```asbel
// Exclusive range (0 to 9)
for i in 0..10:
    print "\(i) " // Output: 0 1 2 3 4 5 6 7 8 9

// Inclusive range (1 to 10)
for i in 1..=10:
    print "\(i) " // Output: 1 2 3 4 5 6 7 8 9 10

// Range with a step (0 to 100, by 10s)
for i in 0..=100 by 10:
    print "\(i) " // Output: 0 10 20 ... 100

// Descending range
for i in 5..=1 by -1:
    print "\(i) " // Output: 5 4 3 2 1
```

## 6. Pipelines

ASBEL elevates **pipelines** to a first-class language feature to improve the readability and fluency of chained operations. Pipelines allow data to flow from left-to-right, making complex data transformations intuitive and easy to follow. This design is inspired by functional languages and command-line shells.

*   **Inline Flow (`|>`):** The pipe operator for chaining sequential operations.
*   **Named Flow (`=>`):** The assignment operator for naming an intermediate result within a pipeline.

### 6.1. Inline Flow (`|>`) - The Pipe Operator

The pipe operator (`|>`) takes the result of the expression on its left and passes it as the *first argument* to the function or method on its right. This is extremely effective for transforming data through a series of steps.

**General Syntax:**

```asbel
initial_value |> function1(arg1) |> function2() |> method3()
```

**Examples:**

```asbel
// Without pipelines (harder to read, nested calls)
let result_nested = process(filter(normalize(input_data), 10))

// With pipelines (clear, left-to-right data flow)
let result_piped = input_data
    |> normalize()
    |> filter(10)
    |> process()

// Practical example: Text processing
let text = "  ASBEL is a simple but efficient language.  "
let processed_text = text
    |> String.trim()          // "ASBEL is a simple but efficient language."
    |> String.to_lower()      // "asbel is a simple but efficient language."
    |> String.replace("simple", "powerful")
    |> String.capitalize_first() // "Asbel is a powerful but efficient language."

println "Processed: \(processed_text)"
```

### 6.2. Named Flow (`=>`) - Pipeline Assignment

The named flow operator (`=>`) allows you to assign an intermediate result within a pipeline to a named variable. This is useful for:

*   **Clarity:** Naming a significant intermediate step to improve understanding.
*   **Reuse:** Using an intermediate result in multiple subsequent operations.
*   **Debugging:** Easily inspecting values at specific points in the flow.

**Syntax:**

```asbel
initial_value
    |> step1()
    |> step2()
    => meaningful_intermediate_result

// Now use the named result
meaningful_intermediate_result |> step3()
```

**Example:**

```asbel
// Data processing with a named intermediate step
let raw_data = get_sensor_readings()

raw_data
    |> normalize_values()
    |> remove_outliers(threshold: 0.05)
    => cleaned_data // The result of remove_outliers is now bound to 'cleaned_data'

// Now, 'cleaned_data' can be used in multiple subsequent pipelines
let report = cleaned_data
    |> generate_summary_report()
    |> format_as_pdf()

let db_result = cleaned_data
    |> aggregate_metrics()
    |> save_to_database()!

println "Report generated: \(report.path)"
println "Database result: \(db_result)"
```

## 7. Structs, Interfaces, and Hygienic Macros

ASBEL provides robust mechanisms for abstraction and code reuse through user-defined types (`type`), interfaces for shared behavior, and a safe macro system.

### 7.1. Structs (`type`)

Structs, declared with the `type` keyword, are custom composite data types that group related fields. They are the foundation for creating custom data models. By default, structs are value types (copied on assignment), but they can be passed by reference to avoid copies.

**Syntax:**

```asbel
type StructName
    field1: Type1
    field2: Type2
    // ...
```

**Example:**

```asbel
type Vec2
    x: f32
    y: f32

type Person
    name: String
    age: u8(0..=150) // Using a refined type!
    is_active: Bool

let point = Vec2(x: 10.5, y: -3.2)
let user = Person(name: "Aldrete", age: 30, is_active: true)

println "User: \(user.name), Age: \(user.age)"
```

### 7.2. Implementations (`impl`)

The `impl` block is used to define methods associated with a specific type. Methods are functions that operate on an instance of the type, typically taking `self` as their first parameter.

*   `&self`: An immutable reference to the instance (the method cannot modify it).
*   `&mut self`: A mutable reference to the instance (the method can modify it).
*   `self`: Takes ownership of the instance (it is consumed by the method).
*   No `self`: An associated function (like a static method), often used for constructors.

**Example:**

```asbel
type Vec2
    x: f32
    y: f32

impl Vec2
    // Associated function (constructor)
    fn new(x: f32, y: f32) -> Vec2
        Vec2(x: x, y: y)

    // Method taking an immutable reference
    fn length(&self) -> f32
        (self.x^2 + self.y^2).sqrt()

    // Method taking a mutable reference
    fn normalize(&mut self)
        let len = self.length()
        if len > 0.0:
            self.x /= len
            self.y /= len

let mut vec = Vec2.new(x: 3.0, y: 4.0)
println "Initial length: \(vec.length())" // Output: 5.0
vec.normalize()
println "Normalized vector: (\(vec.x), \(vec.y))" // Output: (0.6, 0.8)
```

### 7.3. Interfaces (`interface`)

Interfaces (similar to Rust's traits or Haskell's typeclasses) define a set of behaviors that types can implement. An interface specifies a contract of method signatures that a type must provide to conform to that interface. This enables polymorphism and code reuse across different types that share common functionality.

**Syntax:**

```asbel
interface InterfaceName
    fn required_method(&self, param: Type) -> ReturnType
    // ...
```

**Example:**

```asbel
interface Serializable
    fn to_json(&self) -> String

// Implement the interface for the Person type
impl Serializable for Person
    fn to_json(&self) -> String
        // Simplified JSON generation
        "{\"name\": \"\(self.name)\", \"age\": \(self.age)}"

// Implement it for Vec2 as well
impl Serializable for Vec2
    fn to_json(&self) -> String
        "{\"x\": \(self.x), \"y\": \(self.y)}"

// A generic function that works with any type implementing Serializable
fn print_json(item: impl Serializable)
    println(item.to_json())

let user = Person(name: "Alice", age: 30, is_active: true)
let point = Vec2(x: 1, y: 2)

print_json(user)  // Prints {"name": "Alice", "age": 30}
print_json(point) // Prints {"x": 1, "y": 2}
```

### 7.4. Metaprogramming: Hygienic Macros and Derivations (Enhanced)

**Hygienic Macros:** 

ASBEL includes a powerful macro system for compile-time code generation. Unlike C/C++ preprocessor macros, ASBEL macros are **hygienic**, meaning they avoid common pitfalls like variable capture and name collisions. This is achieved by ensuring that variables introduced by a macro do not interfere with variables in the context where the macro is expanded.

Macros are written in ASBEL itself, allowing developers to abstract away boilerplate code and create Domain-Specific Languages (DSLs) safely and predictably.

**Conceptual Example:**

```asbel
// A macro to automatically generate getter and setter methods
macro with_accessors(TypeName, field_name, FieldType) {
    impl TypeName
        fn get_{field_name}(&self) -> FieldType
            self.{field_name}

        fn set_{field_name}(&mut self, value: FieldType)
            self.{field_name} = value
}

// Using the macro
type MyData
    value: i32

with_accessors!(MyData, value, i32) // Macro expands here at compile time

let mut data = MyData(value: 10)
println "Initial value: \(data.get_value())" // 10
data.set_value(20)
println "New value: \(data.get_value())" // 20
```

**Compile-Time Derivations:** To eliminate common boilerplate, ASBEL provides a `@derive` attribute that uses the macro system to automatically generate standard method implementations based on a type's structure.

```asbel
@derive(Serialize, Deserialize, Debug, Hash, Eq, Builder)
type User
    name: String
    age: u8(0..=150)
    email: String

// The compiler will automatically generate:
// - to_json() and from_json() methods (from Serialize/Deserialize)
// - A readable string representation for debugging (from Debug)
// - Hashing and equality logic (from Hash, Eq)
// - A full builder pattern for constructing User instances:
//   let user = User.builder().name("Alice").age(30).build()!
```

## 8. Foreign Function Interface (FFI) with C

The ability to interface with existing code is critical. ASBEL provides a robust and safe Foreign Function Interface (FFI) for interoperating with C libraries, which is a natural fit given ASBEL's C-transpilation strategy. This allows ASBEL developers to leverage the vast ecosystem of existing C libraries for systems programming, hardware interaction, and performance-critical tasks.

### 8.1. Declaring External C Functions (`use c`)

To call C functions, ASBEL uses the `use c` directive, which instructs the compiler to link against a C library and make its functions available.

**Syntax:**

```asbel
// Import specific functions from a C header
use c "stdio.h": printf, fprintf

// Import all functions (use with caution to avoid name clashes)
use c "math.h": *
```

**Example:**

```asbel
use c "stdio.h": printf

fn main()
    // The string must be C-compatible (null-terminated).
    // The standard library will provide safe ways to create these.
    let c_str = "Hello from ASBEL, calling C!\n".to_c_string()
    unsafe: // Calling C functions is inherently unsafe
        printf(c_str)
```

### 8.2. C Data Types and Conversion

ASBEL provides a set of C-compatible types (e.g., `c_int`, `c_char`, `c_ptr<T>`) for seamless interoperability. The language provides safe mechanisms for converting between ASBEL types and their C equivalents.

### 8.3. Exporting ASBEL Functions to C (`export c fn`)

ASBEL can also be compiled into a library that C code can call. This is useful for incrementally migrating a C codebase to ASBEL or for writing high-performance components in ASBEL for use in other applications. Functions are exposed using the `export c fn` syntax.

**Example:**

```asbel
// in asbel_library.asbel

// This function will be callable from C
export c fn add_in_asbel(a: c_int, b: c_int) -> c_int
    a + b
```

When compiled as a library, ASBEL will generate both the shared object (`.so`, `.dll`) and a corresponding C header file (`.h`) for easy integration.

### 8.4. Safety and `unsafe`

Interacting with C is inherently unsafe because C does not provide the memory safety guarantees that ASBEL does. Any operation that involves raw pointers or calling C functions must be wrapped in an `unsafe` block. This block is a signal to the developer that they are responsible for upholding the safety invariants that the ASBEL compiler can no longer verify. ASBEL's design goal is to minimize the surface area of `unsafe` code, confining it to the FFI boundary.

## 9. Tooling and Ecosystem

A productive language is supported by a great ecosystem. ASBEL is committed to providing a comprehensive, first-class tooling experience from the outset.

### 9.1. The `asbel` CLI

The `asbel` command-line interface is the central hub for all development tasks:

*   `asbel init`: Creates a new ASBEL project.
*   `asbel build`: Compiles the current project, transpiling to C and then invoking the C compiler.
*   `asbel run`: Builds and runs the project.
*   `asbel test`: Runs all unit and integration tests.
*   `asbel add [package]`: Adds a new dependency to the project.
*   `asbel fmt`: Formats all ASBEL code according to a canonical style guide, ensuring consistency.
*   `asbel doc`: Generates HTML documentation from source code comments.
*   `asbel repl`: Starts an interactive Read-Eval-Print Loop for experimenting with the language.

### 9.2. Language Server Protocol (LSP)

ASBEL will provide a full-featured Language Server (`asbel-lsp`) for tight integration with modern code editors like VS Code, Neovim, and others. This will enable:

*   Intelligent, context-aware autocompletion.
*   Real-time diagnostics and error checking.
*   Go-to-definition, find-references, and symbol renaming.
*   Inline documentation and type information on hover.

### 9.3. WASM Playground

An online playground, compiled to WebAssembly, will allow anyone to try ASBEL in their browser without any local installation. This is a key tool for education, experimentation, and sharing code snippets.

### 9.4. Contract Verifier

A tool, integrated into the compiler and test runner, will be responsible for checking the `requires` and `ensures` contracts. It will support:

*   **Static verification:** Proving contracts at compile-time where possible.
*   **Runtime checking:** Inserting checks for dynamic contracts, which can be enabled for debug/test builds.
*   **Fuzzing:** Generating inputs to test contract boundaries and find edge cases.

---

### 9.5. Integrated Debugging Hooks (New)

ASBEL provides language-level support for advanced debugging. The `debug_snapshot()` function allows developers to capture the state of variables at specific points, which can be consumed by integrated tooling for time-travel debugging.

```asbel
fn complex_algorithm(data: List<i32>) -> i32
    let step1 = data |> normalize()
    debug_snapshot("After normalization", step1)

    let step2 = step1 |> complex_transform()
    debug_snapshot("After transform", step2)

    // In debug mode, a tool can step between these snapshots.
    step2.sum()
```

## 10. Concurrency and Parallelism (New)

ASBEL is designed for modern multi-core architectures with safe and easy-to-use concurrency primitives.

**Lightweight Tasks:** The `spawn` keyword launches a new concurrent task, similar to a goroutine or lightweight thread. ASBEL's ownership system guarantees thread safety by preventing data races at compile time.

```asbel
spawn || handle_client(client)
```

**Automatic Parallelization Hints:** For data-parallel problems, developers can provide hints to the compiler with the `@parallel` attribute, allowing ASBEL to automatically distribute work across available CPU cores when it can prove the operation is safe.

```asbel
let results = data
    |> filter(|x| x > 0)     @parallel  // This filter can run on multiple threads.
    |> map(expensive_compute) @parallel  // This map operation is also parallelized.
    |> reduce(|a, b| a + b)   @sequential // This final reduction is sequential.
```

## 11. Integrated Testing Framework (New)

In ASBEL, testing is not a library or an afterthought; it is a first-class construct built into the language. This encourages developers to write tests alongside their code.

**Embedded Test Cases:** Simple unit tests can be embedded directly within a function's definition block.

**Property-Based Testing:** For more robust validation, ASBEL supports property-based tests, which define logical properties that must hold true for all valid inputs. The test runner will generate hundreds of inputs to try to falsify the property.

```asbel
fn factorial(n: u32) -> u32
    requires n <= 12 // A contract to prevent overflow

    // Simple, embedded unit tests
    test_cases:
        factorial(0) == 1
        factorial(5) == 120

    // A property that must hold for all inputs in the range
    test_property forall x in 0..=12:
        factorial(x) > 0 || x == 0

    // Function implementation
    match n
        0 | 1 : 1
        _ : n * factorial(n - 1)
```
Running `asbel test` will automatically discover and execute all embedded `test_cases` and `test_property` blocks throughout the project.

## 12. Future Directions and Experimental Features (New)

ASBEL is designed with the future in mind. The following are experimental ideas being explored for future versions of the language:

*   **Probabilistic Types:** For the age of AI, types that can natively represent uncertainty. `let confidence: f32~(0.8) = model.predict()`.
*   **Time-Aware Types:** For temporal programming and complex event processing, variables that natively store their history. `let price: f64@history(5m) = get_stock_price()`.
*   **Visual Pattern Matching:** For multi-dimensional data, using an ASCII-art-like syntax to match on the structure of data like matrices or images.

---
