use std::marker::PhantomData;

// Trait used to ensure type safety of coordinate spaces.
pub trait CoordinateSpace {}

// A Point structure of x and y coordinate.
// Type parameter used for type safety to defined the coordinate space.
#[derive(Debug, Clone, PartialEq)]
pub struct Point<T: CoordinateSpace> {
    pub x: f64,
    pub y: f64,
    _marker: PhantomData<T>
}

impl<T: CoordinateSpace> Point<T> {
    pub fn new(x: f64, y: f64) -> Self {
        Point::<T> { x: x, y: y, _marker: PhantomData }
    }
}
