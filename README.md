# Mandelbrot set plotter

This is a naive implementation of the [escape-time algorithm](https://en.wikipedia.org/wiki/Plotting_algorithms_for_the_Mandelbrot_set) to plot the Mandelbrot set.

Usage:

```
cargo run OUTPUT_FILENAME RESOLUTION UPPER_LEFT LOWER_RIGHT
```

e.g.

```
cargo run mandel.png 1000x750 -1.20,0.35 -1,0.20
```

![alt text](mandel.png)

The code is taken from the "Programming Rust, 2nd Edition" [book](https://www.oreilly.com/library/view/programming-rust-2nd/9781492052586/) by Jim Blandy, Jason Orendorff, Leonora F.S. Tindall.
