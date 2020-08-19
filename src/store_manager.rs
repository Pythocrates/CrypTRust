//
// store_manager.rs
// Copyright (C) 2020 Pythocrates
// Distributed under terms of the MIT license.
//
use std::path::PathBuf;
use symlink::{remove_symlink_dir, symlink_dir};

use crate::store::Store;


pub struct StoreManager {
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

    pub fn add_store(&self, name: &str, url: &str, initial_key: Option<&PathBuf>) {
        let store = Store::clone_repo(url, &self.base_path.join(name));
        if let Some(key) = initial_key {
            store.initialize(key);
        }
    }

    pub fn select_store(&self, name: &str) {
        let store_iter = self.stores();
        let store = store_iter.iter().find(|x| x.name() == name).expect("Store not found.");
        remove_symlink_dir(&self.current_path).ok();  // Fine if none found.
        symlink_dir(store.path(), &self.current_path).expect("Error creating symlink.");
    }

    pub fn list_stores(&self) {
        for store in self.stores() {
            println!("{}", store.name());
        }
    }

    pub fn current_store(&self) -> Store{
        Store::new(&self.current_path)
    }

    fn stores(&self) -> Vec<Store> {
        self.base_path.read_dir().expect("Error listing directory contents.")
            .map(|p| Store::new(&p.unwrap().path()))
            .collect()
    }
}





#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
	}
}
