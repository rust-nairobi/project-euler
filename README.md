Project Euler 
--
[![Build Status](https://travis-ci.org/rust-nairobi/project-euler.svg?branch=master)](https://travis-ci.org/rust-nairobi/project-euler)

A repository with solutions for [Project Euler](https://projecteuler.net) problems implemented in
the [Rust programming language](https://www.rust-lang.org/en-US/).

How to Contribute
--
1. Fork this repo.
2. Create a binary in [src/bin](src/bin) corresponding to your the euler problem number e.g `100.rs`
3. Implement your solution.
4. Write some documentation.
5. Write test(s) for your solution.
6. Submit a pull request. 
7. Voila!

## Also
- The code you contribute is public domain.
- Don't be afraid of comments: the code is going to be written once, read hundreds of times, and maintained well past when you submit it.
- Keep your code as simple as possible.
- Please avoid compiler warnings.
- [Rust Clippy](https://github.com/rust-lang-nursery/rust-clippy) is a great tool for linting your code.
- Use the `cargo run --bin {your-solution-number}` to run your solution e.g `cargo run --bin 1`.
- Use the `cargo test --bin {your-solution-number}` to test your solution e.g. `cargo test --bin 1`.

## Not sure how to help?
- Improve an existing solution (increase readability, performance, etc.).
- Add tests.
