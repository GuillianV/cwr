use bevy::{prelude::*, utils::HashMap};

use crate::game::world::generation::grid::components::Coords;

#[derive(Component)]
pub struct Chunks {
    chunk_list: HashMap<String, Chunk>,
    chunk_size: Vec2,
}

impl Default for Chunks {
    fn default() -> Self {
        Self {
            chunk_list: HashMap::<String, Chunk>::new(),
            chunk_size: Vec2::new(16.0, 16.0),
        }
    }
}

impl Chunks {
    pub fn update_grid(&mut self, coords: &Coords) -> Vec<Chunk> {
        let x = if coords.x < 0 {
            coords.x - self.chunk_size.x as i32
        } else {
            coords.x + self.chunk_size.x as i32
        };
        let rest_x = x % self.chunk_size.x as i32;
        let modulo_x = (x - rest_x) / self.chunk_size.x as i32;

        let z = if coords.z < 0 {
            coords.z - self.chunk_size.y as i32
        } else {
            coords.z + self.chunk_size.y as i32
        };
        let rest_z = z % self.chunk_size.y as i32;
        let modulo_z = (z - rest_z) / self.chunk_size.y as i32;

        let chunk_id = format!("{0},{1}", modulo_x, modulo_z);
        if !self.chunk_list.contains_key(&chunk_id) {
            self.chunk_list.insert(
                chunk_id.clone(),
                Chunk {
                    cells: Vec::<Coords>::new(),
                    fullfilled: false,
                },
            );
        }

        let chunk = self.chunk_list.get_mut(&chunk_id).unwrap();
        chunk.cells.push(coords.clone());
        println!("Chunk update {0},{1}, coords {2},{3} , chunck size {4}", modulo_x, modulo_z, coords.x, coords.z, chunk.cells.len());
        let mut fullfilled_chunks = Vec::<Chunk>::new();
        if chunk.cells.len() == self.chunk_size.x as usize * self.chunk_size.y as usize {
            println!("Chunk fullfilled ID {0}, len {1}", chunk_id, chunk.cells.len());
            chunk.fullfilled = true;
            fullfilled_chunks.push(chunk.clone());
        }
        // println!("Chunk fullfilled ID {0}, len {1}", chunk_id, chunk.cells.len());
        fullfilled_chunks
    }
}

#[derive(Component, Clone)]
pub struct Chunk {
    pub cells: Vec<Coords>,
    fullfilled: bool,
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            cells: Vec::<Coords>::new(),
            fullfilled: false,
        }
    }
}
