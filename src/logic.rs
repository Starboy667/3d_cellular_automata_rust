use bevy::{math::IVec3, utils::hashbrown::HashSet};

use crate::{
    render::CellRenderer,
    rule::Rule,
    utils::{index_to_pos, out_of_bounds, pos_to_index, random_cells},
};

#[derive(Debug, Clone, Copy)]
struct Cell {
    value: u8,
    neighbors: u8,
}

pub struct Logic {
    cells: Vec<Cell>,
    bounds: i32,
}

impl Logic {
    pub fn new(bounds: i32) -> Self {
        let mut cells = vec![];
        cells.resize(
            (bounds * bounds * bounds) as usize,
            Cell {
                value: 0,
                neighbors: 0,
            },
        );
        let it = random_cells(bounds, 0.3);
        for (x, y, z) in it {
            let index = pos_to_index(&IVec3::new(x as i32, y as i32, z as i32), &bounds);
            cells[index].value = 1;
        }
        Self { cells, bounds }
    }

    pub fn update(&mut self, rule_handler: &Rule) {
        // println!("len {}", self.cells.len());
        // println!("bounds {}", self.bounds);

        // if self.cells.len() != (self.bounds * self.bounds * self.bounds) as usize {
        //     println!("resize");
        //     self.cells.clear();
        //     self.cells.resize(
        //         (self.bounds * self.bounds * self.bounds) as usize,
        //         Cell {
        //             value: 0,
        //             neighbors: 0,
        //         },
        //     );
        // }
        // TODO add noise at start + egui
        let mut dead: Vec<usize> = vec![];
        let mut alive: Vec<usize> = vec![];
        for (index, cell) in self.cells.iter_mut().enumerate() {
            // check spawn
            if cell.value == 0 && rule_handler.alive[cell.neighbors as usize] {
                cell.value = rule_handler.states;
                alive.push(index);
            }
            // check survive
            if cell.value < rule_handler.states || !rule_handler.dead[cell.neighbors as usize] {
                if cell.value == rule_handler.states {
                    dead.push(index);
                }
                if cell.value > 0 {
                    cell.value -= 1;
                }
            }
        }
        // let neighbor_dead: Vec<IVec3> = dead
        //     .iter()
        //     .flat_map(|pos| {
        //         rule_handler
        //             .get_neighbors_iter()
        //             .iter()
        //             .map(|neighbor| *pos + *neighbor)
        //             .collect::<Vec<IVec3>>()
        //     })
        //     .collect();
        // let neighbor_alive: Vec<IVec3> = alive
        //     .iter()
        //     .flat_map(|pos| {
        //         rule_handler
        //             .get_neighbors_iter()
        //             .iter()
        //             .map(|neighbor| *pos + *neighbor)
        //             .collect::<Vec<IVec3>>()
        //     })
        //     .collect();
        // self.update_neighbors(None, rule_handler);
        let merged: Vec<usize> = dead.into_iter().chain(alive.into_iter()).collect();
        let unique_set: HashSet<_> = merged.into_iter().collect();
        let result: Vec<_> = unique_set.into_iter().collect();
        // println!("result {:?}", result);
        // self.update_neighbors(Some(neighbor_dead), rule_handler);
        self.update_neighbors(Some(result), rule_handler)
    }

    pub fn update_neighbors(&mut self, pos: Option<Vec<usize>>, rule: &Rule) {
        let cells = match pos {
            Some(p) => p
                .iter()
                .flat_map(|pos| {
                    rule.get_neighbors_iter().iter().map(|neighbor| {
                        let pos = index_to_pos(pos, &self.bounds);
                        pos + *neighbor
                    })
                })
                .collect::<Vec<IVec3>>(),
            None => {
                let mut positions = Vec::new();
                for x in 0..self.bounds {
                    for y in 0..self.bounds {
                        for z in 0..self.bounds {
                            let pos = IVec3::new(x, y, z);
                            positions.push(pos);
                        }
                    }
                }
                positions
            }
        };

        for pos in cells {
            if out_of_bounds(&pos, &self.bounds) {
                continue;
            }
            let mut neighbors = 0;
            for neighbor in rule.get_neighbors_iter() {
                let neighbor_pos = pos + *neighbor;
                if out_of_bounds(&neighbor_pos, &self.bounds) {
                    continue;
                }
                let neighbor_index = pos_to_index(&neighbor_pos, &self.bounds);
                if self.cells[neighbor_index].value == 1 {
                    neighbors += 1;
                }
            }
            let index = pos_to_index(&pos, &self.bounds);
            self.cells[index].neighbors = neighbors;
        }
    }

    pub fn render(&self, renderer: &mut CellRenderer) {
        renderer.clear();
        for (index, cell) in self.cells.iter().enumerate() {
            renderer.set(index, cell.value, cell.neighbors);
        }
    }
}
