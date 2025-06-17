<template>
  <article class="prose prose-lg max-w-none lg:prose-xl xl:prose-2xl text-gray-700">
    <h2 class="text-3xl font-bold mb-6 text-gray-800 border-b pb-2">4. Control Flow</h2>
    <p>ASBEL provides modern, expressive control flow constructs designed to be powerful, safe, and highly readable.</p>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">4.1. <code>if/else</code> as an Expression</h3>
    <p><code>if/else</code> is an expression, evaluating to a value. Both branches must evaluate to the same type.</p>
    <pre><code class="language-asbel">
let label = if score > 90: "A" else: "B"
println "Grade: \(label)"

let message = if user.is_logged_in():
    "Welcome back, \(user.name)!"
else:
    "Please log in."
    </code></pre>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">4.2. <code>guard</code> for Early Exit</h3>
    <p>Validates conditions at the start of a scope and exits early if not met, avoiding nested <code>if</code>s. The <code>else</code> block must exit the current scope.</p>
    <pre><code class="language-asbel">
fn process_request(request: Request)
    guard let user = request.user else:
        println "Error: No user."
        return

    guard user.is_admin else:
        println "Error: User not admin."
        return

    println "Processing for admin: \(user.name)"
    </code></pre>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">4.3. <code>match</code> with Patterns and Ranges</h3>
    <p>Powerful pattern matching with compile-time exhaustiveness checking. Supports ranges and pattern guards.</p>
    <pre><code class="language-asbel">
match http_code
    200: println "OK"
    404: println "Not Found"
    500..=599: println "Server Error" // Range pattern
    _ : println "Unknown code"

match request
    HttpRequest { method: "POST", body: Some(data) } if let Ok(json) = parse_json(data):
        handle_json_post(json)
    _ :
        handle_default(request)
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
