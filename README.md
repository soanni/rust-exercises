# rust-exercises

A personal collection of small, self-contained Rust programs written while working
through two books on the language. Each concept lives in its own tiny Cargo crate,
so you can open any folder, read a few lines, and run it in isolation to see one
idea in action.

If you're learning Rust, feel free to browse, copy, run, and break these examples.

## What's inside

The repository is split into two collections, one per book:

| Folder | Source | Focus |
| --- | --- | --- |
| repository root | *The Rust Programming Language* ("the Book") | Core language: ownership, structs, enums, generics, traits, lifetimes, error handling, tests |
| [`rust-in-month-of-luches/`](rust-in-month-of-luches/) | *Rust in a Month of Lunches* | Iterators, closures, collections, interior mutability, more idiomatic std usage |

Every subfolder is a standalone crate (its own `Cargo.toml` + `src/main.rs`, or
`src/lib.rs` for the library examples). They all target **Rust edition 2024** and
use only the standard library, with a single exception: `guessing_game` depends on
the `rand` crate.

The examples are numbered (`enums_1`, `enums_2`, `traits_1` … `traits_9`) to show a
concept being built up step by step. Folder names describe the idea being explored,
so you can jump straight to whatever you're trying to understand.

## Running an example

Each crate runs on its own. Pick a folder and use Cargo:

```bash
cd guessing_game
cargo run
```

Library-style examples (e.g. `restaurant`, `adder`, `tests_1`) are meant to be
tested rather than run:

```bash
cd tests_1
cargo test
```

You'll need a Rust toolchain — install it via [rustup](https://rustup.rs/) if you
haven't already. Since these use edition 2024, a recent stable `rustc` (1.85+) is
recommended.

## Topic map (root — *The Rust Programming Language*)

A rough guide to where things live at the repository root:

- **Getting started** — `hello_world`, `hello_cargo`, `guessing_game`
- **Common concepts** — `variables`, `shadow`, `expression`, `func*`, `branches*`,
  `if_let*`, `loop*`, `while_loop*`, `for_loop*`
- **Primitive types** — `bool`, `floats`, `num_operations`, `tuple`, `array`,
  `array_invalid_index`
- **Ownership, references & slices** — `scope_1`, `stack`, `clone_1`, `ref_owner*`,
  `mut_ref_1`, `func_and_owner_1`, `dangling_ref_1`, `slices_*`, `str_1`,
  `str_immutable`
- **Structs** — `struct_*`, `tuple_struct_1`, `rectangle_1` … `rectangle_10`
- **Enums & pattern matching** — `enums_*`, `match_*`
- **Collections** — `vec_1`, `strings_*`, `hash_map_*`
- **Modules & crates** — `restaurant*`, `backyard`, `use_1`
- **Error handling** — `panic_*`, `result_*`, `error_propagation_*`, `try_*`
- **Generics, traits & lifetimes** — `generics_*`, `generics_structs_*`, `traits_*`,
  `lifetimes_*`
- **Testing** — `adder`, `tests_1`

## Topic map (`rust-in-month-of-luches/`)

- **Iterators & adapters** — `iterator_*`, `fold_*`, `flatten_1`, `all_any_*`,
  `find_position_1`, `chunks_windows_1`, `cycle_zip_1`, `peek_1`, `inspect_*`,
  `match_indices_1`
- **Closures** — `closure_1` … `closure_14`
- **Collections** — `vector_*`, `vec_deque_1`, `hash_map_*`, `hash_set_1`,
  `btree_map_1`, `binary_heap_*`
- **Interior mutability & sync** — `interior_mutability_*` (incl. `_mutex_*` and
  `_rwlock_*`)
- **Options & results** — `option_*`, `result_*`, `unwrap_or`, `expect_1`,
  `question_mark_operator_*`, `let_else_*`, `while_let_1`, `if_let_1`
- **Traits, generics & the orphan rule** — `traits_*`, `generics_*`, `from_*`,
  `orphan_rule_*`
- **References & lifetimes** — `ref_*`, `reference_1`, `by_ref_1`, `lifetime_*`,
  `lifetime_anonymous_*`
- **Types & conversions** — `types_1`, `cast_as_char`, `enum_cast_int*`, `size_of`,
  `const_and_static`
- **Odds & ends** — `dbg_*`, `print_1`, `destructure`, `shadow_1`, `unit`,
  `uninitialized_var`

## Notes

- `target/` build artifacts are checked in for some crates but are gitignored going
  forward — you can safely `cargo clean` any of them.
- These are learning exercises, not production code. Expect the occasional
  deliberately-broken example (e.g. `array_invalid_index`, `invalid_statement`,
  `array_overflow`) that exists to demonstrate a compiler or runtime error.
