use crate::render::CellRenderer;
use rand::Rng;

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
    pub fn update(&mut self) {
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
        for cell in &mut self.cells {
            cell.value = 0;
        }
        for _ in 0..100 {
            let num = rand::thread_rng().gen_range(0..(64 * 64 * 64));
            self.cells[num].value = 1;
        }
        // print!("update logic {}", self.cells.len());
    }

    pub fn render(&self, renderer: &mut CellRenderer) {
        renderer.clear();
        for (index, cell) in self.cells.iter().enumerate() {
            renderer.set(index, cell.value, cell.neighbors);
        }
    }
}
