use bevy::log::info_span;
use bevy::prelude::*;
use bracket_noise::prelude::{FastNoise, FractalType, NoiseType};
use itertools::iproduct;
use std::ops::RangeInclusive;

use crate::game::world::{
    block::components::{Block, BlockFamily, Blocks},
    generation::{
        constants::{CHUNK_S1, CHUNK_S1I, MAX_GEN_HEIGHT},
        noise::resources::NoisesList,
        pos::ColPos,
    },
    voxel::resources::VoxelWorld,
};

fn pos_to_range(pos: ColPos) -> [RangeInclusive<i32>; 2] {
    let x = pos.z * CHUNK_S1I;
    let y = pos.x * CHUNK_S1I;
    [x..=(x + CHUNK_S1I - 1), y..=(y + CHUNK_S1I - 1)]
}

pub fn gen_terrain(world: &VoxelWorld, col: ColPos, fast_noise: &NoisesList) {
    let gen_span = info_span!("noise gen", name = "noise gen").entered();

    let mut temperature_noise = FastNoise::new();
    temperature_noise.set_frequency(0.0001);

    let mut humidity_noise = FastNoise::new();
    humidity_noise.set_frequency(0.001);

    let mut weird_noise = FastNoise::new();
    weird_noise.set_frequency(0.004);

    let mut erosion_noise = FastNoise::new();
    erosion_noise.set_frequency(0.002);

    let mut continental_noise = FastNoise::new();
    continental_noise.set_frequency(0.001);

    let mut pv_noise = FastNoise::new();
    pv_noise.set_frequency(0.0002);

    let ranges = pos_to_range(col);
    let offsets = ranges
        .clone()
        .map(|range| (range.start() / 1 as i32) as f32 / 1. as f32);

    gen_span.exit();
    let fill_span = info_span!("chunk filling", name = "chunk filling").entered();
    for (dx, dz) in iproduct!(0..CHUNK_S1, 0..CHUNK_S1) {
        let height = 1.;
        let offset_x = offsets[1];
        let offset_z = offsets[0];
        let biome = get_biome(
            &continental_noise,
            &erosion_noise,
            &temperature_noise,
            &humidity_noise,
            &pv_noise,
            offset_x,
            offset_z,
            dx as f32,
            dz as f32,
        );

        let temperature_height =
            ((temperature_noise.get_noise(offset_x + dx as f32, offset_z + dz as f32) + 1.) * 0.5);
        let humidity_height =
            ((humidity_noise.get_noise(offset_x + dx as f32, offset_z + dz as f32) + 1.) * 0.5);
        let weird_height =
            ((weird_noise.get_noise(offset_x + dx as f32, offset_z + dz as f32) + 1.) * 0.5);
        let erosion_height =
            ((erosion_noise.get_noise(offset_x + dx as f32, offset_z + dz as f32) + 1.) * 0.5);
        let continental_height =
            ((continental_noise.get_noise(offset_x + dx as f32, offset_z + dz as f32) + 1.) * 0.5);
        let pv_height =
            ((pv_noise.get_noise(offset_x + dx as f32, offset_z + dz as f32) + 1.) * 0.5);

        let height = (temperature_height
            + humidity_height
            + weird_height * weird_height
            + erosion_height
            + continental_height * continental_height
            + pv_height * pv_height)
            / 6.;

        
        let block = match biome {
                Biome::MushroomFields => Blocks::mushroom(),
                Biome::DeepFrozenOcean => Blocks::ice(),
                Biome::DeepOcean => Blocks::dirt(),
                Biome::FrozenOcean => Blocks::ice(),
                Biome::ColdOcean => Blocks::ice(),
                Biome::Ocean => Blocks::dirt(),
                Biome::LukewarmOcean => Blocks::dirt(),
                Biome::WarmOcean => Blocks::dirt(),
                Biome::DeepDark => Blocks::dirt(),
                Biome::LushCaves => Blocks::dirt(),
                Biome::DripstoneCaves => Blocks::dirt(),
                Biome::FrozenRiver => Blocks::ice(),
                Biome::River => Blocks::dirt(),
                Biome::StonyShore => Blocks::sand(),
                Biome::Badlands => Blocks::dirt(),
                Biome::Beach => Blocks::sand(),
                Biome::SnowyBeach => Blocks::sand(),
                Biome::Desert => Blocks::sand(),
                Biome::WoodedBadlands => Blocks::dirt(),
                Biome::SnowySlopes => Blocks::sand(),
                Biome::Plateau => Blocks::dirt(),
                Biome::JaggedPeaks => Blocks::dirt(),
                Biome::StonyPeaks => Blocks::dirt(),
                Biome::Plains => Blocks::dirt(),
        };
        
        
        // Placer les blocs dans le monde
        let y = (height * MAX_GEN_HEIGHT as f32) as i32;
        world.set_yrange(col, (dx, dz), y, 4, block);
        world.set_yrange(col, (dx, dz), y - 4, 2, Blocks::stone());
        world.set_yrange(col, (dx, dz), y - 6, MAX_GEN_HEIGHT, Blocks::deepslate());
    }

    fill_span.exit();
}

