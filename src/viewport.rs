use std::marker::PhantomData;

use slint::Rgba8Pixel;

use crate::{fractals::Pixelator, CoordinateSpace, Point};

/// A typesafe ViewPort structure that improves upon the slint view point definition.
#[derive(Debug, Clone, PartialEq)]
pub struct Viewport<T: CoordinateSpace> {
    pub x1: f64,
    pub y1: f64,
    pub dx: f64,
    pub dy: f64,
    _marker: std::marker::PhantomData<T>
}

impl<S: CoordinateSpace> Viewport<S> {
    pub fn new(x1: f64, y1: f64, dx: f64, dy: f64) -> Self {
        Viewport { x1: x1, y1: y1, dx: dx, dy: dy, _marker: PhantomData }
    }

    /// Transforms a given pixelator in the orig_vp<S> to the new_vp<T>.
    pub fn decorate_pixelator<'a, T: CoordinateSpace>(&'a self, new_vp: &'a Viewport<T>, pixelator: &'a dyn Pixelator<T>) -> impl Pixelator<S> + 'a {
        let transformer = Box::new(self.transformer(&new_vp));
        ViewPortDecorator::<'a, S, T> { transformer, pixelator }
    }

    pub fn transformer<'a, T: CoordinateSpace>(&self, new_vp: &Viewport<T>) -> impl Fn(&Point<S>) -> Point<T> + 'a {
        let dx_factor = new_vp.dx/self.dx;
        let dy_factor = new_vp.dy/self.dy;
        let factor = dx_factor.abs().max(dy_factor.abs());

        // Needed when one of the axis flips it's sign between the two viewports.
        let x_factor = if dx_factor < 0.0 { -factor } else { factor };
        let y_factor = if dy_factor < 0.0 { -factor } else { factor };

        let center_orig_x = self.x1 + self.dx/2.0;
        let center_orig_y = self.y1 + self.dy/2.0;

        let center_new_x = new_vp.x1 + new_vp.dx/2.0;
        let center_new_y = new_vp.y1 + new_vp.dy/2.0;

        move | point: &Point<S> | {
            let new_x = (point.x - center_orig_x) * x_factor + center_new_x;
            let new_y = (point.y - center_orig_y) * y_factor + center_new_y;
            Point::<T>::new(new_x, new_y)
        }
    }
}

// Internal definition of the view port decorator.
struct ViewPortDecorator<'a, S: CoordinateSpace, T: CoordinateSpace> {
    transformer: Box<dyn Fn(&Point<S>) -> Point<T>>,
    pixelator: &'a dyn Pixelator<T>,
}

