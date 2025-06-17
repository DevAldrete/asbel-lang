<template>
  <article class="prose prose-lg max-w-none lg:prose-xl xl:prose-2xl text-gray-700">
    <h2 class="text-3xl font-bold mb-6 text-gray-800 border-b pb-2">8. Foreign Function Interface (FFI) with C</h2>
    <p>ASBEL provides a robust FFI for interoperating with C libraries, leveraging its C-transpilation strategy.</p>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">8.1. Declaring External C Functions (<code>use c</code>)</h3>
    <p>Use the <code>use c</code> directive to link against C libraries and make functions available.</p>
    <pre><code class="language-asbel">
// Import specific functions
use c "stdio.h": printf, fprintf

// Import all (use with caution)
use c "math.h": *

fn main()
    let c_str = "Hello from ASBEL, calling C!\n".to_c_string()
    unsafe: // Calling C is inherently unsafe
        printf(c_str)
    </code></pre>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">8.2. C Data Types and Conversion</h3>
    <p>ASBEL provides C-compatible types (e.g., <code>c_int</code>, <code>c_char</code>, <code>c_ptr&lt;T&gt;</code>) and safe conversion mechanisms.</p>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">8.3. Exporting ASBEL Functions to C (<code>export c fn</code>)</h3>
    <p>Expose ASBEL functions to be callable from C using <code>export c fn</code>. ASBEL generates a shared object and a C header file.</p>
    <pre><code class="language-asbel">
// in asbel_library.asbel
export c fn add_in_asbel(a: c_int, b: c_int) -> c_int
    a + b
    </code></pre>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">8.4. Safety and <code>unsafe</code></h3>
    <p>Operations involving raw pointers or C functions must be in an <code>unsafe</code> block, signaling developer responsibility for upholding safety invariants.</p>
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
