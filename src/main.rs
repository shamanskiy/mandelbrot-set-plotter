use std::{env, fs::File, str::FromStr};

use image::{codecs::png::PngEncoder, ExtendedColorType, ImageEncoder};
use num::Complex;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20",
            args[0]
        );
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("error parsing lower right corner point");
    let mut pixels = vec![0; bounds.0 * bounds.1];

    render_set(&mut pixels, bounds, upper_left, lower_right);

    save_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}

fn render_set(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert_eq!(pixels.len(), bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for col in 0..bounds.0 {
            let point = pixel_to_point(bounds, (col, row), upper_left, lower_right);
            pixels[row * bounds.0 + col] = match escape_time(point, 255) {
                None => 0,
                Some(time) => 255 - time as u8,
            };
        }
    }
}

fn save_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    let output_file = File::create(filename)?;
    let encoder = PngEncoder::new(output_file);
    encoder
        .write_image(
            pixels,
            bounds.0 as u32,
            bounds.1 as u32,
            ExtendedColorType::L8,
        )
        .unwrap();

    Ok(())
}

// this function checks whether a given complex number `c`
// belongs to the Mandelbrot set using at most `limit` iterations.
//
// if `c` belongs to the Mandelbrot set, it returns `None`.
// otherwise, the function returns `Some(i)` where `i` is the number of iterations
// it took `c` to escape the circle of radius 2.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

#[test]
fn test_escape_time() {
    assert_eq!(escape_time(Complex { re: 0.0, im: 0.0 }, 10), None);
    assert_eq!(escape_time(Complex { re: 0.25, im: 0.0 }, 10), None);
    assert_eq!(escape_time(Complex { re: 0.5, im: 0.0 }, 10), Some(5));
    assert_eq!(escape_time(Complex { re: 1.0, im: 0.0 }, 10), Some(3));
    assert_eq!(escape_time(Complex { re: 0.0, im: 0.25 }, 10), None);
    assert_eq!(escape_time(Complex { re: 0.0, im: 0.5 }, 10), None);
    assert_eq!(escape_time(Complex { re: 0.0, im: 1.0 }, 10), None);
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex { re, im }),
        _ => None,
    }
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("  ", 'x'), None);
    assert_eq!(parse_pair::<i32>("100x", 'x'), None);
    assert_eq!(parse_pair::<i32>("x200", 'x'), None);
    assert_eq!(parse_pair::<i32>("100x200bv", 'x'), None);
    assert_eq!(parse_pair::<i32>("100x200", 'x'), Some((100, 200)));
}

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            (100, 200),
            (25, 175),
            Complex { re: -1.0, im: 1.0 },
            Complex { re: 1.0, im: -1.0 }
        ),
        Complex {
            re: -0.5,
            im: -0.75
        }
    )
}
