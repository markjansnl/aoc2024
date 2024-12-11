# Advent of Code 2024

In this repository you can find the source code of my solutions to the [Advent of Code 2024](https://adventofcode.com/2024) puzzles in the [Rust](https://www.rust-lang.org/) programming language.

## Setup

**Step 1**: After cloning the repository, run `./init.sh [ session ]` to create empty `input.txt` files for every day, which is needed to compile the binary that can download your own input files and run the solutions. When a session key is provided as parameter, it will be saved in `.session`.

**Step 2**: To download your input files, run `cargo run -- --download [ --all | --day XX,XX,... ]`. When no parameters are provided, the input file for 'today' is downloaded.

**Step 3**: Run the solutions using `cargo run [ -- <PARAMETERS> ]`. The following parameters are supported:
* `--download`:         Download input files. See `--all`, `--day` and `--part` to specify which input files to download.
                        Default the input file of 'today' is downloaded.
* `--day XX,XX,...`:    Download or run only specified days, comma separated. When no `--day` or `--all` is provided, only day 'today' is downloaded or run.
* `--part XX,XX`:       Run only the specified parts, comma separated. When no `--part` or `--all` is provided, only part 1 is run.
* `--all`:              Same as specifying all days and all parts using `--day` and `--part`.
* `--help`:             Show help and available parameters.

**Step 4**: [Benchmark](#benchmarks) the code on your own system using `cargo bench`.

## Framework

This year I have built a new framework to run the solutions, with a lot less macros than previous year. The advantages of this new framework are:
* Specify the types for `Parsed` and `Output` at the top instead of a separate module. Also the bench sample size can be configured in the file of the day.
* Template for each day is the same, no need to have struct `DayXX` anymore. All days have a struct `Day` and a struct `Parser`.
* Tests are in the file of the day and can be run from VS Code/Rust Analyzer. For every example you can just click on `Run Test`. There are also
  tests for running the parts, which are ommitted by default.

## Tests

Using `cargo test` you can test all the examples. In the file days you can run single tests per example by just clicking on `Run Test` in VS Code/Rust Analyzer.

## Benchmarks

All days can be benchmarked using Criterion. Run `cargo bench` to run all benchmarks on your own system.
To run the benchmarks for a single day run `cargo bench "Day XX"`.

The sample size can be configured in the file of the day within the `day!` macro.

Some nice charts are generated. You can find them after benchmarking in `target/criterion/report/index.html`.

