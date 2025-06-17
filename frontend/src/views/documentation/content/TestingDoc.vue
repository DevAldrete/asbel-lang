<template>
  <article class="prose prose-lg max-w-none lg:prose-xl xl:prose-2xl text-gray-700">
    <h2 class="text-3xl font-bold mb-6 text-gray-800 border-b pb-2">11. Integrated Testing Framework</h2>
    <p>In ASBEL, testing is a first-class construct built into the language, encouraging developers to write tests alongside their code.</p>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">Embedded Test Cases</h3>
    <p>Simple unit tests can be embedded directly within a function's definition block using <code>test_cases</code>.</p>
    <pre><code class="language-asbel">
fn factorial(n: u32) -> u32
    requires n <= 12 // Contract to prevent overflow

    // Simple, embedded unit tests
    test_cases:
        factorial(0) == 1
        factorial(1) == 1
        factorial(5) == 120

    // Function implementation
    match n
        0 | 1 : 1
        _ : n * factorial(n - 1)

// Running 'asbel test' will execute these.
    </code></pre>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">Property-Based Testing</h3>
    <p>ASBEL supports property-based tests (<code>test_property</code>), which define logical properties that must hold true for all valid inputs. The test runner generates inputs to try to falsify the property.</p>
    <pre><code class="language-asbel">
fn abs_is_positive_or_zero(val: i32) -> i32
    if val < 0: -val else: val

// A property that must hold for all integer inputs
test_property forall x in i32: // 'i32' here implies a range of generated integers
    abs_is_positive_or_zero(x) >= 0

// Example with a specific range for property testing
fn is_even(n: i32) -> Bool
    n % 2 == 0

test_property forall x in 0..1000 if is_even(x):
    (x + 1) % 2 != 0 // If x is even, x+1 must be odd

// 'asbel test' discovers and runs these properties.
    </code></pre>
    <p>The <code>asbel test</code> command automatically discovers and executes all embedded <code>test_cases</code> and <code>test_property</code> blocks throughout the project, providing a comprehensive and integrated testing experience.</p>
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
