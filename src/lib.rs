extern crate wasm_bindgen;
extern crate wasm_bindgen_futures;
extern crate git2;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::*;
use std::process::Command;
use git2::{Repository, BranchType};
use git2::build::CheckoutBuilder;
use std::path::Path;

#[wasm_bindgen]
pub async fn clone_repo(repo_url: &str, branch: &str, dest_path: &str) -> Result<(), JsValue> {
    let repo = Repository::clone(repo_url, dest_path).expect("Failed to clone repo");
    let branch = repo.find_branch(branch, BranchType::Local).expect("Failed to find branch");
    let obj = branch.get().peel(git2::ObjectType::Commit).expect("Failed to get commit");
    let mut checkout = CheckoutBuilder::new();
    repo.checkout_tree(&obj, Some(&mut checkout)).expect("Failed to checkout");
    Ok(())
}
#[wasm_bindgen]
pub async fn turn_off_computer() -> Result<(), JsValue> {
    Command::new("shutdown").arg("-s").arg("-t").arg("0").spawn().expect("Failed to shutdown");
    Ok(())
}
#[wasm_bindgen]
pub async fn restart_computer() -> Result<(), JsValue> {
    Command::new("shutdown").arg("-r").arg("-t").arg("0").spawn().expect("Failed to restart");
    Ok(())
}