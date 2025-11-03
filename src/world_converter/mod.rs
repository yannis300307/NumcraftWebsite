/* Confusing part !!! */

use nalgebra::Vector3;

pub mod numcraft_v0_1_0;
pub mod numcraft_v0_1_3;

pub fn from_v0_1_0_to_0_1_3(data: &Vec<u8>) -> Option<Vec<u8>> {
    let mut save_manager_1 = numcraft_v0_1_0::save_manager::SaveManager::new();
    let mut save_manager_2 = numcraft_v0_1_3::save_manager::SaveManager::new();

    save_manager_1.load_from_file(data).ok()?;

    save_manager_2.set_gamemode(
        if save_manager_1.get_game_mode() == numcraft_v0_1_0::save_manager::GameMode::Creative {
            numcraft_v0_1_3::save_manager::GameMode::Creative
        } else {
            numcraft_v0_1_3::save_manager::GameMode::Survival
        },
    );
    save_manager_2.set_world_name(&save_manager_1.world_info.world_name);
    save_manager_2.set_world_seed(save_manager_1.world_info.world_seed);

    save_manager_2.player_data.pos = (
        4. * 8. - save_manager_1.player_data.pos.0,
        4. * 8. - save_manager_1.player_data.pos.1,
        save_manager_1.player_data.pos.2,
    );
    save_manager_1.player_data.rotation = save_manager_2.player_data.rotation;
    for slot in 0..save_manager_1.player_data.inventory.get_all_slots().len() {
        let old_item_stack: &numcraft_v0_1_0::inventory::ItemStack = save_manager_1
            .player_data
            .inventory
            .get_ref_to_slot(slot)
            .unwrap();
        let new_item_stack = crate::world_converter::numcraft_v0_1_3::inventory::ItemStack::new(
            numcraft_v0_1_3::constants::ItemType::get_from_id(old_item_stack.get_item_type() as u8)
                .unwrap(),
            old_item_stack.get_amount(),
            old_item_stack.creative_slot,
        );
        save_manager_2
            .player_data
            .inventory
            .replace_slot_item_stack(slot, new_item_stack);
    }

    for x in 0..4 {
        for y in 0..4 {
            for z in 0..4 {
                let chunk1 = save_manager_1
                    .get_chunk_at_pos(Vector3::new(x, y, z))
                    .ok()?;
                let old_pos = *chunk1.get_pos();
                let mut chunk2 = crate::world_converter::numcraft_v0_1_3::chunk::Chunk::new(
                    Vector3::new(3 - old_pos.x, 3 - old_pos.y, old_pos.z),
                );
                for bx in 0..8 {
                    for by in 0..8 {
                        for bz in 0..8 {
                            let block_id = chunk1
                                .get_at(Vector3::new(bx as isize, by as isize, bz as isize))
                                .unwrap() as u8;

                            chunk2.set_at(
                                Vector3::new(7 - bx, 7 - by, bz),
                                numcraft_v0_1_3::constants::BlockType::get_from_id(block_id)
                                    .unwrap(),
                            );
                        }
                    }
                }
                save_manager_2.set_chunk(&chunk2);
            }
        }
    }

    Some(save_manager_2.get_raw())
}
