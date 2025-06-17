<template>
  <article class="prose prose-lg max-w-none lg:prose-xl xl:prose-2xl text-gray-700">
    <h2 class="text-3xl font-bold mb-6 text-gray-800 border-b pb-2">3. Lambdas and UFCS</h2>
    <p>ASBEL provides powerful tools for functional programming and code expressivity through lambdas (anonymous functions) and Universal Function Call Syntax (UFCS).</p>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">3.1. Lambdas (Anonymous Functions)</h3>
    <p>Lambdas are functions defined inline without a name, useful for short operations passed to higher-order functions. Syntax: <code>|| { ... }</code> or <code>|params| expression</code>.</p>
    <pre><code class="language-asbel">
let square = |x| x * x
println "Square of 5: \(square(5))" // Output: 25

let numbers = [1, 2, 3, 4, 5]
let squares = numbers.map(|n| n * n)
println "Squares: \(squares)" // [1, 4, 9, 16, 25]

let factor = 10
let multiply_by_factor = |x| x * factor // Captures 'factor'
println "5 * factor: \(multiply_by_factor(5))" // Output: 50
    </code></pre>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">3.2. UFCS (Universal Function Call Syntax)</h3>
    <p>Allows any free function that takes an instance of a type as its first argument to be called using method syntax (<code>object.method()</code> is equivalent to <code>method(object)</code>).</p>
    <pre><code class="language-asbel">
type Vec2
    x: f32
    y: f32

fn length(vec: Vec2) -> f32 // Free function
    (vec.x^2 + vec.y^2).sqrt()

let my_vec = Vec2(x: 3.0, y: 4.0)

let len1 = length(my_vec)         // Traditional call
let len2 = my_vec.length()        // UFCS call
println "Length (UFCS): \(len2)"
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
