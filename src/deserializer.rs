use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
pub enum GameMode {
    Survival,
    Creative,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum WorldVersion {
    V0_0_7_,
    V0_1_0,
    V0_1_3,
    UNKNOWN,
}

impl WorldVersion {
    pub fn get_matching_name(&self) -> String {
        match *self {
            WorldVersion::V0_0_7_ => "v0.0.7/8/9".to_string(),
            WorldVersion::V0_1_0 => "v0.1.0".to_string(),
            WorldVersion::V0_1_3 => "v0.1.3".to_string(),
            WorldVersion::UNKNOWN => "unknown".to_string(),
        }
    }
}

// Generic
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorldInfo {
    pub world_version: WorldVersion,
    pub world_name: String,
    pub gamemode: GameMode,
}

// v0.1.3
#[derive(Serialize, Deserialize, Debug)]
pub struct WorldInfo3 {
    pub world_version: u16,
    pub world_name: String,
    pub world_seed: i32,
    pub gamemode: GameMode,
}

// v0.1.0
#[derive(Serialize, Deserialize, Debug)]
pub struct WorldInfo2 {
    pub world_name: String,
    pub world_seed: i32,
    pub gamemode: GameMode,
}

// v0.0.7 - v0.0.9
#[derive(Serialize, Deserialize, Debug)]
pub struct WorldInfo1 {
    pub world_name: String,
    pub world_seed: i32,
}

fn get_version_from_version_number(version_number: u16) -> WorldVersion {
    match version_number {
        0 => WorldVersion::V0_1_3,
        _ => WorldVersion::UNKNOWN,
    }
}

pub fn get_world_info(raw: &Vec<u8>) -> Option<WorldInfo> {
    if raw.len() < 2 {
        return None;
    }

    let world_info_size = u16::from_be_bytes([raw[0], raw[1]]);

    if raw.len() < (2 + world_info_size) as usize {
        return None;
    }

    // Try different versions
    if let Ok(world_info) =
        postcard::from_bytes::<WorldInfo3>(raw.get(2..(2 + world_info_size as usize))?)
    {
        Some(WorldInfo {
            world_version: get_version_from_version_number(world_info.world_version),
            world_name: world_info.world_name,
            gamemode: world_info.gamemode,
        })
    } else if let Ok(world_info) =
        postcard::from_bytes::<WorldInfo2>(raw.get(2..(2 + world_info_size as usize))?)
    {
        Some(WorldInfo {
            world_version: WorldVersion::V0_1_0,
            world_name: world_info.world_name,
            gamemode: world_info.gamemode,
        })
    } else if let Ok(world_info) =
        postcard::from_bytes::<WorldInfo1>(raw.get(2..(2 + world_info_size as usize))?)
    {
        Some(WorldInfo {
            world_version: WorldVersion::V0_0_7_,
            world_name: world_info.world_name,
            gamemode: GameMode::Creative,
        })
    } else {
        None
    }
}
