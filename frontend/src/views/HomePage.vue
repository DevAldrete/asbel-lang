<template>
  <div class="bg-gray-50 text-gray-800">
    <!-- Hero Section -->
    <section class="bg-gradient-to-r from-gray-700 via-gray-800 to-gray-900 text-white py-20 md:py-32">
      <div class="container mx-auto px-6 text-center">
        <img src="/img/firefly-logo-placeholder.svg" alt="ASBEL Firefly Mascot" class="h-24 w-24 md:h-32 md:w-32 mx-auto mb-6 transition-transform duration-500 hover:scale-110" />
        <h1 class="text-4xl md:text-6xl font-bold mb-4 leading-tight">ASBEL</h1>
        <p class="text-xl md:text-2xl font-light mb-8">Write like Python – Run like C – Be safe like Rust</p>
        <div class="space-x-4">
          <router-link to="/getting-started"
            class="bg-yellow-500 hover:bg-yellow-400 text-gray-900 font-semibold py-3 px-8 rounded-lg text-lg transition-colors duration-300 shadow-lg hover:shadow-xl transform hover:scale-105">
            Get Started
          </router-link>
          <router-link to="/documentation"
            class="bg-transparent hover:bg-gray-700 text-white font-semibold py-3 px-8 rounded-lg text-lg border-2 border-white transition-colors duration-300 shadow-lg hover:shadow-xl transform hover:scale-105">
            Read the Docs
          </router-link>
        </div>
      </div>
    </section>

    <!-- Introduction Section -->
    <section class="py-16 md:py-24">
      <div class="container mx-auto px-6 text-center">
        <h2 class="text-3xl md:text-4xl font-bold mb-12 text-gray-800">What is ASBEL?</h2>
        <p class="text-lg md:text-xl text-gray-600 max-w-3xl mx-auto leading-relaxed">
          ASBEL (A Simple But Efficient Language) is a next-generation systems programming language
          that aims to combine the ergonomics of high-level scripting languages with the raw performance
          and predictability of low-level languages. Written in Rust, the ASBEL compiler transpiles
          to highly-optimized C, allowing it to leverage decades of mature C toolchains while enforcing
          modern safety guarantees and powerful abstractions.
        </p>
      </div>
    </section>

    <!-- Why ASBEL? Section -->
    <section class="bg-white py-16 md:py-24">
      <div class="container mx-auto px-6">
        <h2 class="text-3xl md:text-4xl font-bold text-center mb-16 text-gray-800">Why Choose ASBEL?</h2>
        <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8 lg:gap-12">
          <div class="p-6 bg-gray-50 rounded-xl shadow-lg hover:shadow-2xl transition-shadow duration-300 transform hover:-translate-y-1">
            <h3 class="text-2xl font-semibold mb-3 text-gray-700">Transparent Abstraction</h3>
            <p class="text-gray-600 leading-relaxed">What you write maps directly to the generated binary. No hidden allocations, no magic costs. Performance analysis is intuitive.</p>
          </div>
          <div class="p-6 bg-gray-50 rounded-xl shadow-lg hover:shadow-2xl transition-shadow duration-300 transform hover:-translate-y-1">
            <h3 class="text-2xl font-semibold mb-3 text-gray-700">Inferred Ownership & Borrowing</h3>
            <p class="text-gray-600 leading-relaxed">Memory safety without the steep learning curve of manual lifetime annotations. Focus on your logic, not the memory manager.</p>
          </div>
          <div class="p-6 bg-gray-50 rounded-xl shadow-lg hover:shadow-2xl transition-shadow duration-300 transform hover:-translate-y-1">
            <h3 class="text-2xl font-semibold mb-3 text-gray-700">Refined & Resource-Bound Types</h3>
            <p class="text-gray-600 leading-relaxed">Encode value ranges (e.g., <code class="bg-gray-200 p-1 rounded text-sm">u8(0..=100)</code>) or tie system resources to scope (e.g., <code class="bg-gray-200 p-1 rounded text-sm">File@auto_close</code>) directly in the type system.</p>
          </div>
          <div class="p-6 bg-gray-50 rounded-xl shadow-lg hover:shadow-2xl transition-shadow duration-300 transform hover:-translate-y-1">
            <h3 class="text-2xl font-semibold mb-3 text-gray-700">Design by Contract</h3>
            <p class="text-gray-600 leading-relaxed">Use <code class="bg-gray-200 p-1 rounded text-sm">requires</code> and <code class="bg-gray-200 p-1 rounded text-sm">ensures</code> clauses to build verifiable, robust software, catching logic errors early.</p>
          </div>
          <div class="p-6 bg-gray-50 rounded-xl shadow-lg hover:shadow-2xl transition-shadow duration-300 transform hover:-translate-y-1">
            <h3 class="text-2xl font-semibold mb-3 text-gray-700">First-Class Pipelines</h3>
            <p class="text-gray-600 leading-relaxed">Chain data transformations elegantly with the <code class="bg-gray-200 p-1 rounded text-sm">|&gt;</code> and <code class="bg-gray-200 p-1 rounded text-sm">=&gt;</code> operators, making complex flows easy to read.</p>
          </div>
          <div class="p-6 bg-gray-50 rounded-xl shadow-lg hover:shadow-2xl transition-shadow duration-300 transform hover:-translate-y-1">
            <h3 class="text-2xl font-semibold mb-3 text-gray-700">Modern Tooling & Concurrency</h3>
            <p class="text-gray-600 leading-relaxed">A canonical CLI, LSP support, lightweight <code class="bg-gray-200 p-1 rounded text-sm">spawn</code> tasks, and automatic data parallelism hints.</p>
          </div>
        </div>
      </div>
    </section>

    <!-- Taste of the Syntax Section -->
    <section class="py-16 md:py-24 bg-gray-800 text-white">
      <div class="container mx-auto px-6">
        <h2 class="text-3xl md:text-4xl font-bold text-center mb-16">A Taste of ASBEL Syntax</h2>
        <div class="max-w-3xl mx-auto bg-gray-900 rounded-xl shadow-2xl p-6 md:p-8">
          <pre class="overflow-x-auto language-asbel rounded-md"><code class="language-asbel">// Immutable vs mutable
