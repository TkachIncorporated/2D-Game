use std::collections::HashMap;

use gdnative::{
    api::{CanvasLayer, GridContainer, TextureRect},
    prelude::*,
};

#[derive(NativeClass)]
#[inherit(CanvasLayer)]
pub struct Inventory {
    inventory: HashMap<String, u64>,
    texture: Ref<TextureRect>,
    grid: Ref<GridContainer>,
}

#[methods]
impl Inventory {
    fn new(_owner: TRef<CanvasLayer>) -> Self {
        Inventory {
            inventory: HashMap::new(),
            texture: TextureRect::new().into_shared(),
            grid: GridContainer::new().into_shared(),
        }
    }

    #[export]
    fn _ready(&mut self, owner: TRef<CanvasLayer>) {
        self.texture = unsafe {
            owner
                .get_node_as::<TextureRect>("TextureRect")
                .expect("There's no Texture")
                .assume_shared()
        };

        self.grid = unsafe {
            owner
                .get_node_as::<GridContainer>("TextureRect/GridContainer")
                .expect("There's no Grid")
                .assume_shared()
        };
    }

    #[export]
    fn add_item(&mut self, _owner: TRef<CanvasLayer>, item: String, amount: u64) {
        let initial = self.inventory.get(&item).copied().unwrap_or(0);

        if initial.checked_add(amount).is_none() {
            panic!("Amount too large");
        }

        self.inventory.insert(item, initial + amount);
    }

    #[export]
    fn sub_item(&mut self, _owner: TRef<CanvasLayer>, item: String, amount: u64) {
        let initial = self.inventory.get(&item).copied().unwrap_or(0);

        if amount > initial {
            panic!("Amount too large");
        }

        self.inventory.insert(item, initial - amount);
    }

    #[export]
    fn hide(&mut self, _owner: TRef<CanvasLayer>) {
        let texture = unsafe { self.texture.assume_safe() };

        texture.set_visible(false);

        let grid = unsafe { self.grid.assume_safe() };

        for child in &grid.get_children() {
            let child = child.to_object::<Node>().expect("Child not exist");
            grid.remove_child(child);
            unsafe { child.assume_safe().queue_free() }
        }
    }

    #[export]
    fn show(&mut self, _owner: TRef<CanvasLayer>) {
        let texture = unsafe { self.texture.assume_safe() };

        texture.set_visible(true);

        let grid = unsafe { self.grid.assume_safe() };

        for (item, amount) in self.inventory.iter() {
            let label = Label::new();
            label.set_text(format!("{}:{}", item, amount));
            grid.add_child(label, false);
        }
    }

    #[export]
    fn get_inventory(&mut self, _owner: TRef<CanvasLayer>) -> Vec<(String, u64)> {
        self.inventory.clone().into_iter().collect()
    }
}
