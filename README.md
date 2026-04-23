# Project Euler Solutions

Solutions to [Project Euler](https://projecteuler.net) problems, written in Rust.

Each problem lives in its own crate under `problems/` and typically includes multiple approaches — from a straightforward solution to a more optimised one.

## Structure

```text
problems/
  problem-000/   ← warmup / example problem
  problem-001/   ← and so on...
```

## Running a solution

```bash
cargo run -p problem-001 -- <N>
```

Replace `problem-001` with the relevant package name and supply any required arguments.

## Building everything

```bash
cargo build --workspace
```

## Prerequisites

- [Rust](https://rustup.rs) (edition 2024, stable toolchain)
