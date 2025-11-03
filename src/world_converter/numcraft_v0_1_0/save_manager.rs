use lz4_flex::{decompress, decompress_size_prepended};
use nalgebra::Vector3;
use postcard::from_bytes;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum GameMode {
    Survival,
    Creative,
}

use crate::{
    world_converter::numcraft_v0_1_0::chunk::Chunk,
    world_converter::numcraft_v0_1_0::constants::{world::CHUNK_SIZE, BlockType},
    world_converter::numcraft_v0_1_0::inventory::Inventory,
};

#[derive(Serialize, Deserialize)]
pub struct PlayerData {
    pub pos: (f32, f32, f32),
    pub rotation: (f32, f32), // Only Pitch and Yaw
    pub inventory: Inventory, // More in the futur
}

#[derive(Serialize, Deserialize)]
pub struct WorldInfo {
    pub world_name: String,
    pub world_seed: i32,
    pub gamemode: GameMode,
}

impl WorldInfo {
    pub fn new() -> Self {
        WorldInfo {
            world_name: String::new(),
            world_seed: 1,
            gamemode: GameMode::Survival,
        }
    }
}

impl PlayerData {
    pub fn new() -> Self {
        PlayerData {
            pos: (0., 0., 0.),
            rotation: (0., 0.),
            inventory: Inventory::new(0),
        }
    }
}

pub struct SaveManager {
    chunks_data: [Vec<u8>; 64],
    pub player_data: PlayerData,
    pub world_info: WorldInfo,
}

impl SaveManager {
    pub fn new() -> Self {
        SaveManager {
            chunks_data: [const { Vec::new() }; 64],
            player_data: PlayerData::new(),
            world_info: WorldInfo::new(),
        }
    }

    pub fn get_game_mode(&self) -> GameMode {
        self.world_info.gamemode
    }


    fn read_world_info(&mut self, data: &[u8]) -> Result<usize, SaveFileLoadError> {
        let mut world_data_offset = 0;
        // If world info is missing, the world is currupted
        if world_data_offset + 1 >= data.len() {
            return Err(SaveFileLoadError::CorruptedWorld);
        }

        // Extract world info
        let world_info_size =
            u16::from_be_bytes([data[world_data_offset], data[world_data_offset + 1]]) as usize;

        world_data_offset += 2; // world info size

        // Check for overflow
        if world_data_offset + world_info_size > data.len() {
            return Err(SaveFileLoadError::CorruptedWorld);
        }

        // Read the raw data
        let world_info_raw = &data[world_data_offset..(world_data_offset + world_info_size)];

        if let Ok(world_info) = from_bytes::<WorldInfo>(world_info_raw) {
            self.world_info = world_info;
            Ok(world_data_offset + world_info_size)
        } else {
            return Err(SaveFileLoadError::CorruptedWorld);
        }
    }

    pub fn load_from_file(&mut self, raw_data: &Vec<u8>) -> Result<(), SaveFileLoadError> {
        if let Ok(world_data_offset) = self.read_world_info(&raw_data) {
            // Decompress the entire file
            if let Ok(data) = decompress_size_prepended(&raw_data[world_data_offset..]) {
                let mut current_pos = 128;
                for i in 0..64 {
                    let size = u16::from_be_bytes([data[i * 2], data[i * 2 + 1]]) as usize; // Get the compressed chunk size from the headers

                    if current_pos + size > data.len() {
                        // Check for corruption. If overflow, the size is wrong and the world is ... unusable ...
                        return Err(SaveFileLoadError::CorruptedWorld);
                    }
                    let raw_chunk = &data[current_pos..(current_pos + size)];

                    self.chunks_data[i] = raw_chunk.to_vec();

                    current_pos += size;
                }

                // If player data is missing, the world is currupted
                if current_pos + 1 >= data.len() {
                    return Err(SaveFileLoadError::CorruptedWorld);
                }

                // Extract player_data
                let player_data_size =
                    u16::from_be_bytes([data[current_pos], data[current_pos + 1]]) as usize;

                current_pos += 2; // player data size

                // Check for overflow
                if current_pos + player_data_size > data.len() {
                    return Err(SaveFileLoadError::CorruptedWorld);
                }

                // Read the raw data
                let player_data_raw = &data[current_pos..(current_pos + player_data_size)];

                if let Ok(player_data) = from_bytes::<PlayerData>(player_data_raw) {
                    self.player_data = player_data;
                } else {
                    return Err(SaveFileLoadError::CorruptedWorld);
                }

                Ok(())
            } else {
                Err(SaveFileLoadError::CorruptedWorld)
            }
        } else {
            Err(SaveFileLoadError::CorruptedWorld)
        }
    }

    pub fn get_chunk_at_pos(&self, pos: Vector3<isize>) -> Result<Chunk, ChunkReadingError> {
        if pos.x < 0 || pos.x >= 4 || pos.y < 0 || pos.y >= 4 || pos.z < 0 || pos.z >= 4 {
            return Err(ChunkReadingError::OOBChunk);
        }

        let index = (pos.x + pos.y * 4 + pos.z * 16) as usize;

        let raw_chunk = &self.chunks_data[index];

        if let Ok(chunk_data) = decompress(raw_chunk, 512) {
            if chunk_data.len() != 512 {
                return Err(ChunkReadingError::CorruptedChunk);
            }

            let mut chunk = Chunk::new(pos);

            for x in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    for z in 0..CHUNK_SIZE {
                        if let Some(block_type) = BlockType::get_from_id(
                            chunk_data[x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE],
                        ) {
                            chunk.set_at(Vector3::new(x, y, z), block_type);
                        } else {
                            return Err(ChunkReadingError::CorruptedChunk);
                        }
                    }
                }
            }

            Ok(chunk)
        } else {
            Err(ChunkReadingError::CorruptedChunk)
        }
    }
}

#[derive(Debug)]
pub enum ChunkReadingError {
    OOBChunk,
    CorruptedChunk,
}

#[derive(Debug)]
pub enum SaveFileLoadError {
    CorruptedWorld,
}

/*
Save file format. World is 4 x 4 x 4 chunks.

Header:
    4x4x4 x 2 B array : represent the compressed size of the chunk for each chunk

    4x4x4 x variable size : chunks data.

    2 + variable : Player info

    2 + variable : World Info
*/
