mod fractals;
mod point;
mod singlecache;
mod viewport;

use singlecache::SingleCache;
use slint::{Image, Rgba8Pixel, SharedPixelBuffer, SharedString};

use fractals::*;
use point::*;
use viewport::*;


/// Creates the image to be used by the slint image.
///
/// # Arguments
///
/// * `func` - A function that takes an x, y coordinate in usize and returns an rgba pixel color.
fn create_image(width: u32, height: u32, pixelator: impl Pixelator<Screen>) -> Image {
    let stride = width as usize;

    let mut buffer = SharedPixelBuffer::<Rgba8Pixel>::new(width, height);
    let bytes = buffer.make_mut_slice();
    for index in 0..bytes.len() {
        let x = (index % stride) as f64;
        let y = (index / stride) as f64;
        let point = Point::<Screen>::new(x, y);
        bytes[index] = pixelator.get_pixel(&point);
    }
    Image::from_rgba8(buffer)
}


slint::include_modules!();

/// Creates a type safe viewport from a slint view port
impl<T: CoordinateSpace> Into<Viewport<T>> for SlintViewport {
    fn into(self) -> Viewport<T> {
        Viewport::new(self.x1 as f64, self.y1 as f64, self.dx as f64, self.dy as f64)
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Type used in Screen coordinate view port.
pub struct Screen { }
impl CoordinateSpace for Screen { }
impl Screen {
    pub fn new(width: f32, height: f32) -> Viewport<Screen> {
        Viewport::new(0.0, 0.0, width as f64, height as f64)
    }
}


/// Handles the clicks around the mandelbrot fractal to set the Julia constant.
fn handle_calculate_julia_constant(slint_viewport: SlintViewport, width: f32, height: f32, x: f32, y: f32) -> JuliaConstant {
    let julia_viewport: Viewport<Julia> = slint_viewport.into();
    let screen_viewport = Screen::new(width, height);
    let point = Point::<Screen>::new(x as f64, y as f64);
    let transformed_point = screen_viewport.transformer(&julia_viewport)(&point);
    JuliaConstant { real: transformed_point.x as f32, imag: transformed_point.y as f32 }
}


/// Handles the scroll wheel function by zooming in or out the viewport.
/// Returns the resulting viewport.
///
/// Centers the zoom on the mouse cursor.
fn handle_zoom_viewport(slint_viewport: SlintViewport, scroll: f32, mousex: f32, mousey: f32, width: f32, height: f32) -> SlintViewport {
    let fractal_viewport: Viewport<Mandelbrot> = slint_viewport.into();
    let screen_viewport = Screen::new(width, height);

    let diff_factor = ((scroll/10.0).min(60.0)/100.0) as f64;
    let dx_diff = fractal_viewport.dx * diff_factor;
    let dy_diff = fractal_viewport.dy * diff_factor;
    
    let point = Point::<Screen>::new(mousex as f64, mousey as f64);
    let transformed_point = screen_viewport.transformer(&fractal_viewport)(&point);

    SlintViewport { 
        x1: (fractal_viewport.x1 + dx_diff * (transformed_point.x - fractal_viewport.x1) / fractal_viewport.dx) as f32,
        y1: (fractal_viewport.y1 + dy_diff * (transformed_point.y - fractal_viewport.y1) / fractal_viewport.dy) as f32,
        dx: (fractal_viewport.dx - dx_diff) as f32,
        dy: (fractal_viewport.dy - dy_diff) as f32,
    }
}

/// Handles opening a URL in a browser.
fn handle_open_url(url: SharedString) {
    let _ = webbrowser::open(url.as_str());
}

#[cfg_attr(target_arch = "wasm32",
           wasm_bindgen::prelude::wasm_bindgen(start))]
pub fn main() {
    let main_window = MainWindow::new().unwrap();
    let logic = main_window.global::<Logic>();
    
    let mut mandelbort_cache = SingleCache::new();
    logic.on_generate_mandelbrot(move | iterations, slint_viewport, width, height | {
        let mandelbrot_viewport: Viewport<Mandelbrot> = slint_viewport.into();
        let screen_viewport = Screen::new(width, height);
        let key = (iterations, mandelbrot_viewport.clone(), width, height);

        mandelbort_cache.get_or_set(key, || {
            println!("--** on_generate_mandelbrot {width}, {height}, {iterations}, {mandelbrot_viewport:?}");
            let mandelbrot = Mandelbrot::new(iterations);
            let pixelator = screen_viewport.decorate_pixelator(&mandelbrot_viewport, &mandelbrot);
            create_image(width as u32, height as u32, pixelator)
        })
    });
    
    let mut julia_cache = SingleCache::new();
    logic.on_generate_julia(move | iterations, c, slint_viewport, width, height | {
        let julia_viewport: Viewport<Julia> = slint_viewport.into();
        let screen_viewport = Screen::new(width, height);
        let key = (iterations, julia_viewport.clone(), c.clone(), width, height);

        julia_cache.get_or_set(key, || {
            println!("--** on_generate_julia {c:?}: {width}, {height}, {iterations}, {julia_viewport:?}");
            let julia = Julia::new(iterations, c.real as f64, c.imag as f64);
            let pixelator = screen_viewport.decorate_pixelator(&julia_viewport, &julia);
            create_image(width as u32, height as u32, pixelator)
        })
    });

    logic.on_calculate_julia_constant(handle_calculate_julia_constant);
    logic.on_open_url(handle_open_url);
    logic.on_zoom_viewport(handle_zoom_viewport);

    let mw = main_window.as_weak();
    logic.on_close(move || { let _ = mw.unwrap().hide(); });

    main_window.run().unwrap();
}
