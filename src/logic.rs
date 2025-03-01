const WIDTH: i8 = 7;
const HEIGHT: i8 = 6;

const DIRECTIONS: [[i8; 2]; 8] = [
    [0, -1],
    [1, -1],
    [1, 0],
    [1, 1],
    [0, 1],
    [-1, 1],
    [-1, 0],
    [-1, -1],
];

fn in_bounds(x: i8, y: i8) -> bool {
    x >= 0 && x < WIDTH && y >= 0 && y < HEIGHT
}

fn search(x: i8, y: i8, val: u8, dir: [i8; 2], count: i8, grid: &[[u8; 7]; 6]) -> bool {
    let sx = x as i8 + dir[0];
    let sy = y as i8 + dir[1];

    if in_bounds(sx, sy) {
        let sibling_cell = grid[sy as usize][sx as usize];
        if sibling_cell == val {
            if count < 4 {
                return search(sx, sy, val, dir, count + 1, grid);
            } else if count == 4 {
                return true;
            }
        }
    }

    return false;
}

pub fn is_connect_four(grid: [[u8; 7]; 6]) -> bool {
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell != 0u8 {
                // Search siblings for matches
                for dir in DIRECTIONS {
                    // Count starts at 2 since we've already confirmed the first match
                    if search(x as i8, y as i8, *cell, dir, 2, &grid) {
                        return true;
                    } else {
                        continue;
                    }
                }
            }
        }
    }

    return false;
}

pub fn is_tie_game(grid: [[u8; 7]; 6]) -> bool {
    for row in grid.iter() {
        for cell in row.iter() {
            if *cell == 0u8 {
                return false;
            }
        }
    }
    return true;
}

pub fn get_lowest_tile_position(col: usize, row: usize, grid: [[u8; 7]; 6]) -> [usize; 2] {
    let mut row = row;
    while row < 5 {
        // Check next row down for tile
        let cell = grid[row + 1][col];
        if cell == 0 {
            row += 1;
        } else {
            break;
        }
    }
    [col, row]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect_four_false() {
        let grid: [[u8; 7]; 6] = [
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
        ];

        assert_eq!(is_connect_four(grid), false);
    }

    #[test]
    fn test_connect_four_up_down_diag_true() {
        let grid: [[u8; 7]; 6] = [
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [1, 2, 0, 0, 0, 0, 0],
            [2, 1, 2, 0, 0, 0, 0],
            [1, 2, 1, 2, 0, 0, 0],
            [1, 2, 2, 1, 0, 0, 0],
        ];

        assert_eq!(is_connect_four(grid), true);
    }

    #[test]
    fn test_connect_four_down_up_diag_true() {
        let grid: [[u8; 7]; 6] = [
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 0],
            [2, 1, 2, 0, 0, 0, 0],
            [1, 2, 1, 0, 0, 0, 0],
            [2, 2, 2, 1, 0, 0, 0],
        ];

        assert_eq!(is_connect_four(grid), true);
    }

    #[test]
    fn test_connect_four_down_up_diag_false() {
        let grid: [[u8; 7]; 6] = [
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 0],
            [0, 2, 0, 0, 0, 0, 0],
            [1, 2, 1, 0, 0, 0, 0],
            [0, 2, 0, 1, 0, 0, 0],
        ];

        assert_eq!(is_connect_four(grid), false);
    }

    #[test]
    fn test_connect_four_up_down_true() {
        let grid: [[u8; 7]; 6] = [
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [1, 0, 2, 0, 0, 0, 0],
            [0, 1, 2, 0, 0, 0, 0],
            [0, 0, 2, 0, 0, 0, 0],
            [0, 0, 2, 2, 0, 0, 0],
        ];

        assert_eq!(is_connect_four(grid), true);
    }

    #[test]
    fn test_is_tie_game_true() {
        let grid: [[u8; 7]; 6] = [
            [1, 2, 2, 1, 2, 1, 2],
            [2, 1, 2, 1, 1, 2, 2],
            [1, 2, 2, 1, 2, 1, 2],
            [2, 1, 2, 1, 2, 2, 1],
            [2, 1, 2, 2, 1, 1, 2],
            [2, 2, 2, 2, 2, 2, 1],
        ];

        assert_eq!(is_tie_game(grid), true);
    }

    #[test]
    fn test_is_tie_game_false() {
        let grid: [[u8; 7]; 6] = [
            [1, 2, 2, 1, 2, 1, 2],
            [2, 1, 2, 1, 1, 2, 2],
            [1, 2, 2, 1, 2, 1, 2],
            [2, 1, 2, 1, 2, 2, 1],
            [2, 1, 2, 2, 1, 1, 2],
            [2, 2, 2, 2, 2, 2, 0],
        ];

        assert_eq!(is_tie_game(grid), false);
    }

    #[test]
    fn test_get_lowest_tile_position() {
        let grid: [[u8; 7]; 6] = [
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 1],
            [0, 0, 0, 0, 0, 0, 2],
            [0, 0, 0, 0, 0, 0, 1],
        ];

        let res = get_lowest_tile_position(6, 0, grid);

        assert_eq!(res, [6, 2]);
    }
}
