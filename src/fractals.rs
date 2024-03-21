use slint::Rgba8Pixel;
//use color::{Deg, Hsv, Rgb, ToRgb};

use crate::{CoordinateSpace, Point};

/// Calculates a pixel's color given an x, y coordinate in some coordinate space T
pub trait Pixelator<T: CoordinateSpace> {
    fn get_pixel(&self, point: &Point<T>) -> Rgba8Pixel;
}


/// Creates a mandelbrot fractal.
#[derive(Debug, Clone, PartialEq)]
pub struct Mandelbrot {
    iterations: i32,
}

impl Mandelbrot {
    pub fn new(iterations: i32) -> Self {
        Mandelbrot { iterations }
    }
}

/// Indicates that Mandelbrot is a coordinate space.
impl CoordinateSpace for Mandelbrot {}

impl Pixelator<Mandelbrot> for Mandelbrot {
    /// Calculates each pixel of the mandelbrot fractal given the coordinate of that pixel.
    fn get_pixel(&self, point: &Point<Mandelbrot>) -> Rgba8Pixel {
        let c_r = point.x;
        let c_i = point.y;
        
        let mut z_r = 0.0f64;
        let mut z_i = 0.0f64;
        for i in 0..=self.iterations {
            let z_r2 = z_r * z_r;
            let z_i2 = z_i * z_i;
            let dist2 = z_r2 + z_i2;
            if dist2 > 4.0 {
                let color = (i*(255/self.iterations)) as u8;
                return slint::Rgba8Pixel::new(color, color, color, 255);
            }
            let new_z_r = z_r2 - z_i2 + c_r;
            let new_z_i = 2.0 * z_r * z_i + c_i;
            z_r = new_z_r;
            z_i = new_z_i;
        }
        
        slint::Rgba8Pixel::new(0, 0, 0, 0)
    }
}



/// Creates a Julia fractal.
#[derive(Debug, Clone, PartialEq)]
pub struct Julia {
    /// Number of iterations to run the function for.
    iterations: i32,

    /// Complex constant c = (c_r + c_i * i) used to calculate the julia function z = z^2 + c
    c_r: f64, c_i: f64,
}

impl Julia {
    pub fn new(iterations: i32, c_r: f64, c_i: f64) -> Self {
        Julia { iterations, c_r, c_i }
    }
}

/// Indicates that Julia is a coordinate space.
impl CoordinateSpace for Julia {}

impl Pixelator<Julia> for Julia {
    /// Calculates each pixel of the Julia fractal given the coordinate of that pixel.
    fn get_pixel(&self, point: &Point<Julia>) -> Rgba8Pixel {
        let mut z_r = point.x;
        let mut z_i = point.y;
        
        for i in 0..=self.iterations {
            let z_r2 = z_r * z_r;
            let z_i2 = z_i * z_i;
            let dist2 = z_r2 + z_i2;
            if dist2 > 4.0 {
                let color = (i*(255/self.iterations)) as u8;
                return slint::Rgba8Pixel::new(color, color, color, 255);
            }
            let new_z_r = z_r2 - z_i2 + self.c_r;
            let new_z_i = 2.0 * z_r * z_i + self.c_i;
            z_r = new_z_r;
            z_i = new_z_i;
        }
        
        slint::Rgba8Pixel::new(0, 0, 0, 0)
    }
}
