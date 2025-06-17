<template>
  <article class="prose prose-lg max-w-none lg:prose-xl xl:prose-2xl text-gray-700">
    <h2 class="text-3xl font-bold mb-6 text-gray-800 border-b pb-2">6. Pipelines</h2>
    <p>ASBEL elevates pipelines to a first-class language feature to improve readability and fluency of chained operations, allowing data to flow from left-to-right.</p>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">6.1. Inline Flow (<code>|></code>) - The Pipe Operator</h3>
    <p>The pipe operator (<code>|></code>) passes the result of the left expression as the first argument to the function/method on its right.</p>
    <pre><code class="language-asbel">
// Without pipelines
let result_nested = process(filter(normalize(input_data), 10))

// With pipelines (clearer flow)
let result_piped = input_data
    |> normalize()
    |> filter(10)
    |> process()

let text = "  ASBEL is simple.  "
let processed_text = text
    |> String.trim()          // "ASBEL is simple."
    |> String.to_lower()      // "asbel is simple."
    |> String.replace("simple", "powerful")
    |> String.capitalize_first() // "Asbel is powerful."
println "Processed: \(processed_text)"
    </code></pre>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">6.2. Named Flow (<code>=></code>) - Pipeline Assignment</h3>
    <p>The named flow operator (<code>=></code>) assigns an intermediate result within a pipeline to a named variable for clarity, reuse, or debugging.</p>
    <pre><code class="language-asbel">
let raw_data = get_sensor_readings()

raw_data
    |> normalize_values()
    |> remove_outliers(threshold: 0.05)
    => cleaned_data // Result of remove_outliers is bound to 'cleaned_data'

// 'cleaned_data' can be used in subsequent pipelines
let report = cleaned_data
    |> generate_summary_report()
    |> format_as_pdf()

let db_result = cleaned_data
    |> aggregate_metrics()
    |> save_to_database()!
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
