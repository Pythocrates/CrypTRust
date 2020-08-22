//
// store.rs
// Copyright (C) 2020 Pythocrates
// Distributed under terms of the MIT license.
//

use std::path::PathBuf;
use edit::edit_file;
use git2::{Cred, MergeOptions, Repository, RemoteCallbacks, FetchOptions};
use git2::build::RepoBuilder;


struct Secret {
    path: PathBuf,
    user_keys_path: PathBuf,
}

impl Secret {
    pub fn show(&self) {
        edit_file(&self.path).ok();
    }
}


pub struct Store {
    repo_path: PathBuf,
    //repo,
    user_keys_path: PathBuf,
    secret: Secret,
}

impl Store {
    pub fn new(path: &PathBuf) -> Self {
        let repo_path = path.canonicalize().expect("Error getting the canonical path.");
        let user_keys_path = repo_path.join("user-keys");
        Self {
            //repo_path: PathBuf::from(path),
            secret: Secret{
                path: repo_path.join("keys.asc"),
                user_keys_path: user_keys_path.clone(),
            },
            repo_path,
            // TODO: repo
            user_keys_path,
        }
    }

    pub fn name(&self) -> &str {
        self.repo_path.as_path().file_stem().expect("Error getting the file stem.").to_str().unwrap()
    }

    pub fn path(&self) ->&PathBuf {
        &self.repo_path
    }

    // based on https://stackoverflow.com/a/58022401
    fn get_remote(&self) {
        let repo = Repository::open(self.path()).expect("Error opening local repository.");
        let ours = repo.head().expect("Failed getting HEAD.").resolve().expect("Failed resolving HEAD.").peel_to_commit().expect("Failed peeling reference");
        let theirs = repo.find_reference("FETCH_HEAD").expect("Failed finding FETCH_HEAD.").peel_to_commit().expect("Failed peeling reference"); 
        let _index = repo.merge_commits(&ours, &theirs, Some(&MergeOptions::new()));

        //repo.find_remote("origin").expect("No remote origin found.").fetch();
    }

    pub fn show(&self) {
        self.get_remote();

        // TODO: decrypt and display
        // Python:
        // with self._secret.decrypted as clear_file:
        //     run(['less', clear_file], check=True)
        self.secret.show();

    }

    pub fn clone_repo(url: &str, path: &PathBuf) -> Self {
        let mut builder = RepoBuilder::new();
        let mut callbacks = RemoteCallbacks::new();
        let mut fetch_options = FetchOptions::new();

        callbacks.credentials(|_, _, _| {
            let credentials = Cred::ssh_key_from_agent("git").expect("Failed getting ssh key from agent.");
            Ok(credentials)
        });

        fetch_options.remote_callbacks(callbacks);

        builder.fetch_options(fetch_options);

        let repo = builder.clone(url, path).expect("Failed cloning repository.");
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
