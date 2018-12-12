const SERIAL_NUMBER: i32 = 1723;
const GRID_WIDTH: usize = 300;
const GRID_HEIGHT: usize = 300;

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut grid = FuelGrid::new(SERIAL_NUMBER);
    let square = grid.find_best_square_with_size(3);
    println!(
        "The best place to get a 3x3 fuel square is at {:?}",
        square.corner
    );
}

fn part2() {
    let mut grid = FuelGrid::new(SERIAL_NUMBER);
    let square = grid.find_best_square();
    println!(
        "The best place to get a {size}x{size} fuel square is at {corner:?}",
        size = square.size,
        corner = square.corner
    );
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Coordinate(usize, usize);

struct FuelGrid {
    sum_grid: [[i32; GRID_WIDTH + 1]; GRID_HEIGHT + 1],
}

#[derive(Clone, Debug)]
struct Square {
    corner: Coordinate,
    power_level: i32,
    size: usize,
}

impl FuelGrid {
    fn new(serial_number: i32) -> FuelGrid {
        let mut grid = [[0i32; GRID_WIDTH + 1]; GRID_HEIGHT + 1];
        let mut sum_grid = [[0i32; GRID_WIDTH + 1]; GRID_HEIGHT + 1];

        for y in 1..=GRID_HEIGHT {
            for x in 1..=GRID_WIDTH {
                grid[y][x] = power_level(Coordinate(x, y), serial_number);
            }
        }

        for y in 1..=GRID_HEIGHT {
            for x in 1..=GRID_HEIGHT {
                sum_grid[y][x] = grid[y - 1][x - 1] + sum_grid[y][x - 1] + sum_grid[y - 1][x]
                    - sum_grid[y - 1][x - 1];
            }
        }

        FuelGrid { sum_grid }
    }

    fn find_best_square(&mut self) -> Square {
        (1..=GRID_WIDTH - 1)
            .map(|size| self.find_best_square_with_size(size))
            .max_by_key(|square| square.power_level)
            .unwrap()
    }

    fn find_best_square_with_size(&mut self, size: usize) -> Square {
        let mut squares = Vec::new();
        for y in 1..=GRID_WIDTH {
            for x in 1..=GRID_HEIGHT {
                if let Some(square) = self.get_square(Coordinate(x, y), size) {
                    squares.push(square);
                }
            }
        }
        squares
            .into_iter()
            .max_by_key(|square| square.power_level)
            .expect(&format!("No best square for {}", size))
    }

    fn get_square(&self, corner: Coordinate, size: usize) -> Option<Square> {
        let bottom_right = Coordinate(corner.0 + size, corner.1 + size);

        if bottom_right.0 > GRID_WIDTH || bottom_right.1 > GRID_HEIGHT {
            return None;
        }

        let top_right = Coordinate(corner.0 + size, corner.1);
        let bottom_left = Coordinate(corner.0, corner.1 + size);

        let a = self.get_sum(&corner);
        let b = self.get_sum(&top_right);
        let c = self.get_sum(&bottom_left);
        let d = self.get_sum(&bottom_right);

        let power_level = d - b - c + a;

        Some(Square {
            corner,
            power_level,
            size,
        })
    }

    fn get_sum(&self, coordinate: &Coordinate) -> i32 {
        self.sum_grid[coordinate.1][coordinate.0]
    }
}

fn power_level(coordinate: Coordinate, serial_number: i32) -> i32 {
    let rack_id = (coordinate.0 as i32) + 10;
    let mut power_level = rack_id * (coordinate.1 as i32);
    power_level += serial_number;
    power_level *= rack_id;
    power_level = (power_level % 1000) / 100;
    power_level - 5
}
