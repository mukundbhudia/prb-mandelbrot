# Mandelbrot set generator

Generates the [mandelbrot set](https://en.wikipedia.org/wiki/Mandelbrot_set) as a PNG using concurrency methods in Rust.
This example was taken from the [Programming Rust](https://www.oreilly.com/library/view/programming-rust/9781491927274/) book.

## Development

* Within the repo directory run `cargo run FILE_NAME DIMENSIONS UPPERLEFT LOWERRIGHT` where `FILE_NAME` is the PNG filename, `DIMENSIONS` is the image dimensions in pixels and `UPPERLEFT` and `LOWERRIGHT` are the points on the complex plane that we generate the set from.

* To make a production build, within the repo directory run `cargo build --release`.

## Usage

Within the project directory run `cargo run mandel.png 1000x750 -1.20,0.35 -1,0.20` to generate the image above in this readme.

### Resources
* The excellent [Programming Rust](https://www.oreilly.com/library/view/programming-rust/9781491927274/) book.
* Rust Filesystem docs https://doc.rust-lang.org/std/fs/index.html.
