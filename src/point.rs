//! Traits for more easily dealing with the various types to represent 2d points/sizes
use glam::{IVec2, UVec2, Vec2};

use crate::{
    directions::{DIR_4, DIR_8},
    pivot::PivotedPoint,
    Pivot,
};

/// A trait for types representing an arbitrary 2d point.
pub trait Point2d {
    fn x_f32(&self) -> f32;
    fn y_f32(&self) -> f32;

    fn as_ivec2(&self) -> IVec2 {
        self.as_vec2().as_ivec2()
    }
    fn as_uvec2(&self) -> UVec2 {
        self.as_vec2().as_uvec2()
    }
    fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x_f32(), self.y_f32())
    }

    fn as_array(&self) -> [i32; 2] {
        self.as_ivec2().to_array()
    }

    fn as_iarray(&self) -> [i32; 2] {
        self.as_ivec2().to_array()
    }

    fn as_f32_array(&self) -> [f32; 2] {
        self.as_vec2().to_array()
    }
    fn as_usize_array(&self) -> [usize; 2] {
        let [x, y] = self.as_iarray();
        [x as usize, y as usize]
    }

    fn as_tuple(&self) -> (f32, f32) {
        let [x, y] = self.as_vec2().to_array();
        (x, y)
    }

    fn as_i32_tuple(&self) -> (i32, i32) {
        let [x, y] = self.as_ivec2().to_array();
        (x, y)
    }

    fn as_usize_tuple(&self) -> (usize, usize) {
        let (x, y) = self.as_i32_tuple();
        (x as usize, y as usize)
    }
}

macro_rules! impl_point2d {
    ($type:ty) => {
        impl Point2d for $type {
            fn x_f32(&self) -> f32 {
                self[0] as f32
            }

            fn y_f32(&self) -> f32 {
                self[1] as f32
            }
        }
    };
    (tuple: $type:ty) => {
        impl Point2d for ($type, $type) {
            fn x_f32(&self) -> f32 {
                self.0 as f32
            }

            fn y_f32(&self) -> f32 {
                self.1 as f32
            }
        }
    };
}

impl_point2d!(Vec2);
impl_point2d!(IVec2);
impl_point2d!(UVec2);
impl_point2d!([u32; 2]);
impl_point2d!([i32; 2]);
impl_point2d!([f32; 2]);
impl_point2d!([usize; 2]);
impl_point2d!( tuple: u32 );
impl_point2d!( tuple: i32 );
impl_point2d!( tuple: f32 );
impl_point2d!( tuple: usize);

/// A trait for types representing an integer point on a 2d grid.
#[allow(clippy::len_without_is_empty)]
pub trait GridPoint: Clone + Copy + Point2d {
    fn x(&self) -> i32;
    fn y(&self) -> i32;

    fn len(&self) -> usize {
        (self.x() * self.y()) as usize
    }

    /// Get the grid point's corresponding 1d index.
    #[inline]
    fn as_index(&self, grid_width: usize) -> usize {
        self.y() as usize * grid_width + self.x() as usize
    }

    /// Return a [PivotedPoint].
    fn pivot(&self, pivot: Pivot) -> PivotedPoint {
        PivotedPoint {
            point: self.as_ivec2(),
            pivot,
        }
    }

    /// Returns the grid point the given number of spaces above this one.
    fn up(&self, amount: i32) -> IVec2 {
        IVec2::new(self.x(), self.y() + amount)
    }

    /// Returns the grid point the given number of spaces below this one.
    fn down(&self, amount: i32) -> IVec2 {
        IVec2::new(self.x(), self.y() - amount)
    }

    /// Returns the grid point the given number of spaces to the right of
    /// this one.
    fn right(&self, amount: i32) -> IVec2 {
        IVec2::new(self.x() + amount, self.y())
    }

    /// Returns the grid point the given number of spaces to the left of
    /// this one.
    fn left(&self, amount: i32) -> IVec2 {
        IVec2::new(self.x() - amount, self.y())
    }

    /// Returns the grid point offset by the given amount.
    fn offset(&self, xy: impl GridPoint) -> IVec2 {
        self.as_ivec2() + xy.as_ivec2()
    }

    /// Retrieve the pivot-aligned point on the grid.
    ///
    /// If no pivot has been applied this will simply return the point
    /// directly.
    //fn get_aligned_point(&self, size: impl Size2d) -> IVec2;

    /// Retrieve the [`PivotedPoint`] with applied pivots, if any.
    fn get_pivot(self) -> Option<Pivot>;

    /// The [Manhattan distance](https://en.wikipedia.org/wiki/Taxicab_geometry)
    /// between two grid points.
    #[inline]
    fn manhattan(self, other: impl GridPoint) -> usize {
        let d = (self.as_ivec2() - other.as_ivec2()).abs();
        (d.x + d.y) as usize
    }

    /// The [Chebyshev distance](https://en.wikipedia.org/wiki/Chebyshev_distance)
    /// between two grid points.
    #[inline]
    fn chebyshev(self, other: impl GridPoint) -> usize {
        let d = (self.as_ivec2() - other.as_ivec2()).abs();
        d.max_element() as usize
    }

