use crate::{
    render::CellRenderer,
    rule::Rule,
    utils::{index_to_pos, out_of_bounds},
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
        Self {
            cells: vec![],
            bounds: bounds,
        }
    }
    pub fn update(&mut self, rule_handler: &Rule) {
        // println!("len {}", self.cells.len());
        // println!("bounds {}", self.bounds);
        if self.cells.len() != (self.bounds * self.bounds * self.bounds) as usize {
            println!("resize");
            self.cells.clear();
            self.cells.resize(
                (self.bounds * self.bounds * self.bounds) as usize,
                Cell {
                    value: 0,
                    neighbors: 0,
                },
            );
        }
        // TODO add noise at start + egui
        update_neighbors(&mut self.cells, self.bounds);

        for (index, cell) in self.cells.iter_mut().enumerate() {
            // TODO if cell dead check alive

            let pos = index_to_pos(&index, &self.bounds);
            let mut neighbors = 0;
            for neighbor in rule_handler.get_neighbors_iter() {
                let neighbor_pos = pos + *neighbor;
                if out_of_bounds(&pos, &self.bounds) {
                    continue;
                };
                let neighbor_index = (neighbor_pos.x
                    + neighbor_pos.y * self.bounds
                    + neighbor_pos.z * self.bounds * self.bounds)
                    as usize;
                if self.cells[neighbor_index].value == 1 {
                    neighbors += 1
                }
            }
        }

        // for cell in &mut self.cells {
        //     cell.value = 0;
        // }
        // for _ in 0..100 {
        //     let num = rand::thread_rng().gen_range(0..(64 * 64 * 64));
        //     self.cells[num].value = 1;
        // }
        // print!("update logic {}", self.cells.len());
    }

    pub fn render(&self, renderer: &mut CellRenderer) {
        renderer.clear();
        for (index, cell) in self.cells.iter().enumerate() {
            renderer.set(index, cell.value, cell.neighbors);
        }
    }
}
