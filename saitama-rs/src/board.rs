// EASY MODE: Implement a 3 x 3 board grid
// HARD MODE: Implement an n x n board

use crate::board::Marker::Empty;
use crate::world::World;

enum Marker<'x, 'o> {
    X(&'x str),
    O(&'o str),
    Empty,
}

struct Row<'x, 'o>(Marker<'x, 'o>, Marker<'x, 'o>, Marker<'x, 'o>);

pub enum Board<'x, 'o> {
    Grid {
        row_one: Row<'x, 'o>,
        row_two: Row<'x, 'o>,
        row_three: Row<'x, 'o>,
    },
    Dynamic,
}

impl Board<'_, '_> {
    pub fn build<'x, 'o>(world: World) -> Self {
        let mark = world.player_marker;

        match world.difficulty {
            "EASY" => Board::new_grid(),
            "HARD" => Board::new_dynamic(),
        }
    }

    pub fn choose_position(self) -> String {
        String::from("maybe?")
    }

    fn new_grid() -> Self {
        return Board::Grid {
            row_one: Row(Empty, Empty, Empty),
            row_two: Row(Empty, Empty, Empty),
            row_three: Row(Empty, Empty, Empty),
        };
    }

    fn new_dynamic() -> Self {
        return Board::Dynamic
    }
}
