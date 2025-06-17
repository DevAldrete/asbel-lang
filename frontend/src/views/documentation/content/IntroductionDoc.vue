<template>
  <article class="prose prose-lg max-w-none lg:prose-xl xl:prose-2xl text-gray-700">
    <h2 class="text-3xl font-bold mb-6 text-gray-800 border-b pb-2">Introduction to ASBEL</h2>

    <p>ASBEL (A Simple But Efficient Language) is a programming language designed with the ambitious goal of bridging the gap between the readability and productivity of high-level languages like Python and the safety and performance of low-level systems languages like C and Rust. Its core philosophy is <strong>"transparent abstraction,"</strong> where the behavior of the source code maps directly and predictably to the compiled binary, minimizing surprises and making performance analysis intuitive.</p>

    <p>To achieve this, ASBEL is implemented in Rust and <strong>transpiles to highly optimized C code</strong>. This strategic choice leverages mature and ubiquitous C compilers like GCC and Clang, ensuring maximum performance and broad platform compatibility from day one.</p>

    <p>This documentation provides a deep dive into the ASBEL language specification, detailing its syntax, semantics, and the key features that set it apart. It is intended for developers, language designers, and anyone interested in understanding the foundational principles of ASBEL and its place in the modern programming landscape.</p>

    <p>ASBEL's mission is to empower developers to write robust, efficient systems software without the cognitive overhead of manual memory management or the syntactic complexity of explicit lifetime annotations. Through a carefully designed system of inferred ownership and borrowing, ASBEL aims to eliminate entire classes of common programming errors while maintaining a clean, expressive, and productive development experience.</p>
  </article>
</template>
<script setup></script>
<style scoped>
.prose :where(code):not(:where([class~="not-prose"] *))::before,
.prose :where(code):not(:where([class~="not-prose"] *))::after {
  content: ''; /* Remove backticks from inline code */
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