let message = "Hello, ASBEL!"      // immutable String
// a message = message.to_upper()   // compile-time error: immutable!

var counter = 0                     // mutable i32
counter += 1

// Refined types & contracts
fn sqrt(x: f64) -> f64
    requires x >= 0.0
    ensures (result * result - x).abs() < 1e-9
    x.pow(0.5)

// Pipelines & named flow
let greeting = "  asbel  "
    |> String.trim()
    |> String.capitalize_first()
    => nice

println "Greeting: \(nice)"         // Greeting: Asbel

// Concurrency
spawn || handle_client(sock)
          </code></pre>
          <p class="mt-6 text-center text-gray-400">Clean, expressive, and powerful. For more examples, check out the documentation.</p>
        </div>
      </div>
    </section>

    <!-- Call to Action Footer -->
    <section class="py-16 md:py-24 bg-gray-100">
      <div class="container mx-auto px-6 text-center">
        <h2 class="text-3xl md:text-4xl font-bold mb-6 text-gray-800">Ready to Dive In?</h2>
        <p class="text-lg md:text-xl text-gray-600 max-w-xl mx-auto mb-10 leading-relaxed">
          Explore the documentation, set up your first ASBEL project, or see where the language is headed.
        </p>
        <div class="space-y-4 md:space-y-0 md:space-x-4">
          <router-link to="/documentation"
            class="bg-yellow-500 hover:bg-yellow-400 text-gray-900 font-semibold py-3 px-8 rounded-lg text-lg transition-colors duration-300 shadow-lg hover:shadow-xl transform hover:scale-105 inline-block">
            Full Documentation
          </router-link>
          <router-link to="/roadmap"
            class="bg-gray-600 hover:bg-gray-500 text-white font-semibold py-3 px-8 rounded-lg text-lg transition-colors duration-300 shadow-lg hover:shadow-xl transform hover:scale-105 inline-block">
            View Roadmap
          </router-link>
           <a href="https://github.com/your-org/asbel" target="_blank" rel="noopener noreferrer"
            class="bg-gray-800 hover:bg-gray-700 text-white font-semibold py-3 px-8 rounded-lg text-lg transition-colors duration-300 shadow-lg hover:shadow-xl transform hover:scale-105 inline-block">
            GitHub (Soon)
          </a>
        </div>
      </div>
    </section>

  </div>
</template>

<script setup>
// For more complex animations or interactions, we might import functions or components here.
// For syntax highlighting, we'd typically use a library like Prism.js or Highlight.js.
// This would involve:
// 1. npm install prismjs
// 2. import 'prismjs/themes/prism-okaidia.css'; // or your preferred theme
// 3. import Prism from 'prismjs';
//    import 'prismjs/components/prism-clike'; // Base for many languages
//    import 'prismjs/components/prism-javascript'; // Example, ASBEL would need a custom grammar or similar
// 4. onMounted(() => { Prism.highlightAll(); });
// For now, the <pre><code> block will just show raw text.
// A custom Prism grammar for ASBEL would be ideal for proper highlighting.
</script>

<style scoped>
/* Scoped styles for HomePage if needed */
.language-asbel {
  /* Basic styling for code block if Prism.js isn't fully set up yet */
  display: block;
  white-space: pre;
  background-color: #2d2d2d; /* okaidia-like background */
  color: #f8f8f2; /* okaidia-like text */
  padding: 1.5em;
  border-radius: 0.5rem;
  font-family: 'Courier New', Courier, monospace;
  font-size: 0.9em;
  line-height: 1.5;
}
</style>
