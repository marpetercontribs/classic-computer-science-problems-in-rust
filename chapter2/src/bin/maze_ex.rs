// maze_ex2.rs
// Solution to exercise 2
// in Classic Computer Science Problems in Python/Java Chapter 2
// Copyright 2023 Markus Peter
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use rand::Rng;

#[derive(PartialEq)]
enum Cell {
    Empty,
    Blocked,
    Start,
    Goal,
}

#[derive(PartialEq, PartialOrd, Clone, Eq, Hash, Copy)]
struct MazeLocation {
    row: usize,
    column: usize,
}

struct Maze {
    rows: usize,
    columns: usize,
    start: MazeLocation,
    goal: MazeLocation,
    grid: Vec<Vec<Cell>>,
}
impl Maze {
    fn new(
        rows: usize,
        columns: usize,
        start: MazeLocation,
        goal: MazeLocation,
        sparseness: f32,
    ) -> Self {
        let mut grid = Maze::randomly_fill(rows, columns, sparseness);
        grid[start.row][start.column] = Cell::Start;
        grid[goal.row][goal.column] = Cell::Goal;
        Maze {
            rows,
            columns,
            start,
            goal,
            grid,
        }
    }
    fn default_new() -> Self {
        Maze::new(
            10,
            10,
            MazeLocation { row: 0, column: 0 },
            MazeLocation { row: 9, column: 9 },
            0.2,
        )
    }

    fn randomly_fill(rows: usize, columns: usize, sparseness: f32) -> Vec<Vec<Cell>> {
        let mut grid = Vec::new();
        let mut rng = rand::rng();
        for _ in 0..rows {
            let mut row = Vec::<Cell>::new();
            for _ in 0..columns {
                if rng.random_range(0.0..1.0) < sparseness {
                    row.push(Cell::Blocked);
                } else {
                    row.push(Cell::Empty);
                }
            }
            grid.push(row);
        }
        grid
    }

    fn goal_test(&self, ml: &MazeLocation) -> bool {
        self.goal == *ml
    }

    fn successors(&self, ml: &MazeLocation) -> Vec<MazeLocation> {
        let mut locations = Vec::new();
        if (ml.row + 1 < self.rows) && (self.grid[ml.row + 1][ml.column] != Cell::Blocked) {
            locations.push(MazeLocation {
                row: ml.row + 1,
                column: ml.column,
            });
        }
        if (ml.row > 0) && (self.grid[ml.row - 1][ml.column] != Cell::Blocked) {
            locations.push(MazeLocation {
                row: ml.row - 1,
                column: ml.column,
            });
        }
        if (ml.column + 1 < self.columns) && (self.grid[ml.row][ml.column + 1] != Cell::Blocked) {
            locations.push(MazeLocation {
                row: ml.row,
                column: ml.column + 1,
            });
        }
        if (ml.column > 0) && (self.grid[ml.row][ml.column - 1] != Cell::Blocked) {
            locations.push(MazeLocation {
                row: ml.row,
                column: ml.column - 1,
            });
        }
        locations
    }

    fn manhattan_distance(&self, ml: &MazeLocation) -> f64 {
        let x_distance = (ml.column as f64) - (self.goal.column as f64);
        let y_distance = (ml.row as f64) - (self.goal.row as f64);
        x_distance.abs() + y_distance.abs()
    }
}

fn main() {
    let mut sum_solved_dfs = 0;
    let mut sum_no_solution_dfs = 0;
    let mut sum_solved_bfs = 0;
    let mut sum_no_solution_bfs = 0;
    let mut sum_solved_astar = 0;
    let mut sum_no_solution_astar = 0;
    let mut total_explored_dfs = 0;
    let mut total_explored_bfs = 0;
    let mut total_explored_astar = 0;
    let mut total_considered_dfs = 0;
    let mut total_considered_bfs = 0;
    let mut total_considered_astar = 0;

    let runs = 1000;

    for _ in 0..runs {
        let maze = Maze::default_new();
        let (solution1, explored, considered) = generic_search::dfs_counting(
            maze.start,
            |ml: &MazeLocation| maze.goal_test(ml),
            |ml: &MazeLocation| maze.successors(ml),
        );
        match solution1 {
            None => sum_no_solution_dfs += 1,
            Some(_) => sum_solved_dfs += 1,
        }
        total_explored_dfs += explored;
        total_considered_dfs += considered;

        let (solution2, explored, considered) = generic_search::bfs_counting(
            maze.start,
            |ml: &MazeLocation| maze.goal_test(ml),
            |ml: &MazeLocation| maze.successors(ml),
        );
        match solution2 {
            None => sum_no_solution_bfs += 1,
            Some(_) => sum_solved_bfs += 1,
        }
        total_explored_bfs += explored;
        total_considered_bfs += considered;

        let (solution3, explored, considered) = generic_search::astar_counting(
            maze.start,
            |ml: &MazeLocation| maze.goal_test(ml),
            |ml: &MazeLocation| maze.successors(ml),
            |ml: &MazeLocation| maze.manhattan_distance(ml),
        );
        match solution3 {
            None => sum_no_solution_astar += 1,
            Some(_) => sum_solved_astar += 1,
        }
        total_explored_astar += explored;
        total_considered_astar += considered;
    }
    println!(
        "Depth-first search   found {sum_solved_dfs} solutions, tested {} states and considered {} on average.",
        total_explored_dfs / runs,
        total_considered_dfs / runs
    );
    println!(
        "Breadth-first search found {sum_solved_bfs} solutions, tested {} states and considered {} on average.",
        total_explored_bfs / runs,
        total_considered_bfs / runs
    );
    println!(
        "A-Star search        found {sum_solved_astar} solutions, tested {} states and considered {} on average.",
        total_explored_astar / runs,
        total_considered_astar / runs
    );
    println!("No solutions were found in {sum_no_solution_dfs} / {sum_no_solution_bfs} / {sum_no_solution_astar} cases.");
}
