use crate::random::Random;
use std::fmt::{Debug, Display, Formatter};

pub const FIELD_ROW_SIZE: usize = 41;
pub const FIELD_COLUMN_SIZE: usize = 63;
pub const FIELD_CHIPS: usize = FIELD_ROW_SIZE * FIELD_COLUMN_SIZE;

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

impl Field<Floor> {
    fn gen_field_tree_dip<R: Random>(rnd: &mut R, rows: usize, columns: usize) -> Self {
        assert!(FIELD_ROW_SIZE / rows >= 10);
        assert!(FIELD_COLUMN_SIZE / columns >= 10);
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
                    FIELD_ROW_SIZE / rows - 1,
                    FIELD_COLUMN_SIZE / columns - 1,
                );
                if matches!(areas[i * columns + j], FieldFloorArea::Room { .. }) {
                    room_cnt += 1;
                }
            }
        }
        if room_cnt < 4 {
            return Self::gen_field_tree_dip(rnd, rows, columns);
        }

        let mut areas_groups = [0usize; 24];
        for i in 0..(rows*columns) {
            areas_groups[i] = i+1
        }
        // todo 木チックに経路を生成する。
        loop {
            let vertical = rnd.gen_max(2) == 1;
            let si = rnd.gen_max((rows * columns) as u32) as usize;
            let ti = if vertical {
                if si + columns >= rows * columns {
                    continue;
                }
                si + columns
            } else {
                if (si + 1) % columns == 0 {
                    continue;
                }
                si + 1
            };
            if areas_groups[si] == areas_groups[ti] {
                continue;
            }
            let overwrite_group = areas_groups[ti];
            for i in 0..(rows*columns) {
                if areas_groups[i] == overwrite_group {
                    areas_groups[i] = areas_groups[si];
                }
            }
        }
    }
}

impl<T: FieldPointDefault + Copy> Default for Field<T> {
    fn default() -> Field<T> {
        let mut points = [[T::default_inter(); FIELD_COLUMN_SIZE]; FIELD_ROW_SIZE];
        for i in 0..FIELD_COLUMN_SIZE {
            points[0][i] = T::default_edge();
            points[FIELD_ROW_SIZE - 1][i] = T::default_edge();
        }
        for i in 0..FIELD_ROW_SIZE {
            points[i][0] = T::default_edge();
            points[i][FIELD_COLUMN_SIZE - 1] = T::default_edge();
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
