<template>
  <article class="prose prose-lg max-w-none lg:prose-xl xl:prose-2xl text-gray-700">
    <h2 class="text-3xl font-bold mb-6 text-gray-800 border-b pb-2">1. Variables and Types</h2>

    <p>ASBEL provides an intuitive and flexible variable declaration system with a strong emphasis on type inference and safety. The language clearly distinguishes between immutable and mutable variables, promoting a programming style that reduces side effects and improves code predictability.</p>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">1.1. Variable Declaration</h3>
    <p>ASBEL uses two primary keywords for variable declaration:</p>
    <ul>
      <li><code>let</code>: Declares an <strong>immutable</strong> variable. Once a value is assigned to a <code>let</code> binding, it cannot be modified.</li>
      <li><code>var</code>: Declares a <strong>mutable</strong> variable. <code>var</code> bindings can be reassigned to new values after their initial declaration.</li>
    </ul>
    <p>Type inference is a cornerstone of ASBEL. The compiler automatically deduces a variable's type from its assigned value.</p>
    <pre><code class="language-asbel">
let name = "ASBEL"         // 'name' is an immutable String
var counter = 0             // 'counter' is a mutable i32 (default integer type)
let pi = 3.14159            // 'pi' is an immutable f64 (default float type)
var is_active = true        // 'is_active' is a mutable Bool
    </code></pre>
    <p>Explicit type annotations are supported for clarity:</p>
    <pre><code class="language-asbel">
let count: i64 = 100_000_000_000 // 'count' is an immutable i64
var temperature: f32 = 25.5      // 'temperature' is a mutable f32
    </code></pre>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">1.2. Refined Types and Ranges</h3>
    <p>ASBEL supports <strong>refined types</strong>, allowing value constraints within type declarations (e.g., <code>Type(min..=max)</code>).</p>
    <pre><code class="language-asbel">
let pct: u8(0..=100) = 42
var angle: f32(0..=360) = 270.0
let positive_value: i32(1..) = 5
    </code></pre>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">1.3. Primitive Data Types</h3>
    <ul>
      <li><strong>Integers:</strong> <code>i8</code>, <code>i16</code>, <code>i32</code>, <code>i64</code>, <code>i128</code>, <code>u8</code>, <code>u16</code>, <code>u32</code>, <code>u64</code>, <code>u128</code>. Default: <code>i32</code>.</li>
      <li><strong>Floating-Point:</strong> <code>f32</code>, <code>f64</code>. Default: <code>f64</code>.</li>
      <li><strong>Boolean:</strong> <code>Bool</code> (<code>true</code>, <code>false</code>).</li>
      <li><strong>Character:</strong> <code>Char</code> (e.g., <code>'A'</code>, <code>'😊'</code>).</li>
      <li><strong>String:</strong> <code>String</code> (UTF-8, immutable by default, e.g., <code>"Hello"</code>).</li>
    </ul>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">1.4. Literals</h3>
    <p>Supports integer (<code>1_000</code>), float (<code>1.2e-5</code>), boolean, character, and string literals.</p>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">1.5. Context-Aware Strings</h3>
    <p>Eliminate injection vulnerabilities with context-aware string literals (e.g., <code>sql"..."</code>, <code>html"..."</code>).</p>
    <pre><code class="language-asbel">
let user_input = "Robert'); DROP TABLE Students;--"
let query = sql"SELECT * FROM users WHERE name = \(user_input)"
// Transpiles to something like: "SELECT * FROM users WHERE name = 'Robert''); DROP TABLE Students;--'"
    </code></pre>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">1.6. Resource-Bound Types</h3>
    <p>Tie system resources to a variable's scope using annotations like <code>@auto_close</code>.</p>
    <pre><code class="language-asbel">
let file: File@auto_close = open("data.txt")!
// When 'file' goes out of scope, its release function is automatically called.
    </code></pre>
  </article>
</template>
<script setup></script>
<style scoped>
.prose :where(code):not(:where([class~="not-prose"] *))::before,
.prose :where(code):not(:where([class~="not-prose"] *))::after {
  content: '';
}
.prose code {
  @apply bg-gray-200 text-sm px-1 py-0.5 rounded;
}
.prose pre {
  @apply bg-gray-800 text-white p-4 rounded-md overflow-x-auto;
  font-family: 'Courier New', Courier, monospace;
}
.prose pre code {
  @apply bg-transparent p-0 text-sm;
}
</style>
