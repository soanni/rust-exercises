# rust-exercises

A personal collection of small, self-contained programs written while working
through several books on Rust (plus a bit of C). Each concept lives in its own tiny
crate, so you can open any folder, read a few lines, and run it in isolation to see
one idea in action.

The examples are now grouped by the book they came from. Within each book, folders
are named after the concept being explored and numbered (`enums_1`, `enums_2`,
`traits_1` … `traits_9`) to show an idea being built up step by step.

If you're learning Rust, feel free to browse, copy, run, and break these examples.

## Books

| Folder | Source | What's covered |
| --- | --- | --- |
| [`the-book/`](the-book/) | *The Rust Programming Language* (the official "Book") | The whole language, start to finish: basics, ownership, structs, enums, generics, traits, lifetimes, error handling, tests, smart pointers, concurrency, and async |
| [`rust-in-month-of-luches/`](rust-in-month-of-luches/) | *Rust in a Month of Lunches* | Idiomatic std usage: iterators, closures, collections, interior mutability, conversions |
| [`oreilly-programming-rust/`](oreilly-programming-rust/) | *Programming Rust* (O'Reilly) | GCD calculator (incl. an Actix web version), moves, copies, references, closures, panics, overflow |
| [`async-rust/`](async-rust/) | Async Rust practice | `async`/`await` with `tokio` and `reqwest` — a coffee-shop simulation, file and HTTP I/O, spawning processes, timed requests |
| [`csapp_2016/`](csapp_2016/) | *Computer Systems: A Programmer's Perspective* | A plain C `hello` — a stray companion from systems study, not Rust |
| [`rust-for-rustaceans/`](rust-for-rustaceans/) | *Rust for Rustaceans* | Placeholder — nothing here yet |

Most crates use only the standard library. The exceptions:

- `the-book/guessing_game` → `rand`
- `the-book/hello-async`, `hello-async-1`, `concur_async_*` → `trpl` (the Book's async helper crate)
- `async-rust/*` → `tokio`, and some also `reqwest` / `serde` / `serde_json`
- `oreilly-programming-rust/actix-gcd` → `actix-web`, `serde`

Every Rust crate targets **edition 2024**.

## New since the last pass

This is a much bigger collection than before. `the-book/` has grown well past the
basics to cover the later chapters:

- **Smart pointers** — `box_1`, `box_2`, `rc_1`, `deref_1`, `deref_2`, `drop_1`, `drop_2`
- **Concurrency** — `thread_1` … `thread_4`, `mutex_1`, `mutex_2`, `channel_1` … `channel_4`
- **Async** — `hello-async`, `hello-async-1`, `concur_async_1` … `concur_async_3`
- **A real project** — `minigrep`, `minigrep_1` (the Book's grep clone, split into lib + binary)
- **A Cargo workspace** — `add/` (a workspace with `add_one` and `adder` members)

And three entirely new book folders joined the collection: `oreilly-programming-rust/`,
`async-rust/`, and `csapp_2016/` (with `rust-for-rustaceans/` reserved for what's next).

## Running an example

Each folder is a standalone crate. Pick one and use Cargo:

```bash
cd the-book/guessing_game
cargo run
```

Library-style examples (e.g. `the-book/restaurant`, `the-book/adder`,
`the-book/tests_1`, and the `minigrep` libs) are meant to be tested:

```bash
cd the-book/tests_1
cargo test
```

The `add/` workspace builds all members together:

```bash
cd the-book/add
cargo run -p adder
```

A couple of things aren't Cargo crates:

- `the-book/hello_world` is the very first rustc example — compile it directly with
  `rustc main.rs`.
- `csapp_2016/hello.c` is C — `gcc hello.c -o hello` (or `cc`).

You'll need a Rust toolchain — install it via [rustup](https://rustup.rs/) if you
haven't already. Because these use edition 2024, a recent stable `rustc` (1.85+) is
recommended.

## Notes

- These are learning exercises, not production code. Expect the occasional
  deliberately-broken example (e.g. `the-book/array_invalid_index`,
  `the-book/invalid_statement`) that exists to demonstrate a compiler or runtime
  error, and a few experiments that overflow or panic on purpose
  (`oreilly-programming-rust/int_overflow`, `stackoverflow_1`, `panic_*`).
- `target/` build artifacts are gitignored — you can safely `cargo clean` any crate.
