use home::home_dir;

mod store;
mod store_manager;
use store_manager::StoreManager;

// Test phase: list-stores as the default action.
fn main() {
    // TODO: add arg parsing; for now, we assume list-stores as the action
    let manager = StoreManager::new(home_dir().expect("User home not found!"));

    manager.list_stores();
    manager.select_store("test_1");

    println!("{}", manager.current_store().name());
}
