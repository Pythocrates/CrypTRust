use std::path::{Path, PathBuf};
use home::home_dir;
use std::io::Result;
use std::fs::ReadDir;
use std::fs::read_dir;



#[derive(Debug)]
struct Store {
    repo_path: PathBuf,
}

#[derive(Debug)]
struct StoreManager {
    base_path: PathBuf,
    current_path: PathBuf,
}

impl StoreManager {
    pub fn new(base_path: PathBuf) -> Self {
        let config_path = base_path.join(".config").join("spykes");
        Self {
            base_path: config_path.clone(),
            current_path: config_path.join("current_store") ,
        }
    }

    //pub fn select_store(&self, name: &str) {
    //    self.current_path = 
    //}

    pub fn list_stores(&self) {
        for path in self.stores().unwrap() {
            println!("{}", path.expect("Guru Meditation 123")
                     .path().file_name().unwrap()
                     .to_str().unwrap());
        }
    }

    fn stores(&self)  -> Result<ReadDir> {
        self.base_path.read_dir()
    }
}



// Test phase: list-stores as the default action.
fn main() {
    // TODO: add arg parsing; for now, we assume list-stores as the action
    let manager = StoreManager::new(home_dir().expect("User home not found!"));
    println!("{:?}", manager);

    manager.list_stores();
}
