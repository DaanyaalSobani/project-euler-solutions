# Feedback on `main_solution_1.rs` — a learning guide

This is **hints, not a solution**. Goal: help you refactor your working Project Euler #11 code into something more idiomatic Rust *and* better-designed software in general. The lessons here travel beyond Rust — sum types, errors-as-values, and lazy pipelines are everywhere now (TS, Kotlin, Swift, Go, modern Java/Python).

## Suggested order to attack the refactor

1. Collapse I/O with `fs::read_to_string` + `?` — smallest win, learns error handling.
2. Replace the four near-identical direction blocks with one helper that takes `(dr, dc)` deltas, then iterate over a list of directions. **The biggest design lesson in this file.**
3. Replace `direction: &str` with an `enum Direction`.
4. Pull the algorithm out of `main` into its own function and add a unit test on a hand-built tiny grid.

Do them in that order — each step makes the next easier.

---

## Quick scan of the current code

What's working: the algorithm is correct, the bounds checks are right (including `j >= 3` for the anti-diagonal), and you're reading the file relative to `CARGO_MANIFEST_DIR` which is the right idea for a Project Euler script.

What's worth rethinking:

- 10 lines of I/O ceremony (`File::open` + two `match`/panic blocks) where 1 line would do.
- `.split(" ").collect::<Vec<_>>().into_iter().map(...)` — collects a Vec only to immediately re-iterate it, and breaks on tabs / double spaces.
- Four near-identical blocks for right / down / diag / anti-diag — copy-paste with three things changing (deltas + one bounds check).
- `direction: &'static str` — "stringly-typed" state. The compiler can't catch `"rigth"`.
- `max`, `cord_of_max`, `direction` are three loose mutable variables that are always updated together — they want to be one struct.
- `main` does four jobs (read file, parse, search, print). Hard to test the algorithm in isolation.

---

## 1. Modeling with enums & structs

### Why `direction: &str` is the smell

Three things the compiler can't help you with:

1. **Typos compile.** `direction = "rigth"` is valid Rust. Later when you `match direction { "right" => ... }`, you silently miss the case.
2. **No exhaustiveness.** Add a 5th direction tomorrow → nothing forces you to update the printing/handling code. With an `enum`, `match` refuses to compile until you handle the new variant.
3. **Behavior can't live on the type.** You can't ask a `&str` "what's your row/column delta?" — that knowledge has to live somewhere else (the four `if` blocks). With an enum, you can write `impl Direction { fn delta(self) -> (isize, isize) }` and the deltas live *with* the thing they describe.

### The portable lesson: "make illegal states unrepresentable"

Famous Rust/FP slogan. Strings allow `""` or `"banana"` as a direction; an enum allows only valid directions. This is *the* big idea behind sum types, and it shows up far beyond Rust — same lesson in TypeScript discriminated unions, Kotlin sealed classes, Swift enums, Haskell ADTs.

The most common application: replace a `bool` flag plus an `Option<T>` that's "only meaningful when the flag is true" with a single enum.

```text
struct User { is_premium: bool, premium_expires: Option<Date> }
   // allows `is_premium = true, expires = None` — impossible state

enum Membership { Free, Premium { expires: Date } }
   // Free can't have an expiry, Premium can't lack one. Same data, fewer bugs.
```

That pairing of "boolean + sometimes-meaningful Option" is *almost always* an enum hiding in plain sight.

### Designing your `Direction` enum

