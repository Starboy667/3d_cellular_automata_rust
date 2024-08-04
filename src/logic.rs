use bevy::math::IVec3;
use noise::{NoiseFn, Perlin};

use crate::{
    render::CellRenderer,
    rule::Rule,
    utils::{index_to_pos, out_of_bounds, pos_to_index},
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
    pub fn new() -> Self {
        Logic {
            cells: vec![],
            bounds: 0,
        }
    }

    fn wrap(&self, pos: IVec3) -> IVec3 {
        (pos + self.bounds) % self.bounds
    }

    pub fn set_size(&mut self, bounds: i32) {
        self.bounds = bounds;
        self.cells.clear();
        self.cells.resize(
            (bounds * bounds * bounds) as usize,
            Cell {
                value: 0,
                neighbors: 0,
            },
        );
    }

    pub fn update(&mut self, rule_handler: &Rule) {
        let mut death: Vec<usize> = vec![];
        let mut birth: Vec<usize> = vec![];
        // for (index, cell) in self.cells.iter_mut().enumerate() {
        //     // check spawn
        //     if cell.value == 0 && rule_handler.birth[cell.neighbors as usize] {
        //         cell.value = rule_handler.states;
        //         birth.push(index);
        //     }
        //     // check survive
        //     if cell.value != 0
        //         && (cell.value < rule_handler.states
        //             || !rule_handler.survive[cell.neighbors as usize])
        //     {
        //         if cell.value == rule_handler.states {
        //             death.push(index);
        //         }
        //         cell.value -= 1;
        //     }
        // }
        for (index, cell) in self.cells.iter_mut().enumerate() {
            if cell.value == 0 {
                if rule_handler.birth[cell.neighbors as usize] {
                    cell.value = rule_handler.states;
                    birth.push(index);
                }
            } else {
                if cell.value < rule_handler.states
                    || !rule_handler.survive[cell.neighbors as usize]
                {
                    if cell.value == rule_handler.states {
                        death.push(index);
                    }
                    cell.value -= 1;
                }
            }
        }
        for index in birth {
            self.test(rule_handler, index, true);
        }
        for index in death {
            self.test(rule_handler, index, false);
        }
    }

    fn test(&mut self, rule: &Rule, index: usize, inc: bool) {
        let pos = index_to_pos(&index, &self.bounds);
        if out_of_bounds(&pos, &self.bounds) {
            return;
        }
        for dir in rule.get_neighbors_iter() {
            let neighbor_pos = self.wrap(pos + *dir);
            if out_of_bounds(&neighbor_pos, &self.bounds) {
                continue;
            }

            let index = pos_to_index(&neighbor_pos, &self.bounds);
            if inc {
                self.cells[index].neighbors += 1;
            } else {
                self.cells[index].neighbors -= 1;
            }
        }
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
                if self.cells[neighbor_index].value >= 1 {
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

    pub fn make_some_noise(&mut self, rule: &Rule) {
        let scale = 0.1;
        let perlin = Perlin::new(1);
        for i in 0..self.bounds {
            for j in 0..self.bounds {
                for k in 0..self.bounds {
                    let index = pos_to_index(&IVec3::new(i, j, k), &self.bounds);
                    let val = perlin.get([i as f64 * scale, j as f64 * scale, k as f64 * scale]);
                    if val > 0.7 {
                        self.cells[index].value = rule.states;
                        self.test(rule, index, true);
                    }
                }
            }
        }
    }
}
