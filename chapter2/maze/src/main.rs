//maze.rs
// Adapted From Classic Computer Science Problems in Python/Java Chapter 2
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
use std::fmt;
use rand::{thread_rng, Rng};

#[derive(Debug,PartialEq)]
enum Cell { Empty, Blocked, Start, Goal, Path }
impl fmt::Display for Cell {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Empty => write!(formatter, " "),
            Cell::Blocked => write!(formatter, "X"),
            Cell::Start => write!(formatter, "S"),
            Cell::Goal => write!(formatter, "G"),
            Cell::Path => write!(formatter, "*")
        }
    }
}

#[derive(Debug,PartialEq)]
struct MazeLocation {
    row: usize,
    column: usize,
}
impl fmt::Display for MazeLocation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "({},{})",self.row,self.column)
    }
}

#[derive(Debug)]
struct Maze {
    rows: usize,
    columns: usize,
    start: MazeLocation,
    goal: MazeLocation,
    grid: Vec<Vec<Cell>>,
}

impl Maze {
    fn new(rows: usize, columns: usize, start: MazeLocation, goal: MazeLocation, sparseness: f32) -> Self {
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
        Maze::new(10, 10, MazeLocation {row: 0, column: 0}, MazeLocation {row: 9, column: 9}, 0.2)
     }

     fn randomly_fill(rows: usize, columns: usize, sparseness: f32) -> Vec<Vec<Cell>> {
        let mut grid = Vec::new();
        let mut rng = thread_rng();
        for row in 0..rows {
            let mut row = Vec::<Cell>::new();
            for column in 0..columns {
                if rng.gen_range(0.0 .. 1.0) < sparseness {
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
            locations.push(MazeLocation { row: ml.row + 1, column: ml.column });
        }
        if (ml.row > 0) && (self.grid[ml.row - 1][ml.column] != Cell::Blocked) {
            locations.push(MazeLocation { row: ml.row - 1, column: ml.column });
        }
        if (ml.column + 1 < self.columns) && (self.grid[ml.row][ml.column + 1] != Cell::Blocked) {
            locations.push(MazeLocation { row: ml.row + 1, column: ml.column });
        }
        if (ml.column > 0) && (self.grid[ml.row][ml.column -  1] != Cell::Blocked) {
            locations.push(MazeLocation { row: ml.row + 1, column: ml.column });
        }               
        locations
     }
}
impl fmt::Display for Maze {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            for cell in row {
                write!(formatter, "{}", cell);
            }
            write!(formatter,"\n");
        }
        write!(formatter,"")
    }    
}

fn main() {
    let maze = Maze::default_new();
    println!("{}", maze);
    let true_goal = MazeLocation { row: maze.goal.row, column: maze.goal.column };
    let wrong_goal = MazeLocation { row: 5, column: 5};
    println!("{}", maze.goal_test(&wrong_goal));
    println!("{}", maze.goal_test(&true_goal));
}
