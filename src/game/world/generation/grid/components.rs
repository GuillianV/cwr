use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component, Clone)]
pub struct Grid {
    grid_2d: HashMap<String, Coords>,
    pub watch_distance: i32,
    last_position: Coords,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            grid_2d: HashMap::<String, Coords>::new(),
            watch_distance: 16,
            last_position: Coords::new(0, 0),
        }
    }
}

impl Grid {
    fn init_cell(&mut self, coords: Coords) -> bool {
        let id = format!("{0},{1}", coords.x, coords.z);
        if !self.grid_2d.contains_key(&id) {
            self.grid_2d
                .insert(format!("{0},{1}", coords.x, coords.z), coords);
            return true;
        }
        false
    }

    pub fn initialize(&mut self, entity_position: Vec3) -> Vec<Coords> {
        let coords = Coords::new(entity_position.x as i32, entity_position.z as i32);
        let mut new_cells = Vec::<Coords>::new();

        let minus_x = coords.x - self.watch_distance;
        let minus_z = coords.z - self.watch_distance;
        let plus_x = coords.x + self.watch_distance;
        let plus_z = coords.z + self.watch_distance;

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
                let minus_x = coords.x - self.watch_distance;
                let plus_x = coords.x + self.watch_distance;

                let minus_z = coords.z - self.watch_distance;
                let plus_z = coords.z + self.watch_distance;

                let old_minus_x = self.last_position.x - self.watch_distance;
                let old_plus_x = self.last_position.x + self.watch_distance;

                let old_minus_z = self.last_position.z - self.watch_distance;
                let old_plus_z = self.last_position.z + self.watch_distance;

                for x in minus_x..=plus_x {
                    for z in minus_z..=plus_z {
                        // Vérifier si la cellule est nouvelle
                        if x <= old_minus_x
                            || x >= old_plus_x
                            || z <= old_minus_z
                            || z >= old_plus_z
                        {
                            if self.init_cell(Coords::new(x, z)) {
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
}

#[derive(PartialEq, Clone)]
pub struct Coords {
    pub x: i32,
    pub z: i32,
}

impl Coords {
    // Constructeur pour créer un nouveau Vec3
    pub fn new(x: i32, z: i32) -> Self {
        Coords { x, z }
    }
}
