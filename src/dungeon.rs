use std::fmt::{Debug, Display, Formatter, write};

const FIELD_SIZE_ROW: usize = 41;
const FIELD_SIZE_COLUMN: usize = 63;

trait FieldPointDefault {
    fn default_edge() -> Self;
    fn default_inter() -> Self;
}

#[derive(Copy, Clone)]
struct Field<T> {
    points: [[T; FIELD_SIZE_COLUMN]; FIELD_SIZE_ROW],
}

impl<T: FieldPointDefault + Copy> Default for Field<T> {
    fn default() -> Field<T> {
        let mut points = [[T::default_inter(); FIELD_SIZE_COLUMN]; FIELD_SIZE_ROW];
        for i in 0..FIELD_SIZE_COLUMN {
            points[0][i] = T::default_edge();
            points[FIELD_SIZE_ROW - 1][i] = T::default_edge();
        }
        for i in 0..FIELD_SIZE_ROW {
            points[i][0] = T::default_edge();
            points[i][FIELD_SIZE_COLUMN - 1] = T::default_edge();
        }
        Self {
            points
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Floor {
    Wall,
    Ground,
    Water,
}

impl FieldPointDefault for Floor {
    fn default_edge() -> Self {
        Floor::Wall
    }
    fn default_inter() -> Self {
        Floor::Ground
    }
}

impl FieldPointDefault for u8 {
    fn default_edge() -> Self {
        128
    }
    fn default_inter() -> Self {
        128
    }
}

impl Display for Floor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Floor::Wall => '#',
            Floor::Ground => ' ',
            Floor::Water => '.',
        };
        write!(f, "{}", c)
    }
}

#[derive()]
struct Dungeon {
    individuals: Field<u8>,
    items: Field<u8>,
    floors: Field<Floor>,
    visible: Field<u8>,
}

impl Default for Dungeon {
    fn default() -> Self {
        Dungeon {
            individuals: Field::default(),
            items: Field::default(),
            floors: Field::default(),
            visible: Field::default()
        }
    }
}


impl Display for Dungeon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = self.floors.points.map(|x|
            x.map(|t|  format!("{}", t)).join("")
        ).join("\n");
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod test {
    use std::mem::size_of;
    use crate::dungeon::{Dungeon, Field};

    #[test]
    fn test() {
        println!("{}", size_of::<Field<u8>>());
        println!("{}", size_of::<Dungeon>());
        println!("{}", Dungeon::default());
    }
}
