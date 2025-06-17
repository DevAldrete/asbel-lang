<template>
  <article class="prose prose-lg max-w-none lg:prose-xl xl:prose-2xl text-gray-700">
    <h2 class="text-3xl font-bold mb-6 text-gray-800 border-b pb-2">7. Structs, Interfaces & Macros</h2>
    <p>ASBEL provides robust mechanisms for abstraction and code reuse through user-defined types (<code>type</code>), interfaces for shared behavior, and a safe macro system.</p>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">7.1. Structs (<code>type</code>)</h3>
    <p>Structs, declared with <code>type</code>, are custom composite data types. By default, they are value types.</p>
    <pre><code class="language-asbel">
type Vec2
    x: f32
    y: f32

type Person
    name: String
    age: u8(0..=150) // Using a refined type!
    is_active: Bool

let point = Vec2(x: 10.5, y: -3.2)
let user = Person(name: "Aldrete", age: 30, is_active: true)
    </code></pre>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">7.2. Implementations (<code>impl</code>)</h3>
    <p>The <code>impl</code> block defines methods associated with a type (<code>&self</code>, <code>&mut self</code>, <code>self</code>, or no <code>self</code> for associated functions).</p>
    <pre><code class="language-asbel">
type Vec2
    x: f32
    y: f32

impl Vec2
    fn new(x: f32, y: f32) -> Vec2 // Constructor
        Vec2(x: x, y: y)

    fn length(&self) -> f32 // Immutable reference
        (self.x^2 + self.y^2).sqrt()

    fn normalize(&mut self) // Mutable reference
        let len = self.length()
        if len > 0.0:
            self.x /= len
            self.y /= len

let mut vec = Vec2.new(x: 3.0, y: 4.0)
vec.normalize()
    </code></pre>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">7.3. Interfaces (<code>interface</code>)</h3>
    <p>Interfaces define a set of behaviors (method signatures) that types can implement, enabling polymorphism.</p>
    <pre><code class="language-asbel">
interface Serializable
    fn to_json(&self) -> String

impl Serializable for Person
    fn to_json(&self) -> String
        "{\"name\": \"\(self.name)\", \"age\": \(self.age)}"

fn print_json(item: impl Serializable)
    println(item.to_json())

print_json(user)
    </code></pre>

    <h3 class="text-2xl font-semibold mt-8 mb-4 text-gray-800">7.4. Metaprogramming: Hygienic Macros and Derivations</h3>
    <p>ASBEL includes hygienic macros (written in ASBEL) for compile-time code generation, avoiding variable capture. The <code>@derive</code> attribute uses macros to auto-generate standard methods.</p>
    <pre><code class="language-asbel">
// Conceptual macro example
macro with_accessors(TypeName, field_name, FieldType) { /* ... */ }

@derive(Serialize, Deserialize, Debug, Builder)
type User
    name: String
    age: u8(0..=150)
    email: String
// Compiler generates to_json(), from_json(), debug string, builder pattern, etc.
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
