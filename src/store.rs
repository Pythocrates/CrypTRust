//
// store.rs
// Copyright (C) 2020 Pythocrates
// Distributed under terms of the MIT license.
//

use std::path::PathBuf;
use git2::{MergeOptions, Repository};


pub struct Store {
    repo_path: PathBuf,
}

impl Store {
    pub fn new(path: &PathBuf) -> Self {
        Self {
            //repo_path: PathBuf::from(path),
            repo_path: path.canonicalize().expect("Error getting the canonical path."),
        }
    }

    pub fn name(&self) -> &str {
        self.repo_path.as_path().file_stem().expect("Error getting the file stem.").to_str().unwrap()
    }

    pub fn path(&self) ->&PathBuf {
        &self.repo_path
    }

    fn get_remote(&self) {
        let repo = Repository::open(self.path()).expect("Error opening local repository.");
        let ours = repo.head().expect("Failed getting HEAD.").resolve().expect("Failed resolving HEAD.").peel_to_commit().expect("Failed peeling reference");
        let theirs = repo.find_reference("FETCH_HEAD").expect("Failed finding FETCH_HEAD.").peel_to_commit().expect("Failed peeling reference"); 
        let _index = repo.merge_commits(&ours, &theirs, Some(&MergeOptions::new()));

        //repo.find_remote("origin").expect("No remote origin found.").fetch();
    }

    // based on https://stackoverflow.com/a/58022401
    fn get_remote_2(&self) {
        "AA";
    }

    pub fn clone_repo(url: &str, path: &PathBuf) -> Self {
        Repository::clone(url, path).expect("Failed to clone.");
        Self::new(path)
    }

    pub fn initialize(&self, public_key_path: &PathBuf) {
        // TODO: implement
    }
}




#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
	}
}