fn temperature_level(tn: f32) -> usize {
    if tn >= -1.0 && tn < -0.45 {
        0
    } else if tn >= -0.45 && tn < -0.15 {
        1
    } else if tn >= -0.15 && tn < 0.2 {
        2
    } else if tn >= 0.2 && tn < 0.55 {
        3
    } else {
        4
    }
}

fn humidity_level(hn: f32) -> usize {
    if hn >= -1.0 && hn < -0.35 {
        0
    } else if hn >= -0.35 && hn < -0.1 {
        1
    } else if hn >= -0.1 && hn < 0.1 {
        2
    } else if hn >= 0.1 && hn < 0.3 {
        3
    } else {
        4
    }
}

fn continental_level(cn: f32) -> Continental {
    if cn >= -1.2 && cn < -1.05 {
        Continental::ChampsDeChampignons
    } else if cn >= -1.05 && cn < -0.455 {
        Continental::OceanProfond
    } else if cn >= -0.455 && cn < -0.19 {
        Continental::Ocean
    } else if cn >= -0.19 && cn < -0.11 {
        Continental::Littoral
    } else if cn >= -0.11 && cn < 0.03 {
        Continental::PeuAInterieurDesTerres
    } else if cn >= 0.03 && cn < 0.3 {
        Continental::AInterieurDesTerres
    } else {
        Continental::LoinAInterieurDesTerres
    }
}

fn erosion_level(en: f32) -> usize {
    if en >= -1.0 && en < -0.78 {
        0
    } else if en >= -0.78 && en < -0.375 {
        1
    } else if en >= -0.375 && en < -0.2225 {
        2
    } else if en >= -0.2225 && en < 0.05 {
        3
    } else if en >= 0.05 && en < 0.45 {
        4
    } else if en >= 0.45 && en < 0.55 {
        5
    } else {
        6
    }
}

fn relief_type(pv: f32) -> Relief {
    if pv >= -1.0 && pv < -0.85 {
        Relief::Valley
    } else if pv >= -0.85 && pv < -0.6 {
        Relief::Low
    } else if pv >= -0.6 && pv < 0.2 {
        Relief::Medium
    } else if pv >= 0.2 && pv < 0.7 {
        Relief::Escarped
    } else {
        Relief::Mountain
    }
}

enum Relief {
    Valley,
    Low,
    Medium,
    Escarped,
    Mountain,
}

