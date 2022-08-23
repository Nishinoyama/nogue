use crate::random::Random;
use std::fmt::{Debug, Display, Formatter};

pub const FIELD_SIZE_ROW: usize = 41;
pub const FIELD_SIZE_COLUMN: usize = 63;
const AREA_SIZE_MIN: usize = 4;

#[derive(Debug, Copy, Clone)]
enum Floor {
    Wall,
    Ground,
    Water,
}

trait FieldPointDefault {
    fn default_edge() -> Self;
    fn default_inter() -> Self;
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

#[derive(Copy, Clone)]
struct Field<T> {
    points: [[T; FIELD_SIZE_COLUMN]; FIELD_SIZE_ROW],
}

#[derive(Copy, Clone)]
enum FieldFloorArea {
    Room { row: usize, column: usize },
    PathWay { relay_x: usize, relay_y: usize },
    WallFilled,
    None,
}

impl FieldFloorArea {
    fn gen<R: Random>(rnd: &mut R, max_rows: usize, max_columns: usize) -> Self {
        if rnd.gen_u8() >= 16 {
            let relay_x = (rnd.gen() as usize) % max_rows;
            let relay_y = (rnd.gen() as usize) % max_columns;
            Self::PathWay { relay_x, relay_y }
        } else {
            let row = AREA_SIZE_MIN + (rnd.gen() as usize) % (max_rows - AREA_SIZE_MIN);
            let column = AREA_SIZE_MIN + (rnd.gen() as usize) % (max_columns - AREA_SIZE_MIN);
            Self::Room { row, column }
        }
    }
}

impl Field<Floor> {
    fn gen_field_tree_dip<R: Random>(rnd: &mut R, rows: usize, columns: usize) -> Self {
        assert!(FIELD_SIZE_ROW / rows >= 10);
        assert!(FIELD_SIZE_COLUMN / columns >= 10);
        assert!(rows * columns <= 24);
        //
        // ###
        // #..
        // #..
        //
        let mut areas = [FieldFloorArea::None; 24];
        let mut room_cnt = 0;
        for i in 0..rows {
            for j in 0..columns {
                areas[i * columns + j] = FieldFloorArea::gen(
                    rnd,
                    FIELD_SIZE_ROW / rows - 1,
                    FIELD_SIZE_COLUMN / columns - 1,
                );
                if matches!(areas[i * columns + j], FieldFloorArea::Room { .. }) {
                    room_cnt += 1;
                }
            }
        }
        if room_cnt < 4 {
            return Self::gen_field_tree_dip(rnd, rows, columns);
        }

        let mut areas_seen = [false; 24];
        let mut areas_queue = [24usize; 24];
        let mut queue_i = 0;
        let mut ti = rnd.gen_max((rows * columns) as u32);
        /// todo 木チックに経路を生成する。
    }
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
        Self { points }
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
            visible: Field::default(),
        }
    }
}

impl Dungeon {
    fn new_random<R: Random>(&mut rnd: R) -> Self {
        Self {
            individuals: Field::default(),
            floors: Field::gen_field_tree_dip(rnd, 4, 6),
            items: Field::default(),
            visible: Field::default(),
        }
    }
}

impl Display for Dungeon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self
            .floors
            .points
            .map(|x| x.map(|t| format!("{}", t)).join(""))
            .join("\n");
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod test {
    use crate::dungeon::{Dungeon, Field};
    use std::mem::size_of;

    #[test]
    fn test() {
        println!("{}", size_of::<Field<u8>>());
        println!("{}", size_of::<Dungeon>());
        println!("{}", Dungeon::default());
    }
}
