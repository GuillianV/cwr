use bevy::prelude::*;
use itertools::Itertools;
use parking_lot::lock_api::ArcRwLockWriteGuard;
use parking_lot::{RawRwLock, RwLock};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::game::entity::player::area::resources::PlayerArea;
use crate::game::world::generation::pos::pos2d::Pos2d;
use crate::game::world::generation::{
    constants::{CHUNK_S1},
    pos::ColPos,
};

#[derive(Resource)]
pub struct LoadOrders {
    // { column: { player } }
    player_cols: HashMap<ColPos, HashSet<u32>>,
    // [(column, min dist to player)]
    pub to_generate: Arc<RwLock<Vec<(ColPos, u32)>>>,
    pub to_unload: Vec<ColPos>,
}

impl LoadOrders {
    pub fn new() -> Self {
        LoadOrders {
            player_cols: HashMap::new(),
            to_generate: Arc::new(RwLock::new(Vec::new())),
            to_unload: Vec::new(),
        }
    }

    fn unload_col(&mut self, col_pos: ColPos) {
        self.player_cols.remove(&col_pos);
        // NOTE: very important to store this in an intermediary variable
        // or else the read lock lives long enough that we reach the write lock in the if
        let generate_order_opt = self
            .to_generate
            .read_arc()
            .iter()
            .find_position(|(pos_, _)| *pos_ == col_pos)
            .map(|(i, _)| i);
        if let Some(i) = generate_order_opt {
            // the column was still waiting for load
            self.to_generate.write_arc().remove(i);
        } else {
            self.to_unload.push(col_pos);
        }
    }

    pub fn on_load_area_change(
        &mut self,
        player_id: u32,
        old_load_area: &PlayerArea,
        new_load_area: &PlayerArea,
    ) {
        for col_pos in old_load_area.col_dists.keys() {
            if new_load_area.col_dists.contains_key(col_pos) {
                continue;
            }
            if let Some(players) = self.player_cols.get_mut(col_pos) {
                players.remove(&player_id);
                if players.is_empty() {
                    self.unload_col(*col_pos);
                }
            }
        }
        let mut wlock: ArcRwLockWriteGuard<RawRwLock, Vec<(Pos2d<CHUNK_S1>, u32)>> =
            self.to_generate.write_arc();
        for (col_pos, dist) in new_load_area.col_dists.iter() {
            if old_load_area.col_dists.contains_key(col_pos) {
                continue;
            }
            let players = self.player_cols.entry(*col_pos).or_default();
            let is_new = players.is_empty();
            players.insert(player_id);
            if is_new {
                add_gen_order(&mut wlock, *col_pos, *dist);
            } else {
                update_gen_order(&mut wlock, col_pos, *dist)
            }
        }
    }
}

fn add_gen_order(
    to_generate: &mut ArcRwLockWriteGuard<RawRwLock, Vec<(Pos2d<CHUNK_S1>, u32)>>,
    col_pos: ColPos,
    dist: u32,
) {
    // col_pos should *not* be present in to_generate
    // need to take a write lock before doing read and write or else to_generate could change between the read and the write
    let i = match to_generate.binary_search_by(|(_, other_dist)| dist.cmp(other_dist)) {
        Ok(i) => i,
        Err(i) => i,
    };
    to_generate.insert(i, (col_pos, dist));
}

fn update_gen_order(
    to_generate: &mut ArcRwLockWriteGuard<RawRwLock, Vec<(Pos2d<CHUNK_S1>, u32)>>,
    col_pos: &ColPos,
    dist: u32,
) {
    // col_pos may be present in to_generate
    let Some(old_i) = to_generate
        .iter()
        .position(|(other_col, _)| other_col == col_pos)
    else {
        return;
    };
    let new_i = match to_generate.binary_search_by(|(_, other_dist)| dist.cmp(other_dist)) {
        Ok(i) => i,
        Err(i) => i,
    };
    //Potentiellement foireux
    if old_i != new_i {
        let element = to_generate.remove(old_i);
        to_generate.insert(new_i, element);
    }
}
