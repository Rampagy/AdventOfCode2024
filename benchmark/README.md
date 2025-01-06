# Advent of Code 2024 Benchmark

Runs all of my solutions for each problem and reports how long they took

## Directions

1. Download all of the input files and put them in this folder (`benchmark/`) with the naming convention: `dayNN.txt` (example: `benchmark/day01.txt`)
    1. Input files can be downloaded from https://adventofcode.com/2024/day/N/input where `N` is replaced with the number of the day (example: https://adventofcode.com/2024/day/1/input)
1. Download and install [64-bit rustup](https://www.rust-lang.org/tools/install)
    ```
    rustup 1.27.1 (54dd3d00f 2024-04-24)
    cargo 1.83.0 (5ffbef321 2024-10-29)
    ```
1. Run `cargo clean` in a terminal 
    1. From `benchmark/`
1. Run `cargo run --release` (single threaded benchmark) or `cargo run --release -- --multi` (multi-threaded benchmark) in a terminal
    1. From `benchmark/`

## Results

[AMD Threadripper 3970X results](3970X.md)

[AMD Ryzen 7800X3D](7800X3D.md)