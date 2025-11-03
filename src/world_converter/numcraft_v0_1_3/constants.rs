use serde::{Deserialize, Serialize};


pub mod save_manager {
    pub const SETTINGS_FILENAME: &str = "settings.ncd"; // NCD = NumCraftData

    pub const WORLD_VERSION: u16 = 0; // Update the version at each world breaking update
}

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
    Sand = 4,
    Cobblestone = 5,
    Border = 6,
    Log = 7,
    Leaves = 8,
    Planks = 9,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum ItemType {
    Air = 0,

    StoneBlock = 1,
    GrassBlock = 2,
    DirtBlock = 3,
    SandBlock = 4,
    CobblestoneBlock = 5,
    BorderBlock = 6,
    LogBlock = 7,
    LeavesBlock = 8,
    PlanksBlock = 9,
}

impl ItemType {
    pub const fn get_from_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(ItemType::Air),

            1 => Some(ItemType::StoneBlock),
            2 => Some(ItemType::GrassBlock),
            3 => Some(ItemType::DirtBlock),
            4 => Some(ItemType::SandBlock),
            5 => Some(ItemType::CobblestoneBlock),
            6 => Some(ItemType::BorderBlock),
            7 => Some(ItemType::LogBlock),
            8 => Some(ItemType::LeavesBlock),
            9 => Some(ItemType::PlanksBlock),
            _ => None,
        }
    }

    pub fn get_max_stack_amount(&self) -> u8 {
        match *self {
            ItemType::Air => 0,
            ItemType::StoneBlock => 64,
            ItemType::GrassBlock => 64,
            ItemType::DirtBlock => 64,
            ItemType::SandBlock => 64,
            ItemType::CobblestoneBlock => 64,
            ItemType::BorderBlock => 64,
            ItemType::LogBlock => 64,
            ItemType::LeavesBlock => 64,
            ItemType::PlanksBlock => 64,
        }
    }

    pub fn get_matching_block_type(&self) -> Option<BlockType> {
        match self {
            ItemType::Air => None,
            ItemType::StoneBlock => Some(BlockType::Stone),
            ItemType::GrassBlock => Some(BlockType::Grass),
            ItemType::DirtBlock => Some(BlockType::Dirt),
            ItemType::SandBlock => Some(BlockType::Sand),
            ItemType::CobblestoneBlock => Some(BlockType::Cobblestone),
            ItemType::BorderBlock => Some(BlockType::Border),
            ItemType::LogBlock => Some(BlockType::Log),
            ItemType::LeavesBlock => Some(BlockType::Leaves),
            ItemType::PlanksBlock => Some(BlockType::Planks),
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
            4 => Some(BlockType::Sand),
            5 => Some(BlockType::Cobblestone),
            6 => Some(BlockType::Border),
            7 => Some(BlockType::Log),
            8 => Some(BlockType::Leaves),
            9 => Some(BlockType::Planks),
            _ => None,
        }
    }

    pub const fn get_hardness(&self) -> f32 {
        match self {
            BlockType::Air => 0.,
            BlockType::Stone => 2.,
            BlockType::Grass => 1.2,
            BlockType::Dirt => 1.,
            BlockType::Sand => 1.,
            BlockType::Cobblestone => 2.2,
            BlockType::Border => -1.,
            BlockType::Log => 1.5,
            BlockType::Leaves => 0.3,
            BlockType::Planks => 1.2,
        }
    }

    pub const fn get_dropped_item_type(&self) -> ItemType {
        match self {
            BlockType::Air => ItemType::Air,
            BlockType::Stone => ItemType::CobblestoneBlock,
            BlockType::Grass => ItemType::DirtBlock,
            BlockType::Dirt => ItemType::DirtBlock,
            BlockType::Sand => ItemType::SandBlock,
            BlockType::Cobblestone => ItemType::CobblestoneBlock,
            BlockType::Border => ItemType::BorderBlock,
            BlockType::Log => ItemType::LogBlock,
            BlockType::Leaves => ItemType::LeavesBlock,
            BlockType::Planks => ItemType::PlanksBlock,
        }
    }
}

