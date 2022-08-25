use crate::grid::Direction;
use crate::random::Random;

const AREA_SIZE_MIN: usize = 8;
const AREA_PADDING: usize = 2;
const ROOM_SIZE_MIN: usize = AREA_SIZE_MIN - AREA_PADDING * 2;

struct Area {
    kind: AreaKind,
    x: usize,
    y: usize,
    rows: usize,
    columns: usize,
}

impl Area {
    fn gen<R: Random>(rnd: &mut R, x: usize, y: usize, rows: usize, columns: usize) -> Self {
        assert!(rows >= AREA_SIZE_MIN);
        assert!(columns >= AREA_SIZE_MIN);
        let kind_selection = rnd.gen_u8();
        let kind = match kind_selection {
            0..3 => AreaKind::WallFilled,
            3..16 => {
                let relay_x = rnd.gen_max(rows as u32) as usize;
                let relay_y = rnd.gen_max(columns as u32) as usize;
                AreaKind::PathWay { relay_x, relay_y }
            }
            _ => {
                let room_rows = AREA_SIZE_MIN + (rnd.gen_max((rows - AREA_SIZE_MIN) as u32) as usize);
                let room_columns = AREA_SIZE_MIN + (rnd.gen_max((columns - AREA_SIZE_MIN) as u32) as usize);
                let room_x = AREA_PADDING + (rnd.gen_max((rows - room_rows - AREA_PADDING * 2) as u32) as usize);
                let room_y = AREA_PADDING + (rnd.gen_max((columns - room_columns - AREA_PADDING * 2) as u32) as usize);
                AreaKind::Room { x: room_x, y: room_y, row: room_rows, column: room_columns }
            }
        };
        Self {
            kind, x, y, rows, columns,
        }
    }

    fn reach_out<R: Random>(&self, rnd: &mut R, direction: Direction) -> ((usize, usize), (usize, usize)) {
        match self {
            let
            AreaKind::Room { x, y, row, column } => {
                if vertical {
                    rnd.gen_max()
                }
            }
            AreaKind::PathWay { .. } => {}
            AreaKind::WallFilled => {}
            AreaKind::None => {}
        }
    }
}

#[derive(Copy, Clone)]
enum AreaKind {
    Room { x: usize, y: usize, row: usize, column: usize },
    PathWay { relay_x: usize, relay_y: usize },
    WallFilled,
    None,
}

impl AreaKind {
}

