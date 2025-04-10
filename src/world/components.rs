use std::f64::consts::PI;

use bevy::prelude::*;

use super::events::WorldGridNewCellEvent;

#[derive(Component, Clone)]
pub struct World {
    pub light: f32,
}

#[derive(Component, Clone)]
pub struct WorldGrid {
    grid_2d: Vec<Coords>,
    pub generate_distance: i32,
    grid_size: Vec3,
    chunk_size: Vec2,
    min_y: i32,
    max_y: i32,
    last_position: Coords,
}

impl Default for WorldGrid {
    fn default() -> Self {
        Self {
            grid_2d: Vec::<Coords>::new(),
            generate_distance: 2,
            grid_size: Vec3::new(1.0, 1.0, 1.0),
            chunk_size: Vec2::new(32.0, 32.0),
            min_y: 0,
            max_y: 256,
            last_position: Coords::new(0, 0),
        }
    }
}

impl WorldGrid {
    fn cell_exists(&self, coords: Coords) -> (bool, Coords) {
        (self.grid_2d.contains(&coords), coords)
    }

    fn init_cell(&mut self, coords: Coords) {
        let rest_x = coords.x % self.chunk_size.x as i32;
        let modulo_x = (coords.x - rest_x) / self.chunk_size.x as i32;
        let rest_z = coords.z % self.chunk_size.y as i32;
        let modulo_z = (coords.z - rest_z) / self.chunk_size.y as i32;

        println!("new cell at {}, {}", coords.x, coords.z);
        println!("chunck x: {}, chunck z: {}", modulo_x, modulo_z);

        self.grid_2d.push(coords);
    }

    fn check_init_cell(&mut self, coords: Coords) -> bool {
        let (exist, coords) = self.cell_exists(coords);

        if !exist {
            self.init_cell(coords);
            return true;
        }

        false
    }

    pub fn initialize(&mut self, entity_position: Vec3) -> Vec<Coords> {
        let coords = Coords::new(entity_position.x as i32, entity_position.z as i32);
        let mut new_cells = Vec::<Coords>::new();

        let minus_x = coords.x - self.generate_distance;
        let minus_z = coords.z - self.generate_distance;
        let plus_x = coords.x + self.generate_distance;
        let plus_z = coords.z + self.generate_distance;

        for x in minus_x..plus_x {
            for z in minus_z..plus_z {
                self.init_cell(Coords::new(x, z));
                new_cells.push(Coords::new(x, z));
            }
        }

        self.last_position = coords;
        new_cells
    }

    pub fn update(&mut self, entity_position: Vec3) -> Vec<Coords> {
        let coords = Coords::new(entity_position.x as i32, entity_position.z as i32);
        let mut new_cells = Vec::<Coords>::new();

        if coords != self.last_position {
            let difference_x = coords.x - self.last_position.x;
            let difference_z = coords.z - self.last_position.z;

            if difference_x != 0 || difference_z != 0 {
                let minus_x = coords.x - self.generate_distance;
                let plus_x = coords.x + self.generate_distance;

                let minus_z = coords.z - self.generate_distance;
                let plus_z = coords.z + self.generate_distance;

                let old_minus_x = self.last_position.x - self.generate_distance;
                let old_plus_x = self.last_position.x + self.generate_distance;

                let old_minus_z = self.last_position.z - self.generate_distance;
                let old_plus_z = self.last_position.z + self.generate_distance;

                for x in minus_x..=plus_x {
                    for z in minus_z..=plus_z {
                        // Vérifier si la cellule est nouvelle
                        if x <= old_minus_x || x >= old_plus_x || z <= old_minus_z || z >= old_plus_z {
                            if self.check_init_cell(Coords::new(x, z)) {
                                new_cells.push(Coords::new(x, z));
                            }
                        }
                    }
                }
            }

            self.last_position = coords;
        }
        new_cells
    }

    // fn get_chunk(&self, position: Vec3) -> Vec3 {
    //     let chunk_x = position.x / self.chunk_size.x;
    //     let chunk_y = position.y / self.chunk_size.y;
    //     let chunk_z = position.z / self.chunk_size.x;

    //     Vec3::new(chunk_x, chunk_y, chunk_z)
    // }
}

#[derive(PartialEq, Clone)]
pub struct Coords {
    pub x: i32,
    pub z: i32,
}

impl Coords {
    // Constructeur pour créer un nouveau Vec3
    fn new(x: i32, z: i32) -> Self {
        Coords { x, z }
    }
}

#[derive(Component, Clone)]
pub struct GridCell {
    pub position: Vec3,
}

#[derive(Component, Clone)]
pub struct Voxel {
    pub position: Vec3,
    pub size: Vec3,
    pub color: Color,
}
