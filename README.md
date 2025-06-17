# ASBEL (A Simple But Efficient Language)

> **Write like Python – Run like C – Be safe like Rust**

ASBEL is a next-generation systems programming language that aims to combine the ergonomics of high-level scripting languages with the raw performance and predictability of low-level languages. Written in Rust, the ASBEL compiler transpiles to highly-optimized C, allowing it to ride decades of mature C toolchains while enforcing modern safety guarantees and powerful abstractions.

---

## ✨ Why ASBEL?

* **Transparent Abstraction** – What you write maps directly to the generated binary. No hidden allocations, no magic costs.
* **Inferred Ownership & Borrowing** – Memory safety without lifetime annotations.
* **Refined & Resource-Bound Types** – Types can encode value ranges (`u8(0..=100)`) or tie resources to scope (`File@auto_close`).
* **Design-by-Contract** – `requires` / `ensures` clauses catch logic errors at compile- or run-time.
* **First-Class Pipelines** – Chain transformations with the ergonomic `|>` and `=>` operators.
* **Context-Aware Strings** – Automatic escaping for SQL, HTML, JSON, and more.
* **Hygienic Macros & Derives** – Generate boilerplate safely at compile time.
* **Modern Concurrency** – Lightweight `spawn` tasks and automatic data-parallel hints (`@parallel`).
* **Integrated Tooling** – One canonical `asbel` CLI for build, run, test, fmt, and docs.

---

## 🖥️ A Taste of the Syntax

```asbel
// Immutable vs mutable
let message = "Hello, ASBEL!"      // immutable String
a message = message.to_upper()      // compile-time error: immutable!

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
```

---

## 🚀 Getting Started

### Prerequisites

* **Rust** ≥ 1.70 (for building the compiler)
* **GCC / Clang** (C11 compatible)

### Installation

```bash
# Clone
$ git clone https://github.com/your-org/asbel.git && cd asbel

# Build the compiler & CLI
$ cargo build --release

# Optionally add it to PATH
$ ln -s $(pwd)/target/release/asbel ~/.local/bin/asbel
```

### Hello World

```bash
$ asbel init hello_world && cd hello_world
$ asbel run
```

You should see:

```
Hello, ASBEL!
```

---

## 🛠️ CLI Cheat-Sheet

| Command           | Description                        |
|-------------------|------------------------------------|
| `asbel build`     | Compile the current package        |
| `asbel run`       | Build & run                        |
| `asbel test`      | Execute embedded tests             |
| `asbel fmt`       | Auto-format code                   |
| `asbel doc`       | Generate HTML docs                 |
| `asbel add <pkg>` | Add a dependency                   |
| `asbel repl`      | Interactive Read-Eval-Print Loop   |

---

## 📅 Roadmap

1. **MVP Compiler** – transpile core language to C, basic stdlib.
2. **Package Manager** – dependency resolution & semver.
3. **LSP** – full language-server experience.
4. **WASM Playground** – in-browser demo & shareable gists.
5. **Contract Verifier** – mix static & runtime enforcement.

See `PRD-ASBEL-LANG.md` for the full product vision.

---

## 🤝 Contributing

We welcome early feedback, bug reports, and PRs! Please read `CONTRIBUTING.md` (coming soon) and open an issue to get started.

---

## 📄 License

This project will be released under an open-source license (TBD). Until then, all rights reserved.

---

*Made with ❤️ by the ASBEL core team.*
