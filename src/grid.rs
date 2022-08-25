use std::ops::{Add, Neg, Sub};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Point {
    x: usize,
    y: usize,
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
            Up => Point::from((self.x - 1, self.y)),
            Down => Point::from((self.x + 1, self.y)),
            Left => Point::from((self.x, self.y - 1)),
            Right => Point::from((self.x, self.y + 1)),
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

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Point { x, y }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
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
        }
    }
}

#[cfg(test)]
mod test {
    use std::ops::Add;
    use crate::grid::Direction::*;
    use crate::grid::Point;

    #[test]
    fn test() {
        let x = [Down; 255].into_iter().fold(Point::from((0, 10)), Point::add);
        let y = [Right; 255].into_iter().fold(Point::from((300, 20)), Point::add);
        assert_eq!(x, Point::from((255, 10)));
        assert_eq!(y, Point::from((300, 275)));
        assert_eq!(x + y, Point::from((555, 285)));
        assert_eq!(y - x, Point::from((45, 265)));
        assert_eq!(x + Up, Point::from((254, 10)));
        assert_eq!(x - Up, Point::from((256, 10)));
        assert_eq!(x + Left, Point::from((255, 9)));
        assert_eq!(x - Left, Point::from((255, 11)));
    }
}