    /// The [Euclidean distance squared](https://en.wikipedia.org/wiki/Euclidean_distance)
    /// between two grid points.
    #[inline]
    fn euclidean_squared(self, other: impl GridPoint) -> usize {
        let d = self.as_ivec2() - other.as_ivec2();
        d.length_squared() as usize
    }

    /// The [Euclidean distance](https://en.wikipedia.org/wiki/Euclidean_distance)
    /// between two grid points.
    #[inline]
    fn euclidean(self, other: impl GridPoint) -> f32 {
        let d = (self.as_ivec2() - other.as_ivec2()).as_vec2();
        d.length_squared().sqrt()
    }

    /// Linearly interpolate between points a and b by the amount t.
    #[inline]
    fn lerp(self, other: impl GridPoint, t: f32) -> IVec2 {
        self.as_vec2().lerp(other.as_vec2(), t).as_ivec2()
    }

    /// Returns an iterator over the 8 points adjacent to this one.
    #[inline]
    fn adj_8(&self) -> AdjIterator {
        AdjIterator {
            i: 0,
            p: self.as_ivec2(),
            arr: DIR_8,
        }
    }

    /// Returns an iterator over the 4 points adjacent to this one.
    #[inline]
    fn adj_4(&self) -> AdjIterator {
        AdjIterator {
            i: 0,
            p: self.as_ivec2(),
            arr: DIR_4,
        }
    }
}

pub struct AdjIterator<'a> {
    i: usize,
    p: IVec2,
    arr: &'a [IVec2],
}

impl<'a> Iterator for AdjIterator<'a> {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.arr.len() {
            return None;
        };

        let p = self.p + self.arr[self.i];
        self.i += 1;

        Some(p)
    }
}

macro_rules! impl_grid_point {
    ($type:ty) => {
        impl GridPoint for $type {
            fn x(&self) -> i32 {
                self[0] as i32
            }

            fn y(&self) -> i32 {
                self[1] as i32
            }

            fn get_pivot(self) -> Option<Pivot> {
                None
            }
        }
    };
    (tuple: $type:ty) => {
        impl GridPoint for ($type, $type) {
            fn x(&self) -> i32 {
                self.0 as i32
            }

            fn y(&self) -> i32 {
                self.1 as i32
            }

            fn get_pivot(self) -> Option<Pivot> {
                None
            }
        }
    };
}

impl_grid_point!(IVec2);
impl_grid_point!(UVec2);
impl_grid_point!([u32; 2]);
impl_grid_point!([i32; 2]);
impl_grid_point!([usize; 2]);
impl_grid_point!( tuple: u32);
impl_grid_point!( tuple: i32);
impl_grid_point!( tuple: usize);

/// A trait for types representing a 2d size.
#[allow(clippy::len_without_is_empty)]
pub trait Size2d: Clone + Copy + Point2d {
    fn width(&self) -> usize;
    fn height(&self) -> usize;

    #[inline]
    fn len(&self) -> usize {
        self.width() * self.height()
    }
}

macro_rules! impl_size2d {
    ($type:ty) => {
        impl Size2d for $type {
            fn width(&self) -> usize {
                self[0] as usize
            }

            fn height(&self) -> usize {
                self[1] as usize
            }
        }
    };
    (tuple: $type:ty) => {
        impl Size2d for ($type, $type) {
            fn width(&self) -> usize {
                self.0 as usize
            }

            fn height(&self) -> usize {
                self.1 as usize
            }
        }
    };
}

impl_size2d!(IVec2);
impl_size2d!(UVec2);
impl_size2d!([u32; 2]);
impl_size2d!([i32; 2]);
impl_size2d!([usize; 2]);
impl_size2d!( tuple: u32);
impl_size2d!( tuple: i32);
impl_size2d!( tuple: usize);

#[cfg(test)]
mod tests {
    use glam::IVec2;

    use crate::GridPoint;

    #[test]
    fn taxi() {
        let a = [10, 10];
        let b = [20, 20];

        let dist = GridPoint::manhattan(a, b);
        assert_eq!(dist, 20);
    }

    #[test]
    fn adj() {
        let points: Vec<IVec2> = [10, 10].adj_4().collect();
        assert!(points.contains(&IVec2::new(10, 9)));
        assert!(points.contains(&IVec2::new(9, 10)));
        assert!(points.contains(&IVec2::new(11, 10)));
        assert!(points.contains(&IVec2::new(10, 11)));

        let points: Vec<IVec2> = [10, 10].adj_8().collect();
        assert!(points.contains(&IVec2::new(10, 9)));
        assert!(points.contains(&IVec2::new(9, 10)));
        assert!(points.contains(&IVec2::new(11, 10)));
        assert!(points.contains(&IVec2::new(10, 11)));
        assert!(points.contains(&IVec2::new(11, 11)));
        assert!(points.contains(&IVec2::new(9, 9)));
        assert!(points.contains(&IVec2::new(11, 9)));
        assert!(points.contains(&IVec2::new(9, 11)));
    }
}
