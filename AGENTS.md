# Project Guidelines

## Project

Rust 2024 edition, Bevy 0.19 (2D)

## Code Style

- Use idiomatic Rust.
- Avoid `.unwrap()`. If certain that the condition will never fail, use `.expect(...)` with an explanation for why the code should have never failed.
- Avoid `unsafe`.
- Avoid cloning data as much as possible. Use `Arc` if data needs to be shared.
- Use Rust prefixes (`as_`, `to_`, `try_`, `is_`, `has_`) according to their usual semantics. Prefer descriptive names.
- Prefer combinators for simple logic or mapping. e.g.  `bool::then_some`, `Option::is_some_and`, `Results::and_then`. If the logic hurts code readability or is too long, fallback to `let else`, `match`, etc.
- Prefer `match` over `if` statements for `enum` types. Avoid it for `bool`.
- Avoid nesting if branches with `&&`.
- Use early returns (`let Ok(...) = ... else { return; }`, `if !is_ok { return; }`).
- Prefer `let Ok(...) else { return; }` over `if let Some(...) { }` for `Option` and `Result`.
- Name variables, modules, methods, and other things elegantly and expressively.
- Use `&` and `&mut` borrows instead of taking ownership or cloning data.
- Only add `pub` to `struct`, `impl`, modules, etc. when needed.
- Use `bevy::log` for appropriate log messages.

## Code Changes

- Search and reuse first.
- Avoid allocations and prefer simple borrows for temporary data. Use `Cow` if it fits the use case.
- Use stable Rust APIs.
- Keep responsibility boundaries clear and cohesive.
- For renames, update all related callers/users.

## Refactoring

- Inspect whole module and its callers first.
- Look for duplicate code, extract logic into helper methods.

## Validation

```sh
cargo check
cargo clippy
```
