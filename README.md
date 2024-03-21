# Fractal Explorer
Simple coding project to explore the Julia and Mandelbrot Fractals.

This is a first-project in [Rust](https://www.rust-lang.org/) and [slint UI](https://slint.dev/).

The code compiles to a desktop application via `cargo run`.  Or to a web assembly browser app via `wasm-pack build --release --target web` and then `python3 -m http.server`.

Most of the difficulty with writing this code was in regards to the mouse interactions, zooming, panning, resizing, etc.  Plus the challenges with learning Rust.

More information on these types of Fractals:
* [Julia Set](https://en.wikipedia.org/wiki/Julia_set)
* [Mandelbrot Set](https://en.wikipedia.org/wiki/Mandelbrot_set)

![image](https://github.com/boclair/fractal-explorer/assets/154125/9d617898-61bc-4fad-813e-229b93b54852)
