<template>
  <article class="prose prose-lg max-w-none lg:prose-xl xl:prose-2xl text-gray-700">
    <h2 class="text-3xl font-bold mb-6 text-gray-800 border-b pb-2">9. Tooling and Ecosystem</h2>
    <p>ASBEL is committed to providing a comprehensive, first-class tooling experience from the outset to enhance developer productivity.</p>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">9.1. The <code>asbel</code> CLI</h3>
    <p>The <code>asbel</code> command-line interface is the central hub for development tasks:</p>
    <ul class="list-disc pl-6">
      <li><code>asbel init</code>: Creates a new ASBEL project.</li>
      <li><code>asbel build</code>: Compiles the current project.</li>
      <li><code>asbel run</code>: Builds and runs the project.</li>
      <li><code>asbel test</code>: Runs all unit and integration tests.</li>
      <li><code>asbel add [package]</code>: Adds a new dependency.</li>
      <li><code>asbel fmt</code>: Formats ASBEL code to a canonical style.</li>
      <li><code>asbel doc</code>: Generates HTML documentation.</li>
      <li><code>asbel repl</code>: Starts an interactive Read-Eval-Print Loop.</li>
    </ul>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">9.2. Language Server Protocol (LSP)</h3>
    <p>ASBEL will provide <code>asbel-lsp</code> for tight integration with modern code editors (VS Code, Neovim), enabling features like autocompletion, real-time diagnostics, go-to-definition, etc.</p>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">9.3. WASM Playground</h3>
    <p>An online playground (compiled to WebAssembly) will allow trying ASBEL in-browser without local installation, useful for education and experimentation.</p>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">9.4. Contract Verifier</h3>
    <p>A tool integrated into the compiler and test runner for checking <code>requires</code> and <code>ensures</code> contracts, supporting static verification, runtime checking, and fuzzing.</p>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">9.5. Integrated Debugging Hooks</h3>
    <p>Language-level support like <code>debug_snapshot()</code> for advanced debugging, potentially enabling time-travel debugging with integrated tooling.</p>
    <pre><code class="language-asbel">
fn complex_algorithm(data: List<i32>) -> i32
    let step1 = data |> normalize()
    debug_snapshot("After normalization", step1)
    // ...
    step1.sum() // Example, not full code
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
