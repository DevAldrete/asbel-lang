<template>
  <article class="prose prose-lg max-w-none lg:prose-xl xl:prose-2xl text-gray-700">
    <h2 class="text-3xl font-bold mb-6 text-gray-800 border-b pb-2">10. Concurrency and Parallelism</h2>
    <p>ASBEL is designed for modern multi-core architectures with safe and easy-to-use concurrency primitives.</p>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">Lightweight Tasks (<code>spawn</code>)</h3>
    <p>The <code>spawn</code> keyword launches a new concurrent task (similar to a goroutine or lightweight thread). ASBEL's ownership system guarantees thread safety by preventing data races at compile time.</p>
    <pre><code class="language-asbel">
fn handle_client(client_socket: Socket) { /* ... */ }

// Assuming 'accept_connection' returns a new client socket
let client = accept_connection(listener_socket)!
spawn || handle_client(client) // Spawns a new task to handle the client
    </code></pre>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">Automatic Parallelization Hints (<code>@parallel</code>)</h3>
    <p>For data-parallel problems, developers can provide hints to the compiler with the <code>@parallel</code> attribute. This allows ASBEL to automatically distribute work across available CPU cores when it can prove the operation is safe and beneficial.</p>
    <pre><code class="language-asbel">
let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] // Example data

fn expensive_compute(n: i32) -> i32 {
    // Simulate some intensive computation
    // In a real scenario, this would be a CPU-bound task
    var result = n
    for _ in 0..1_000_000: // Placeholder for work
        result = (result + n) % 1000
    result
}

let results = data
    |> filter(|x| x % 2 == 0)     @parallel  // This filter can run on multiple threads.
    |> map(expensive_compute)     @parallel  // This map operation is also parallelized.
    |> reduce(|a, b| a + b)       @sequential // This final reduction is sequential.

println "Parallel processing results: \(results)"
    </code></pre>
    <p>The <code>@parallel</code> hint suggests to the compiler that the marked operation (like <code>filter</code> or <code>map</code> on a collection) is a good candidate for parallel execution. The compiler will analyze data dependencies and other factors to determine if and how to parallelize the operation safely and effectively.</p>
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
