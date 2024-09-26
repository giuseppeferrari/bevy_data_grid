//! Utilities for dealing with directions on a 2d grid.
use bevy_math::IVec2;

use crate::point::GridPoint;

pub const UP: IVec2 = IVec2::from_array([0, 1]);
pub const DOWN: IVec2 = IVec2::from_array([0, -1]);
pub const LEFT: IVec2 = IVec2::from_array([-1, 0]);
pub const RIGHT: IVec2 = IVec2::from_array([1, 0]);
pub const UP_LEFT: IVec2 = IVec2::from_array([-1, 1]);
pub const UP_RIGHT: IVec2 = IVec2::from_array([1, 1]);
pub const DOWN_LEFT: IVec2 = IVec2::from_array([-1, -1]);
pub const DOWN_RIGHT: IVec2 = IVec2::from_array([1, -1]);

/// Array of four orthogonal grid directions.
pub const DIR_4: &[IVec2] = &[UP, DOWN, LEFT, RIGHT];

/// Array of eight adjacent grid directions.
pub const DIR_8: &[IVec2] = &[
    UP, DOWN, LEFT, RIGHT, UP_LEFT, UP_RIGHT, DOWN_LEFT, DOWN_RIGHT,
];

/// Four orthogonal directions on a 2d grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridCompassQuadrant {
    Up,
    Down,
    Left,
    Right,
}

impl From<GridCompassQuadrant> for IVec2 {
    fn from(d: GridCompassQuadrant) -> Self {
        match d {
            GridCompassQuadrant::Up => UP,
            GridCompassQuadrant::Down => DOWN,
            GridCompassQuadrant::Left => LEFT,
            GridCompassQuadrant::Right => RIGHT,
        }
    }
}

impl GridCompassQuadrant {
    /// Retrieve the direction from the given point, or none if it's (0,0).
    pub fn from_point(p: impl GridPoint) -> Option<GridCompassQuadrant> {
        match p.as_ivec2().signum().to_array() {
            [0, 1] => Some(GridCompassQuadrant::Up),
            [0, -1] => Some(GridCompassQuadrant::Down),
            [-1, 0] => Some(GridCompassQuadrant::Left),
            [1, 0] => Some(GridCompassQuadrant::Right),
            _ => None,
        }
    }

    /// Retrieve a direction from it's corresponding index.
    pub fn from_index(i: usize) -> Option<GridCompassQuadrant> {
        match i {
            0 => Some(GridCompassQuadrant::Up),
            1 => Some(GridCompassQuadrant::Down),
            2 => Some(GridCompassQuadrant::Left),
            3 => Some(GridCompassQuadrant::Right),
            _ => None,
        }
    }

    /// Convert a direction to it's corresponding index.
    pub fn to_index(&self) -> usize {
        match self {
            GridCompassQuadrant::Up => 0,
            GridCompassQuadrant::Down => 1,
            GridCompassQuadrant::Left => 2,
            GridCompassQuadrant::Right => 3,
        }
    }
}

/// 8 directions on a 2d grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridCompassOctant {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl GridCompassOctant {
    /// Retrieve the direction from the given point, or none if it's (0,0).
    pub fn from_point(p: impl GridPoint) -> Option<GridCompassOctant> {
        match p.as_ivec2().signum().to_array() {
            [0, 1] => Some(GridCompassOctant::Up),
            [0, -1] => Some(GridCompassOctant::Down),
            [-1, 0] => Some(GridCompassOctant::Left),
            [1, 0] => Some(GridCompassOctant::Right),
            [-1, 1] => Some(GridCompassOctant::UpLeft),
            [1, 1] => Some(GridCompassOctant::UpRight),
            [-1, -1] => Some(GridCompassOctant::DownLeft),
            [1, -1] => Some(GridCompassOctant::DownRight),
            _ => None,
        }
    }

    /// Retrieve a direction from an index.
    pub fn from_index(i: usize) -> Option<GridCompassOctant> {
        match i {
            0 => Some(GridCompassOctant::Up),
            1 => Some(GridCompassOctant::Down),
            2 => Some(GridCompassOctant::Left),
            3 => Some(GridCompassOctant::Right),
            4 => Some(GridCompassOctant::UpLeft),
            5 => Some(GridCompassOctant::UpRight),
            6 => Some(GridCompassOctant::DownLeft),
            7 => Some(GridCompassOctant::DownRight),
            _ => None,
        }
    }

    /// Convert a direction to it's corresponding index.
    pub fn to_index(&self) -> usize {
        match self {
            GridCompassOctant::Up => 0,
            GridCompassOctant::Down => 1,
            GridCompassOctant::Left => 2,
            GridCompassOctant::Right => 3,
            GridCompassOctant::UpLeft => 4,
            GridCompassOctant::UpRight => 5,
            GridCompassOctant::DownLeft => 6,
            GridCompassOctant::DownRight => 7,
        }
    }
}

impl From<GridCompassOctant> for IVec2 {
    fn from(d: GridCompassOctant) -> Self {
        match d {
            GridCompassOctant::Up => UP,
            GridCompassOctant::Down => DOWN,
            GridCompassOctant::Left => LEFT,
            GridCompassOctant::Right => RIGHT,
            GridCompassOctant::UpLeft => UP_LEFT,
            GridCompassOctant::UpRight => UP_RIGHT,
            GridCompassOctant::DownLeft => DOWN_LEFT,
            GridCompassOctant::DownRight => DOWN_RIGHT,
        }
    }
}