enum Continental {
    ChampsDeChampignons,
    OceanProfond,
    Ocean,
    Littoral,
    PeuAInterieurDesTerres,
    AInterieurDesTerres,
    LoinAInterieurDesTerres,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Biome {
    MushroomFields,
    DeepFrozenOcean,
    DeepOcean,
    FrozenOcean,
    ColdOcean,
    Ocean,
    LukewarmOcean,
    WarmOcean,
    DeepDark,
    LushCaves,
    DripstoneCaves,
    FrozenRiver,
    River,
    StonyShore,
    Badlands,
    Beach,
    SnowyBeach,
    Desert,
    WoodedBadlands,
    SnowySlopes,
    Plateau,
    JaggedPeaks,
    StonyPeaks,
    Plains,
}

fn get_biome(
    continental_noise: &FastNoise,
    erosion_noise: &FastNoise,
    temperature_noise: &FastNoise,
    humidity_noise: &FastNoise,
    pv_noise: &FastNoise,
    offset_x: f32,
    offset_z: f32,
    x: f32,
    z: f32,
) -> Biome {
    let tn = temperature_noise.get_noise(offset_x + x, offset_z + z);
    let hn = humidity_noise.get_noise(offset_x + x, offset_z + z);
    let cn = continental_noise.get_noise(offset_x + x, offset_z + z);
    let en = erosion_noise.get_noise(offset_x + x, offset_z + z);
    let pv = pv_noise.get_noise(offset_x + x, offset_z + z);

    let temperature = temperature_level(tn);
    let humidity = humidity_level(hn);
    let continental = continental_level(cn);
    let erosion = erosion_level(en);
    let relief = relief_type(pv);

    let depth = match relief {
        Relief::Valley => 0.0,
        Relief::Low => 0.2,
        Relief::Medium => 0.5,
        Relief::Escarped => 0.8,
        Relief::Mountain => 1.0,
    };

    match continental {
        Continental::ChampsDeChampignons => Biome::MushroomFields,
        Continental::OceanProfond => match temperature {
            0 => Biome::DeepFrozenOcean,
            1 => Biome::DeepFrozenOcean,
            2 => Biome::DeepOcean,
            3 => Biome::DeepOcean,
            _ => Biome::DeepOcean,
        },
        Continental::Ocean => match temperature {
            0 => Biome::FrozenOcean,
            1 => Biome::ColdOcean,
            2 => Biome::Ocean,
            3 => Biome::LukewarmOcean,
            _ => Biome::WarmOcean,
        },
        Continental::Littoral => {
            if depth == 1.1 && erosion < 4 {
                return Biome::DeepDark;
            }
            if depth >= 0.2 && depth <= 0.9 {
                if matches!(continental, Continental::LoinAInterieurDesTerres) && humidity >= 7 {
                    return Biome::LushCaves;
                }
                return Biome::DripstoneCaves;
            }
            match relief {
                Relief::Valley => {
                    if temperature == 0 {
                        Biome::FrozenRiver
                    } else {
                        Biome::River
                    }
                }
                Relief::Low => match erosion {
                    0..=1 => match temperature {
                        0..=3 => Biome::StonyShore,
                        _ => Biome::Badlands,
                    },
                    2 => Biome::Beach,
                    3 => Biome::Beach,
                    4 => match temperature {
                        0 => Biome::SnowyBeach,
                        1..=3 => Biome::Beach,
                        _ => Biome::Desert,
                    },
                    5 => match humidity {
                        0..=1 => Biome::Badlands,
                        2 => Biome::Badlands,
                        3..=4 => Biome::WoodedBadlands,
                        _ => Biome::Beach,
                    },
                    _ => Biome::Beach,
                },
                Relief::Medium => match erosion {
                    0 => match temperature {
                        0..=2 => Biome::SnowySlopes,
                        _ => Biome::Plateau,
                    },
                    1 => match temperature {
                        0 => Biome::SnowySlopes,
                        _ => Biome::Plateau,
                    },
                    2 => Biome::Plateau,
                    3 => Biome::Plateau,
                    4 => match temperature {
                        0 => Biome::SnowyBeach,
                        1..=3 => Biome::Beach,
                        _ => Biome::Desert,
                    },
                    5 => match humidity {
                        0..=1 => Biome::Badlands,
                        2 => Biome::Badlands,
                        3..=4 => Biome::WoodedBadlands,
                        _ => Biome::Beach,
                    },
                    _ => Biome::Plateau,
                },
                Relief::Escarped => match erosion {
                    0 => match temperature {
                        0..=2 => Biome::SnowySlopes,
                        _ => Biome::JaggedPeaks,
                    },
                    1 => match temperature {
                        0 => Biome::SnowySlopes,
                        _ => Biome::Plateau,
                    },
                    2 => Biome::Plateau,
                    3 => Biome::Plateau,
                    4 => match temperature {
                        0 => Biome::SnowyBeach,
                        1..=3 => Biome::Beach,
                        _ => Biome::Desert,
                    },
                    5 => match humidity {
                        0..=1 => Biome::Badlands,
                        2 => Biome::Badlands,
                        3..=4 => Biome::WoodedBadlands,
                        _ => Biome::Beach,
                    },
                    _ => Biome::Plateau,
                },
                Relief::Mountain => match erosion {
                    0 => match temperature {
                        0..=2 => Biome::JaggedPeaks,
                        _ => Biome::StonyPeaks,
                    },
                    1 => match temperature {
                        0 => Biome::SnowySlopes,
                        _ => Biome::Plateau,
                    },
                    2 => Biome::Plateau,
                    3 => Biome::Plateau,
                    4 => match temperature {
                        0 => Biome::SnowyBeach,
                        1..=3 => Biome::Beach,
                        _ => Biome::Desert,
                    },
                    5 => match humidity {
                        0..=1 => Biome::Badlands,
                        2 => Biome::Badlands,
                        3..=4 => Biome::WoodedBadlands,
                        _ => Biome::Beach,
                    },
                    _ => Biome::Plains,
                },
            }
        }
        _ => Biome::Plains, // Default to Plains if no other condition matches
    }
}
