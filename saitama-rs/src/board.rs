// // EASY MODE: Implement a 3 x 3 board grid
// // HARD MODE: Implement an n x n board

enum Marker{X, O, Empty}

pub struct Grid {
    row_one: (Marker, Marker, Marker),
    row_two: (Marker, Marker, Marker),
    row_three: (Marker, Marker, Marker)
}
