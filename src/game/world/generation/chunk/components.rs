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
        let mut chunk_size_x = self.chunk_size.x as i32;
        let mut chunk_size_z = self.chunk_size.y as i32;

        let x = if coords.x < 0 {
            coords.x - chunk_size_x
        } else {
            coords.x + chunk_size_x
        };
        let rest_x = x % chunk_size_x;
        let modulo_x = (x - rest_x) / chunk_size_x;

        let z = if coords.z < 0 {
            coords.z - chunk_size_z
        } else {
            coords.z + chunk_size_z
        };
        let rest_z = z % chunk_size_z;
        let modulo_z = (z - rest_z) / chunk_size_z;

        let chunk_id = format!("{0},{1}", modulo_x, modulo_z).to_string();

        if !self.chunk_list.contains_key(&chunk_id) {
            self.chunk_list.insert(
                chunk_id.clone(),
                Chunk {
                    cells: Vec::<Coords>::new(),
                    fullfilled: false,
                    id: chunk_id.clone(),
                },
            );
        }

        let chunk = self.chunk_list.get_mut(&chunk_id).unwrap();
        chunk.cells.push(coords.clone());

        if modulo_x == -1 {
            chunk_size_x = chunk_size_x - 1;
        }
        if modulo_z == -1 {
            chunk_size_z = chunk_size_z - 1;
        }

        let mut fullfilled_chunks = Vec::<Chunk>::new();
        if chunk.cells.len() == chunk_size_x as usize * chunk_size_z as usize {
            chunk.fullfilled = true;
            fullfilled_chunks.push(chunk.clone());
        }
        fullfilled_chunks
    }
}

#[derive(Component, Clone)]
pub struct Chunk {
    pub cells: Vec<Coords>,
    pub id: String,
    fullfilled: bool,
}

impl Default for Chunk {
    fn default() -> Self {
        Self {
            id: String::new(),
            cells: Vec::<Coords>::new(),
            fullfilled: false,
        }
    }
}
