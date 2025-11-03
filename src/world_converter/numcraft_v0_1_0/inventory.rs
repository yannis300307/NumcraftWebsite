use core::usize;

use serde::{Deserialize, Serialize};

use crate::world_converter::numcraft_v0_1_0::constants::ItemType;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemStack {
    item_type: ItemType,
    amount: u8,
    pub(crate) creative_slot: bool,
}

impl ItemStack {
    pub const fn void() -> Self {
        ItemStack {
            item_type: ItemType::Air,
            amount: 0,
            creative_slot: false,
        }
    }

    pub fn get_item_type(&self) -> ItemType {
        self.item_type
    }
    pub fn get_amount(&self) -> u8 {
        if self.creative_slot {
            self.item_type.get_max_stack_amount()
        } else {
            self.amount
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Inventory {
    slots: Vec<ItemStack>,
    pub modified: bool,
}

/// A generic inventory. Can be the player inventory, a chest inventory, etc...
impl Inventory {
    pub fn new(size: usize) -> Self {
        let mut slots = Vec::with_capacity(size);
        for _ in 0..size {
            slots.push(ItemStack::void());
        }
        Inventory {
            slots: slots,
            modified: true,
        }
    }
    
    pub fn get_ref_to_slot(&self, slot_index: usize) -> Option<&ItemStack> {
        if slot_index >= self.slots.len() {
            None
        } else {
            let item_stack = &self.slots[slot_index];

            Some(item_stack)
        }
    }

    pub fn get_all_slots(&self) -> &Vec<ItemStack> {
        &self.slots
    }
}
