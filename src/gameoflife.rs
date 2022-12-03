use std::{cell::RefCell, fmt::Debug, rc::Rc};

use itertools::Itertools;

pub struct GolCell {
    pub index: (u32, u32),
    neighbors: Vec<Rc<RefCell<GolCell>>>,
    pub alive: bool,
}

impl GolCell {
    pub fn new(x: u32, y: u32) -> GolCell {
        GolCell {
            index: (x, y),
            neighbors: vec![],
            alive: false,
        }
    }

    pub fn neighbors_alive(&self) -> usize {
        self.neighbors.iter().filter(|n| n.borrow().alive).count()
    }
}

impl Debug for GolCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cell {{ index: {:?}, alive: {:?}, neighbors: [{:}]}}",
            self.index,
            self.alive,
            self.neighbors
                .iter()
                .map(|n| format!("{:?}", n.borrow().index))
                .join(", ")
        )
    }
}

#[derive(Debug)]
pub struct Game {
    cells_counts: (u32, u32),
    pub cells: Vec<Rc<RefCell<GolCell>>>,
}

impl Game {
    pub fn new(count_x: u32, count_y: u32) -> Game {
        let mut game = Game {
            cells_counts: (count_x, count_y),
            cells: vec![],
        };
        for y in 0..count_y {
            for x in 0..count_x {
                game.cells.push(Rc::new(RefCell::new(GolCell::new(x, y))));
            }
        }
        // println!("cells: {:#?}", game.cells);
        game.populate_neighbors();
        game
    }

    fn populate_neighbors(&mut self) {
        for c in self.cells.iter() {
            let cindex = c.borrow().index;
            for (dx, dy) in (-1 as i32..=1).cartesian_product(-1 as i32..=1) {
                if (dx, dy) == (0, 0) {
                    continue;
                }
                if let Some(s) = self.cell_at_index(cindex.0 as i32 + dx, cindex.1 as i32 + dy) {
                    assert!(
                        cindex.0 as i32 + dx == s.borrow().index.0 as i32
                            && cindex.1 as i32 + dy == s.borrow().index.1 as i32,
                        "c: {:?}, dxy: ({:?}, {:?}), s: {:?}",
                        cindex,
                        dx,
                        dy,
                        s.borrow().index
                    );
                    c.borrow_mut().neighbors.push(s);
                }
            }
        }
    }

    pub fn cell_at_index(&self, index_x: i32, index_y: i32) -> Option<Rc<RefCell<GolCell>>> {
        if index_x < 0
            || index_y < 0
            || index_x >= self.cells_counts.0 as i32
            || index_y >= self.cells_counts.1 as i32
        {
            None
        } else if let Some(c) = self
            .cells
            .get((self.cells_counts.0 * index_y as u32 + index_x as u32) as usize)
        {
            Some(c.clone())
        } else {
            None
        }
    }

    pub fn iteration(&mut self) {
        // println!("Iteration");
        let mut next_state = Vec::<bool>::new();
        for cell in self.cells.iter() {
            let cell = cell.borrow();
            // println!("{:?} -> {:}", cell, cell.neighbors_alive());
            // 1. Any live cell with two or three live neighbours survives.
            // 2. Any dead cell with three live neighbours becomes a live cell.
            // 3. All other live cells die in the next generation.
            //    Similarly, all other dead cells stay dead.
            next_state.push(match cell.neighbors_alive() {
                2 => cell.alive,
                3 => true,
                _ => false,
            });
            // if *next_state.last().unwrap() != cell.alive {
            //     println!(
            //         "Update {:?} to {:?} due to {:?}",
            //         cell.index,
            //         !cell.alive,
            //         cell.neighbors_alive()
            //     );
            // }
        }
        for (cell, newstate) in self.cells.iter_mut().zip(next_state) {
            cell.borrow_mut().alive = newstate;
        }
    }
}
