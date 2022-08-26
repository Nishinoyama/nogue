use core::num::Wrapping;
use core::ops::{Add, Neg, Sub};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Point {
    x: Wrapping<u8>,
    y: Wrapping<u8>,
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Self::Output {
        use Direction::*;
        match rhs {
            Down => Point {
                x: self.x + Wrapping(1),
                y: self.y,
            },
            Right => Point {
                x: self.x,
                y: self.y + Wrapping(1),
            },
            Up => Point {
                x: self.x - Wrapping(1),
                y: self.y,
            },
            Left => Point {
                x: self.x,
                y: self.y - Wrapping(1),
            },
            UpLeft => self + Up + Left,
            UpRight => self + Up + Right,
            DownLeft => self + Down + Left,
            DownRight => self + Down + Right,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<Direction> for Point {
    type Output = Point;

    fn sub(self, rhs: Direction) -> Self::Output {
        self + (-rhs)
    }
}

impl From<(u8, u8)> for Point {
    fn from((x, y): (u8, u8)) -> Self {
        let (x, y) = (Wrapping(x), Wrapping(y));
        Point { x, y }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Neg for Direction {
    type Output = Direction;
    fn neg(self) -> Self::Output {
        use Direction::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
            UpLeft => DownRight,
            UpRight => DownLeft,
            DownLeft => UpRight,
            DownRight => UpLeft,
        }
    }
}

impl Add for Direction {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point::from((0, 0)) + self + rhs
    }
}

#[cfg(test)]
mod test {
    use crate::grid::Direction::*;
    use crate::grid::Point;
    use core::ops::Add;

    #[test]
    fn test() {
        let x = [Down; 20]
            .into_iter()
            .fold(Point::from((0, 10)), Point::add);
        let y = [Right; 20]
            .into_iter()
            .fold(Point::from((100, 20)), Point::add);
        assert_eq!(x, Point::from((20, 10)));
        assert_eq!(y, Point::from((100, 40)));
        assert_eq!(x + y, Point::from((120, 50)));
        assert_eq!(y - x, Point::from((80, 30)));
        assert_eq!(x + Up, Point::from((19, 10)));
        assert_eq!(x - Up, Point::from((21, 10)));
        assert_eq!(x + Left, Point::from((20, 9)));
        assert_eq!(x - Left, Point::from((20, 11)));
        assert_eq!(x + UpLeft, Point::from((19, 9)));
        assert_eq!(x + UpRight, Point::from((19, 11)));
        assert_eq!(x + DownLeft, Point::from((21, 9)));
        assert_eq!(x + DownRight, Point::from((21, 11)));
    }
}
