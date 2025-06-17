<template>
  <article class="prose prose-lg max-w-none lg:prose-xl xl:prose-2xl text-gray-700">
    <h2 class="text-3xl font-bold mb-6 text-gray-800 border-b pb-2">2. Functions</h2>
    <p>Functions are fundamental for organizing code. ASBEL offers clean syntax with advanced features like explicit error handling and Design by Contract.</p>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">2.1. Function Definition</h3>
    <p>Use <code>fn</code>, followed by name, parameters, and optional return type (<code>-> ReturnType</code>). The last expression is implicitly returned.</p>
    <pre><code class="language-asbel">
fn greet(name: String)
    println "Hello, \(name)!"

fn add(a: i32, b: i32) -> i32
    a + b // Returned

fn get_pi() -> f64
    3.14159
    </code></pre>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">2.2. Positional and Named Arguments</h3>
    <p>One or two arguments can be positional. More than two require named arguments for clarity.</p>
    <pre><code class="language-asbel">
fn increment(x: i32) -> i32
    x + 1
let result1 = increment(5)

fn create_user(name: String, email: String, is_admin: Bool) -> User
    // ...
let new_user = create_user(name: "Alice", email: "alice@example.com", is_admin: false)
// create_user("Bob", "bob@example.com", true) // Error: Use named arguments.
    </code></pre>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">2.3. Error Handling with <code>Result</code></h3>
    <p>Functions that can fail are marked with <code>!</code> and return <code>Result&lt;T, E&gt;</code> (<code>Ok(T)</code> or <code>Err(E)</code>). Use <code>?</code> for ergonomic error propagation.</p>
    <pre><code class="language-asbel">
fn parse_int(s: String) -> i32!
    // ... returns Ok(integer) or Err(ParseError)

let parsed_num = parse_int("123")
match parsed_num
    Ok(value) : println "Parsed: \(value)"
    Err(error) : println "Error: \(error)"

fn process_input(input: String) -> i32!
    let num = parse_int(input)? // Propagates error
    num * 2
    </code></pre>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">2.4. Contracts with <code>requires</code> and <code>ensures</code></h3>
    <p>Specify preconditions (<code>requires</code>) and postconditions (<code>ensures</code>).</p>
    <pre><code class="language-asbel">
fn sqrt(x: f64) -> f64
    requires x >= 0.0
    ensures (result * result - x).abs() < 1e-9
    // ... implementation ...
    let mut guess = x / 2.0
    for _ in 0..10
        guess = (guess + x / guess) / 2.0
    guess
    </code></pre>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">2.5. Advanced Error Handling and Context</h3>
    <p>Use <code>context</code> to add descriptive information to errors as they propagate.</p>
    <pre><code class="language-asbel">
fn load_user_data(user_id: u32) -> User!
    context "Failed to load data for user \(user_id)"
    let config = load_config("path").context("Config parsing failed")?
    // ...
    db_conn.fetch_user(user_id)?
    </code></pre>
  </article>
</template>
<script setup></script>
<style scoped>
.prose :where(code):not(:where([class~="not-prose"] *))::before,
.prose :where(code):not(:where([class~="not-prose"] *))::after { content: ''; }
.prose code { @apply bg-gray-200 text-sm px-1 py-0.5 rounded; }
.prose pre { @apply bg-gray-800 text-white p-4 rounded-md overflow-x-auto; font-family: 'Courier New', Courier, monospace; }
.prose pre code { @apply bg-transparent p-0 text-sm; }
</style>
