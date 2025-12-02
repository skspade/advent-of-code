# Advent of Code

My solutions for Advent of Code in Rust.

## Project Structure

```
advent-of-code/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── days/            # Day solutions (day01.rs - day24.rs)
│   └── utils/           # Shared utilities
│       └── parsing.rs   # Input parsing helpers
├── day01/input.txt      # Input files per day
├── day02/input.txt
└── ...
```

## Usage

Run a specific day:

```bash
cargo run -- <day>
```

Example:

```bash
cargo run -- 1
```

## Adding Input

Place your puzzle input in the corresponding day folder:

```
day01/input.txt
day02/input.txt
...
```

## Utility Functions

The `utils` module provides helpers for parsing input:

- `read_lines(path)` - Read a file into a vector of strings
- `read_lines_as::<T>(path)` - Read and parse lines into any type
- `read_input(day)` - Convenience wrapper to read `dayXX/input.txt`