// Implementations the translation of the pixelator based on view ports.
impl<'a, S: CoordinateSpace, T: CoordinateSpace> Pixelator<S> for ViewPortDecorator<'a, S, T> {
    fn get_pixel(&self, point: &Point<S>) -> Rgba8Pixel {
        let transformed_point = (self.transformer)(point);
        self.pixelator.get_pixel(&transformed_point)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct Scope1 {}
    impl CoordinateSpace for Scope1 {}

    #[derive(Debug, Clone, PartialEq)]
    struct Scope2 {}
    impl CoordinateSpace for Scope2 {}

    struct TransformTester {
        orig_vp: Viewport<Scope1>,
        new_vp: Viewport<Scope2>
    }

    impl TransformTester {
        fn test(&self, p1: (f64, f64), p2: (f64, f64)) {
            self.test_direct(p1, p2);
            self.test_inverse(p1, p2);
        }

        fn test_direct(&self, new_pt: (f64, f64), orig_pt: (f64, f64)) {
            // test direct
            let f = self.orig_vp.transformer(&self.new_vp);
            assert_eq!(Point::new(new_pt.0, new_pt.1), f(&Point::new(orig_pt.0, orig_pt.1)),
                "Failed test_direct {new_pt:?} = f({orig_pt:?})");
        }

        fn test_inverse(&self, orig_pt: (f64, f64), new_pt: (f64, f64)) {
            // test inverse
            let f_inv = self.new_vp.transformer(&self.orig_vp);
            assert_eq!(f_inv(&Point::new(orig_pt.0, orig_pt.1)), Point::new(new_pt.0, new_pt.1),
                "Failed test_inverse f_inv({orig_pt:?} = {new_pt:?}");
        }
    }

    #[test]
    fn test_transformer_square() {
        let orig_vp = Viewport::<Scope1>::new(10.0, 20.0, 100.0, 100.0);
        let new_vp = Viewport::<Scope2>::new(5.0, 3.0, 10.0, 10.0);
        let tt = TransformTester { orig_vp, new_vp };

        tt.test((5.0, 3.0), (10.0, 20.0));
        tt.test((15.0, 13.0), (110.0, 120.0));
        tt.test((10.0, 8.0), (60.0, 70.0));
    }

    #[test]
    fn test_transformer_square_neg() {
        let orig_vp = Viewport::<Scope1>::new(10.0, 20.0, 100.0, 100.0);
        let new_vp = Viewport::<Scope2>::new(5.0, 3.0, 10.0, -10.0);
        let tt = TransformTester { orig_vp, new_vp };

        tt.test((5.0, 3.0), (10.0, 20.0));
        tt.test((15.0, -7.0), (110.0, 120.0));
        tt.test((10.0, -2.0), (60.0, 70.0));
    }

    #[test]
    fn test_transformer_rect() {
        let orig_vp = Viewport::<Scope1>::new(10.0, 20.0, 100.0, 200.0);
        let new_vp = Viewport::<Scope2>::new(5.0, 3.0, 10.0, 20.0);
        let tt = TransformTester { orig_vp, new_vp };

        tt.test((5.0, 3.0), (10.0, 20.0));
        tt.test((15.0, 23.0), (110.0, 220.0));
        tt.test((10.0, 13.0), (60.0, 120.0));
    }

    #[test]
    fn test_transformer_rect_neg() {
        let orig_vp = Viewport::<Scope1>::new(10.0, 20.0, 100.0, 200.0);
        let new_vp = Viewport::<Scope2>::new(5.0, 3.0, 10.0, -20.0);
        let tt = TransformTester { orig_vp, new_vp };

        tt.test((5.0, 3.0), (10.0, 20.0));
        tt.test((15.0, -17.0), (110.0, 220.0));
        tt.test((10.0, -7.0), (60.0, 120.0));
    }

    #[test]
    fn test_transformer_square_neg_zero_center() {
        let orig_vp = Viewport::<Scope1>::new(0.0, 0.0, 800.0, 800.0);
        let new_vp = Viewport::<Scope2>::new(-2.0, 1.25, 2.5, -2.5);
        let tt = TransformTester { orig_vp, new_vp };

        tt.test((-2.0, 1.25), (0.0, 0.0));
        tt.test((0.5, -1.25), (800.0, 800.0));
        tt.test((-0.75, 0.0), (400.0, 400.0));
        tt.test((0.0, 0.0), (640.0, 400.0));
    }

    #[test]
    fn test_transformer_square_neg_zero_center2() {
        let orig_vp = Viewport::<Scope1>::new(0.0, 0.0, 800.0, 400.0);
        let new_vp = Viewport::<Scope2>::new(-2.0, 2.0, 4.0, -4.0);
        let tt = TransformTester { orig_vp, new_vp };

        tt.test_direct((0.0, 0.0), (400.0, 200.0));
        tt.test_direct((-2.0, 2.0), (200.0, 0.0));
        tt.test_direct((-2.0, -2.0), (200.0, 400.0));
        tt.test_direct((2.0, -2.0), (600.0, 400.0));
        tt.test_direct((2.0, 2.0), (600.0, 000.0));
    }


    #[test]
    fn test_transformer_rect_diff_aspect() {
        let orig_vp = Viewport::<Scope1>::new(0.0, 0.0, 100.0, 200.0);
        let new_vp = Viewport::<Scope2>::new(0.0, 0.0, 10.0, 10.0);
        let tt = TransformTester { orig_vp, new_vp };

        tt.test_direct((0.0, -5.0), (0.0, 0.0));
        tt.test_direct((10.0, 15.0), (100.0, 200.0));
        tt.test_direct((5.0, 5.0), (50.0, 100.0));
    }

    #[test]
    fn test_transformer_rect_diff_aspect2() {
        let orig_vp = Viewport::<Scope1>::new(0.0, 0.0, 200.0, 100.0);
        let new_vp = Viewport::<Scope2>::new(0.0, 0.0, 10.0, 10.0);
        let tt = TransformTester { orig_vp, new_vp };

        tt.test_direct((-5.0, 0.0), (0.0, 0.0));
        tt.test_direct((5.0, 20.0), (100.0, 200.0));
        tt.test_direct((15.0, 10.0), (200.0, 100.0));
        tt.test_direct((0.0, 10.0), (50.0, 100.0));
        tt.test_direct((5.0, 5.0), (100.0, 50.0));
    }

}