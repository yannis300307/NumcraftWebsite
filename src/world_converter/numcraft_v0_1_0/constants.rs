use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

pub mod world {
    pub const CHUNK_SIZE: usize = 8; // MAX 8

    pub const MAX_ITEM_MERGING_DISTANCE: f32 = 2.;
    pub const ITEM_MAGNET_FORCE: f32 = 10.;
    pub const MAX_PLAYER_ITEM_MAGNET_DISTANCE: f32 = 2.2;
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    Player = 0,
    Item = 1,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    Air = 0,
    Stone = 1,
    Grass = 2,
    Dirt = 3,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum ItemType {
    Air = 0,

    StoneBlock = 1,
    GrassBlock = 2,
    DirtBlock = 3,
}

impl ItemType {
    pub fn get_max_stack_amount(&self) -> u8 {
        match *self {
            ItemType::Air => 0,
            ItemType::StoneBlock => 64,
            ItemType::GrassBlock => 64,
            ItemType::DirtBlock => 64,
        }
    }

    pub fn get_matching_block_type(&self) -> Option<BlockType> {
        match self {
            ItemType::Air => None,
            ItemType::StoneBlock => Some(BlockType::Stone),
            ItemType::GrassBlock => Some(BlockType::Grass),
            ItemType::DirtBlock => Some(BlockType::Dirt),
        }
    }
}

impl BlockType {
    pub fn is_air(&self) -> bool {
        *self == BlockType::Air
    }

    pub const fn get_from_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(BlockType::Air),
            1 => Some(BlockType::Stone),
            2 => Some(BlockType::Grass),
            3 => Some(BlockType::Dirt),
            _ => None,
        }
    }

    pub const fn get_hardness(&self) -> f32 {
        match self {
            BlockType::Air => 0.,
            BlockType::Stone => 2.,
            BlockType::Grass => 1.2,
            BlockType::Dirt => 1.,
        }
    }

    pub const fn get_dropped_item_type(&self) -> ItemType {
        match self {
            BlockType::Air => ItemType::Air,
            BlockType::Stone => ItemType::StoneBlock,
            BlockType::Grass => ItemType::DirtBlock,
            BlockType::Dirt => ItemType::DirtBlock,
        }
    }
}
