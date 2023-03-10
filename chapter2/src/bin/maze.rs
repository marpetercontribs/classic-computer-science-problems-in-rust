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
use rand::{thread_rng, Rng};
use generic_search;

#[derive(PartialEq)]
enum Cell { Empty, Blocked, Start, Goal, Path }
impl ToString for Cell {
    fn to_string(&self) -> String {
        match self {
            Cell::Empty   => ' '.to_string(),
            Cell::Blocked => 'X'.to_string(),
            Cell::Start   => 'S'.to_string(),
            Cell::Goal    => 'G'.to_string(),
            Cell::Path    => '*'.to_string()
        }
    }
}

#[derive(PartialEq, PartialOrd, Clone, Eq, Hash, Copy)]
struct MazeLocation {
    row: usize,
    column: usize,
}

impl ToString for MazeLocation {
    fn to_string(&self) -> String {
        let mut result = String::from("(");
        result.push_str(&self.row.to_string());
        result.push(',');
        result.push_str(&self.column.to_string());
        result.push(')');
        result
    }
}

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
        for _ in 0..rows {
            let mut row = Vec::<Cell>::new();
            for _ in 0..columns {
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
            locations.push(MazeLocation { row: ml.row, column: ml.column + 1});
        }
        if (ml.column > 0) && (self.grid[ml.row][ml.column -  1] != Cell::Blocked) {
            locations.push(MazeLocation { row: ml.row, column: ml.column - 1});
        }               
        locations
     }

     fn mark(&mut self, path: &Vec<MazeLocation>) {
        for ml in path {
            self.grid[ml.row][ml.column] = Cell::Path;
        }
        self.grid[self.start.row][self.start.column] = Cell::Start;
        self.grid[self.goal.row][self.goal.column] = Cell::Goal;
     }

     fn clear(&mut self, path: &Vec<MazeLocation>) {
        for ml in path {
            self.grid[ml.row][ml.column] = Cell::Empty;
        }
        self.grid[self.start.row][self.start.column] = Cell::Start;
        self.grid[self.goal.row][self.goal.column] = Cell::Goal;        
     }

     fn euclidean_distance(&self, ml: &MazeLocation) -> f64 {
        let x_distance = (ml.column - self.goal.column) as f64;
        let y_distance = (ml.row - self.goal.row) as f64;
        return (x_distance*x_distance + y_distance*y_distance).sqrt()
     }

     fn manhattan_distance(&self, ml: &MazeLocation) -> f64 {
        let x_distance = (ml.column as f64) - (self.goal.column as f64);
        let y_distance = (ml.row as f64)- (self.goal.row as f64);
        return x_distance.abs() + y_distance.abs()
     }
}
impl ToString for Maze {
    fn to_string(&self) -> String {
        let mut result = String::new();
        for row in &self.grid {
            for cell in row {
                result.push_str(&cell.to_string());
            }
            result.push('\n');
        }
        result
    }    
}

fn main() {
    let mut maze = Maze::default_new();
    let solution1 = crate::generic_search::dfs(maze.start,
        |ml: &MazeLocation| maze.goal_test(&ml),
        |ml: &MazeLocation| maze.successors(&ml));
    match solution1 {
        None => println!("{}", "No solution found using depth-first search"),
        Some(node) => {
            let path = generic_search::node_to_path(&node);
            maze.mark(&path);
            println!("{}", &maze.to_string());
            maze.clear(&path)
        }
    }
    println!("----------");
    let solution2 = generic_search::bfs(maze.start,
        |ml: &MazeLocation| maze.goal_test(&ml),
        |ml: &MazeLocation| maze.successors(&ml));
    match solution2 {
        None => println!("{}", "No solution found using breadth-first search"),
        Some(node) => {
            let path = generic_search::node_to_path(&node);
            maze.mark(&path);
            println!("{}", &maze.to_string());
            maze.clear(&path)
        }
    }
    println!("----------");
    let solution3 = generic_search::astar(maze.start,
        |ml: &MazeLocation| maze.goal_test(&ml),
        |ml: &MazeLocation| maze.successors(&ml),
        |ml: &MazeLocation| maze.manhattan_distance(&ml));
    match solution3 {
        None => println!("{}", "No solution found using astar search"),
        Some(node) => {
            let path = generic_search::node_to_path(&node);
            maze.mark(&path);
            println!("{}", &maze.to_string());
            maze.clear(&path)
        }
    }
}
