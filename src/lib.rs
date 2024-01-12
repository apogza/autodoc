use std::collections::HashMap;
use uuid::Uuid;

pub type Mileage = u32;

pub struct MaintenanceItem {
    id: Uuid,
    label: String,
    installed_at: Mileage,
    mileage: Mileage,
}

impl MaintenanceItem {
    pub fn new(label: String, installed_at: Mileage, mileage: Mileage) -> MaintenanceItem {
        MaintenanceItem {
            id: uuid::Uuid::new_v4(),
            label,
            installed_at,
            mileage,
        }
    }

    pub fn is_for_change(&self, current_milage: Mileage) -> bool {
        current_milage - self.installed_at > self.mileage
    }

    pub fn set_mileage(&mut self, new_milage: Mileage) {
        self.mileage = new_milage;
    }

    pub fn set_installed_at(&mut self, new_installed_at: Mileage) {
        self.installed_at = new_installed_at;
    }

    pub fn get_label(&self) -> &str {
        return &self.label;
    }

    pub fn get_id(&self) -> &Uuid {
        return &self.id;
    }
}

pub struct Vehicule {
    id: Uuid,
    label: String,
    mileage: Mileage,
    maintenance_items: HashMap<Uuid, MaintenanceItem>,
}

impl Vehicule {
    pub fn new(label: String, mileage: Mileage) -> Vehicule {
        Vehicule {
            id: Uuid::new_v4(),
            label: label,
            mileage: mileage,
            maintenance_items: HashMap::new(),
        }
    }

    pub fn update_mileage(&mut self, new_mileage: Mileage) {
        self.mileage = new_mileage;
    }

    pub fn add_maintenance_item(&mut self, item: MaintenanceItem) {
        if !self.maintenance_items.contains_key(&item.id) {
            self.maintenance_items.insert(item.id, item);
        }
    }

    pub fn check_maintenance_items(&self) -> Option<Vec<Uuid>> {
        if self.maintenance_items.is_empty() {
            return None;
        }

        let mut items: Vec<Uuid> = Vec::new();

        for (_, v) in &self.maintenance_items {
            if v.is_for_change(self.mileage) {
                items.push(v.id.clone());
            }
        }

        if items.len() == 0 {
            return None;
        }

        Some(items)
    }

    pub fn get_maintenance_item(&self, item_id: &Uuid) -> Option<&MaintenanceItem> {
        if self.maintenance_items.contains_key(item_id) {
            return self.maintenance_items.get(item_id);
        }

        None
    }

    pub fn get_maintenance_item_mut(&mut self, item_id: &Uuid) -> Option<&mut MaintenanceItem> {
        if self.maintenance_items.contains_key(item_id) {
            return self.maintenance_items.get_mut(item_id);
        }

        None
    }

    pub fn get_label(&self) -> &str {
        &self.label
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }
}
