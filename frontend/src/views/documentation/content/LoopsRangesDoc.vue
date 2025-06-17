<template>
  <article class="prose prose-lg max-w-none lg:prose-xl xl:prose-2xl text-gray-700">
    <h2 class="text-3xl font-bold mb-6 text-gray-800 border-b pb-2">5. Loops and Ranges</h2>
    <p>ASBEL provides flexible and safe looping constructs for iteration and repetition, complemented by an expressive syntax for defining ranges.</p>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">5.1. Iteration Loops (<code>for</code>)</h3>
    <p>The <code>for</code> loop iterates directly over elements of a collection or range, eliminating off-by-one errors.</p>
    <pre><code class="language-asbel">
let numbers = [10, 20, 30, 40, 50]
var total = 0
for num in numbers:
    total += num
println "Sum: \(total)" // Output: Sum: 150

// Iterating over a range
for i in 0..5: // 0, 1, 2, 3, 4
    println "Iteration: \(i)"

// Iterating with index
for (index, value) in names.enumerate():
    println "Item at \(index): \(value)"
    </code></pre>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">5.2. Conditional Loops (<code>while</code>)</h3>
    <p>The <code>while</code> loop executes as long as a boolean condition is true.</p>
    <pre><code class="language-asbel">
var count = 5
while count > 0:
    println "Countdown: \(count)"
    count -= 1
println "Liftoff!"
    </code></pre>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">5.3. Infinite Loops (<code>loop</code>)</h3>
    <p>The <code>loop</code> keyword creates an infinite loop, exited with <code>break</code> or <code>return</code>.</p>
    <pre><code class="language-asbel">
var i = 0
loop:
    println "Looping: \(i)"
    i += 1
    if i >= 3:
        break
    </code></pre>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">5.4. Ranges</h3>
    <p>Ranges represent sequences of values. Used in <code>for</code> loops and <code>match</code> expressions.</p>
    <ul>
      <li>Exclusive Range (<code>start..end</code>): Includes <code>start</code>, excludes <code>end</code>.</li>
      <li>Inclusive Range (<code>start..=end</code>): Includes both <code>start</code> and <code>end</code>.</li>
      <li>Step (<code>by step</code>): Optional increment/decrement.</li>
    </ul>
    <pre><code class="language-asbel">
for i in 0..10: print "\(i) "         // 0 to 9
for i in 1..=10: print "\(i) "        // 1 to 10
for i in 0..=100 by 10: print "\(i) " // 0, 10, ..., 100
for i in 5..=1 by -1: print "\(i) "   // 5, 4, 3, 2, 1
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