Things to think about (don't write yet, just sketch on paper):

- **Variants**: how many directions actually exist? You have 4. Ask whether `UpRight` is "different" from `DownLeft` for *your* problem (it isn't — same line of cells). Naming matters.
- **Where deltas live**: deltas are a property *of* a direction. Co-locate them via `impl Direction { fn delta(self) -> (isize, isize) }`. Now the four repeated blocks in your loop collapse into one — *that's the connection between this section and section 2.*
- **Iterating all variants**: you'll want to loop over every direction. Two common patterns: a `const ALL: [Direction; 4]` array on the enum, or pull in the `strum` crate. The const array is plenty for this problem.
- **`isize` vs `usize` for deltas**: anti-diagonal needs negative deltas. `usize` underflow panics in debug. Look up `usize::checked_sub` and `checked_add_signed` — Rust forces you to confront "what if I walk off the grid?" which is exactly the bug your `if j >= 3` check is trying to prevent.

### Group your result variables into a struct

You have three vars that are always updated together: `max`, `cord_of_max`, `direction`. That's a struct waiting to happen.

```text
struct BestRun {
    product: u64,
    start: (usize, usize),
    direction: Direction,
}
```

Two payoffs:
- Functions can return one value instead of a tuple.
- You can write `.max_by_key(|r| r.product)` over an iterator of candidates and get the whole record back.

**Heuristic: if two variables are always read or written together, they want to be one variable.**

### When an enum is the *wrong* tool

Important to know the limits, otherwise you'll over-apply it:

- **Open-ended sets** (plugins, user-defined shapes, third-party extensions) → use a `trait` and `dyn Trait`. You can't enumerate something that's open.
- **Variants that share 95% of their fields** and differ in trivial ways → maybe one struct with the small variation as a field. Don't force-fit.
- **Many call sites** that all match on it → adding a variant means touching every site. Sometimes desired (forces review); sometimes a trait is better.

**Rule of thumb: enum for closed sets known at compile time, trait for open sets.** "Exactly 4 grid directions" → enum. "Any image format we might support someday" → trait.

### Newbie mistakes

- Reaching for `Option<Option<T>>` (almost always means you needed a 3-variant enum).
- Stringly-typed APIs: `request("GET", "/users")` → `request(Method::Get, "/users")`.
- "Status codes" as integers: `state == 0` for idle, `1` for running. Enum.
- Adding a new variant *and* leaving a `_ => ...` catch-all in match arms — defeats the whole point of exhaustiveness. Avoid catch-alls unless you genuinely mean "ignore everything else."

---

## 2. Error handling & `?`

### The big idea: errors are values, not control flow

In Java/Python/C++, errors hide. A function signature says "returns int," but it might throw. The caller has no way to know without reading docs (or the implementation).

In Rust, errors are part of the return type:

```text
fn read_to_string(path: &Path) -> Result<String, io::Error>
```

The signature *cannot lie* — if it can fail, you see it. The compiler forces you to handle it. Same idea as `Option` (a value might be missing) extended to errors (an operation might fail).

Tradeoff: this makes everything verbose. `?` fixes the verbosity.

### What `?` actually does

It desugars to roughly:

```text
match expr {
    Ok(v) => v,
    Err(e) => return Err(e.into()),
}
```

That's it. "If success, unwrap and keep going. If failure, return it from the enclosing function." So `?` only works in functions that themselves return `Result` (or `Option`). That's why moving I/O code out of `panic`/`match` requires changing `main`'s signature to something like `fn main() -> Result<(), Box<dyn Error>>`.

The `.into()` part is important — it calls the `From` trait, which means `?` will *convert* your error into the function's declared error type if a conversion exists. This is how one function can `?` both `io::Error` and `ParseIntError` cleanly.

### `unwrap` vs `expect` vs `?` — the decision tree

Three options, three meanings:

- **`unwrap()`**: "I'm too lazy to handle this." Panics with no message. Fine in throwaway scripts and tests; avoid in real code.
- **`expect("…")`**: "This *cannot* fail, and if it does, that's a bug." The string should explain *why* you believe it can't fail. Convention: phrase it as the precondition — `expect("CARGO_MANIFEST_DIR is always set by cargo")` reads better than `expect("failed to get manifest dir")`.
- **`?`**: "Propagate to caller, they decide." Default in real code.

The deeper distinction: **panic = bug in your program. Err = expected runtime condition.** File-not-found is not a bug — it's reality. Index-out-of-bounds on a vector you just populated *is* a bug. Use the right tool.

Your current code panics on file-not-found, which is technically wrong by this rule, but for Project Euler nobody cares.

### `Option` also has `?` — the bridge to section 4

This connects directly to your bounds-checking problem. `slice::get` returns `Option<&T>` instead of panicking on out-of-bounds:

```text
fn product_in_dir(grid: &Grid, r: usize, c: usize, dr: isize, dc: isize) -> Option<u64> {
    let mut prod = 1u64;
    for k in 0..4 {
        let rr = (r as isize + k * dr) as usize;
        let cc = (c as isize + k * dc) as usize;
        prod *= grid.get(rr)?.get(cc)?;
    }
    Some(prod)
}
```

(Sketch only — you'll want to handle the `as usize` cast more carefully so negative coords return None instead of wrapping.)

The point: all your `if i+3 < grid.len() && j+3 < grid[i].len()` checks vanish. `?` short-circuits the moment any cell is missing. **Three concepts (enum, helper function, Option/?) clicking into one shape.**

### Error type advice

Two situations, two answers:

- **Application code** (binaries, scripts, Project Euler solutions): use `Box<dyn std::error::Error>` for `main`'s return type, or pull in the `anyhow` crate (`anyhow::Result<()>` + `.context("while reading grid")`). Don't define custom error types.
- **Library code** (crates other people will use): define your own error enum, usually with the `thiserror` crate. Callers want to pattern-match on specific error variants, which `Box<dyn Error>` doesn't allow.

For your file: `Box<dyn Error>` is plenty. Don't over-engineer.

### Specific changes for your file

1. `main` signature changes to `fn main() -> Result<(), Box<dyn std::error::Error>>` and ends with `Ok(())`.
2. The 10 lines of `File::open` + `match` + `read_to_string` + `match` collapse to `let s = std::fs::read_to_string(path)?;`.
3. The parsing `.unwrap()` becomes `?`.

Now your I/O has zero `match` boilerplate, zero `panic!` calls, and the error-flow is visible in every signature.

### Common newbie mistakes

- Spamming `.unwrap()` because "the compiler is yelling and I just want it to compile." That's the compiler doing its job — reach for `?` instead.
- Custom error types in 50-line scripts. Use `Box<dyn Error>` or `anyhow`.
- `Box<dyn Error>` in a *library*. Callers can't match on it. Use `thiserror`.
- Catching an error, printing it, and continuing as if nothing happened. Usually you want to propagate and let *one place* (often `main`) handle reporting.

### The portable lesson

"Errors as values" is the same idea in Go (`val, err := f()`), Haskell (`Either`), Swift (`Result`), Kotlin (`Result`), Scala. FP languages have done this for 40 years; mainstream is catching up. Once you've used it, exception-based languages start to feel slightly dishonest — the type signature isn't telling you the whole truth.

---

## 3. Iterators & functional style

### Lazy + zero-cost

Iterators in Rust are **lazy** and **zero-cost**. Lazy means `.map(...).filter(...)` does nothing until you consume the chain with `.collect()`, `.max()`, `.sum()`, or a `for` loop. Zero-cost means the compiler fuses the whole chain into a single tight loop — usually the same machine code as a hand-written `for`. So you don't pay for the prettier syntax.

Mental shift: imperative says **how** to step and accumulate. Iterator code says **what the pipeline is**. The second is usually shorter, harder to get wrong, and easier to refactor.

### Your problem in one sentence

The whole algorithm is: *"for every cell, for every direction, compute a product if the run fits, then take the max."* That sentence is one iterator expression. The four mutable variables and four `if x > max` blocks vanish.

The pipeline:
1. Generate every (i, j) cell coordinate.
2. For each, generate every direction.
3. For each (cell, direction), try to compute the product. Skip if it walks off the grid.
4. Keep the (product, cell, direction) triple with the largest product.

### The ~10 adapters worth knowing cold

There are dozens; these cover ~95% of real code:

- **`map(f)`** — transform each item.
- **`filter(pred)`** — keep matching items.
- **`filter_map(f)`** — *combo*: return `Option`, drop `None`s, unwrap `Some`s. The secret weapon for "compute X if possible, otherwise skip."
- **`flat_map(f)`** — each input produces multiple outputs (every cell → multiple directions). Flattens one layer.
- **`enumerate()`** — pair each item with its index. Replaces `for i in 0..vec.len()`.
- **`zip(other)`** — pair items from two iterators.
- **`fold(init, f)`** — general accumulation. `sum`/`product`/`max` are special cases.

Terminal methods that actually run the chain:

- `sum`, `product`, `count`, `max`, `min`, **`max_by_key(f)`**, `min_by_key(f)`
- `find(pred)`, `any(pred)`, `all(pred)`
- `collect::<Vec<_>>()` to materialize, `for_each(f)` for side effects

### `filter_map` + `Option`-returning helper is the killer combo

This is the bridge between sections 1, 2, and 3.

You have a function `product_in_dir(...) -> Option<u64>` that returns `None` when the run walks off the grid (using `slice::get` and `?`). Plug it into `filter_map`:

> "For each (cell, direction), call `product_in_dir`. Drop the Nones. Pass the Somes through."

The whole bounds-checking concern is gone from the outer code — it lives inside the helper, returned as `Option`, and `filter_map` quietly drops invalid runs. Pair with `.max_by_key(|(prod, _, _)| *prod)` to get the winning triple. Done.

### `.iter()` vs `.into_iter()` vs `.iter_mut()` — gotcha

This trips up everyone:

- `.iter()` borrows, yields `&T`.
- `.iter_mut()` mutably borrows, yields `&mut T`.
- `.into_iter()` consumes the collection, yields `T`.

Classic surprise: `for x in vec` calls `.into_iter()` and *moves* the vec. `for x in &vec` calls `.iter()` and borrows. Same applies inside iterator chains. When the borrow checker yells about a `move`, this is usually the cause.

### Iterators are not magic — they're a trait

`Iterator` is a trait with one required method:

```text
fn next(&mut self) -> Option<Self::Item>;
```

Returns `None` when exhausted. Every adapter (`map`, `filter`, ...) wraps the previous iterator in a new struct with its own `next()` impl. The compiler inlines the whole stack into one loop. You can write your own iterators by implementing this trait.

### When functional is the *wrong* choice

Don't be religious. Functional style is a tool, not an identity:

- **Long chains get unreadable.** After ~5 adapters, name intermediate steps with `let` bindings.
- **Side effects mid-chain** are a smell. `.inspect(|x| println!("{x:?}"))` is fine for debugging; anything else means you're misusing the pipeline.
- **Funky early-exit logic.** `take_while`, `find`, `try_fold` cover most cases, but if you're contorting them, a plain `for` is clearer.
- **Performance edge cases.** Usually iterators match for-loops, but a stray `.collect::<Vec<_>>()` in the middle allocates. Don't materialize unless you need to.

**Heuristic: if you have to think hard to read it, refactor it.** Clarity over cleverness, always.

### The portable lesson

This pattern is everywhere now: JS/TS `.map().filter().reduce()`, Python comprehensions / `map` / `filter`, Java Streams, C# LINQ, Kotlin sequences. Same idea: **separate the pipeline shape from the iteration mechanics.** Once you internalize it, you'll find yourself writing fewer for-loops in *every* language.

---

## 4. Killing the repetition with a `(dr, dc)` helper

You have four near-identical blocks. The *only* things that change between them are: a row delta, a column delta, and the bounds check. **That's the signal that a helper wants to exist.** Try to describe in one sentence what all four blocks have in common — that sentence is the function signature.

Sketch (don't copy, design your own):

```text
fn product_in_dir(grid: &Grid, r: usize, c: usize, dir: Direction) -> Option<u64>
```

Body: walk 4 cells in the direction, multiply, return `Some(product)`. If any step walks off the grid (`slice::get` returns `None`), `?` short-circuits to `None`. No more `if i+3 < grid.len() && ...`.

Then the outer search becomes:

```text
for every (i, j):
    for every dir in Direction::ALL:
        if let Some(p) = product_in_dir(grid, i, j, dir) {
            // candidate
        }
```

…which is really just `flat_map` + `filter_map` + `max_by_key` if you want to go full iterator.

This single refactor **fuses three lessons**: the enum (Direction), the function helper (kill repetition), and the Option/`?` (bounds checks). One change, three concepts internalized.

### Magic numbers

`3` and `4` appear throughout and mean "run length minus one" and "run length." A `const RUN: usize = 4` makes intent explicit and makes the "what about runs of 5?" variant trivial.

---

## 5. Pulling `main` apart + tests

### Separation of concerns

Your `main` does four jobs: read file, parse grid, run algorithm, print result. Ask yourself: *if I wanted to test the algorithm on a hand-written 4×4 grid, how much would I have to rip apart?* The answer tells you how coupled it is.

A good shape here is roughly:

- `read_grid(path: &Path) -> Result<Grid, …>` — does I/O and parsing.
- `best_run(grid: &Grid) -> Option<BestRun>` — pure function, no I/O, no printing.
- `main` — glue: call them, handle errors, print.

Now `best_run` is a pure function on data. You can test it.

### Testing

```text
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_max_in_tiny_grid() {
        let grid = parse_grid("1 2 3 4\n5 6 7 8\n9 1 2 3\n4 5 6 7");
        let best = best_run(&grid).unwrap();
        assert_eq!(best.product, /* compute by hand */);
    }
}
```

Why a tiny hand-built grid? Because you can verify the answer with a calculator. A good test fixture is small enough to reason about completely.

Run with `cargo test`.

### The portable lesson

**Pure functions are testable. I/O-tangled functions are not.** Wherever possible, push I/O to the edges and keep the core logic as pure transformations on data. This is the "functional core, imperative shell" pattern — applies in every language, every paradigm.

---

## Portable takeaways (the ideas worth carrying everywhere)

1. **Sum types ("make illegal states unrepresentable")** — closed sets of variants, exhaustively matched. Replace stringly-typed and bool+Option pairs.
2. **Errors as values** — return `Result`/`Either`/`(val, err)` instead of throwing. Type signatures stop lying.
3. **Lazy pipelines** — separate "what's the data flow" from "how do I iterate." Recognize the patterns: filter+map, accumulate-max, group-by.
4. **Functional core, imperative shell** — pure logic in the middle, I/O at the edges. Testable by construction.
5. **"If two variables are always updated together, they want to be one struct."**
6. **"If you can't describe the four blocks' difference in one sentence, the helper is the sentence."**
7. **The compiler is a checklist that updates itself** — when requirements change, exhaustive `match` and required `Result` handling tell you every place that needs attention. Don't fight it with `_ =>` catch-alls or `.unwrap()`.
8. **Don't over-engineer.** Custom errors in a 50-line script, structs where a tuple fits, traits where an enum fits — all smells. Reach for the lighter tool first, upgrade only when it stops working.
