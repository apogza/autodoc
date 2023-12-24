#[cfg(test)]
mod vehicules {

    use autodoc::{MaintenanceItem, Vehicule};
    use uuid::Uuid;

    #[test]
    fn test_item_mileage() {
        let item: MaintenanceItem = MaintenanceItem::new(String::from("Engine oil"), 205000, 10000);

        assert!(!item.is_for_change(214000));
        assert!(item.is_for_change(216000));
    }

    #[test]
    fn test_item_mileage_update() {
        let mut item: MaintenanceItem =
            MaintenanceItem::new(String::from("Engine oil"), 205000, 10000);

        assert!(item.is_for_change(216000));
        item.set_mileage(20000);

        assert!(!item.is_for_change(216000));
    }

    #[test]
    fn test_item_installed_at_update() {
        let mut item: MaintenanceItem =
            MaintenanceItem::new(String::from("Engine oil"), 205000, 10000);

        assert!(item.is_for_change(216000));
        item.set_installed_at(210000);
        assert!(!item.is_for_change(216000));
    }

    #[test]
    fn test_vehicule_maintenance_items() {
        let item = MaintenanceItem::new(String::from("Engine oil"), 205000, 10000);

        let mut vehicule: Vehicule = Vehicule::new(String::from("Audi A3"), 210000);
        vehicule.add_maintenance_item(item);

        let mut check_items: Option<Vec<Uuid>> = vehicule.check_maintenance_items();

        assert!(check_items.is_none());

        vehicule.update_mileage(220000);
        check_items = vehicule.check_maintenance_items();
        assert!(check_items.is_some());

        let items: Vec<Uuid> = check_items.unwrap();

        let item = vehicule.get_maintenance_item(&items[0]).unwrap();
        assert!(items.len() == 1);
        assert_eq!(item.get_label(), "Engine oil");
    }

    #[test]
    fn test_update_vehicule_maintenance_item() {
        let item = MaintenanceItem::new(String::from("Engine oil"), 205000, 10000);

        let mut vehicule: Vehicule = Vehicule::new(String::from("Audi A3"), 210000);
        vehicule.add_maintenance_item(item);

        let mut check_items: Option<Vec<Uuid>> = vehicule.check_maintenance_items();

        assert!(check_items.is_none());

        vehicule.update_mileage(220000);
        check_items = vehicule.check_maintenance_items();
        assert!(check_items.is_some());

        let items: Vec<Uuid> = check_items.unwrap();

        assert!(items.len() == 1);
        let item_id = &items[0];

        let item: &mut MaintenanceItem = vehicule.get_maintenance_item_mut(item_id).unwrap();

        item.set_installed_at(220000);

        check_items = vehicule.check_maintenance_items();
        assert!(check_items.is_none());
    }
}
