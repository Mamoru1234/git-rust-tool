use std::env;

use git2::{BranchType, Repository, StatusOptions};
use inquire::Confirm;
use log::info;
use seahorse::{Command, Context};

fn check_has_changes(repo: &Repository) -> bool {
  let mut status_options = StatusOptions::new();
  let statuses = &repo.statuses(Some(status_options.include_ignored(false))).unwrap();
  return statuses.len() != 0;
}

fn set_branch(repo: &Repository, name: &str) {
  let reference = repo.resolve_reference_from_short_name(name).unwrap();
  let head_ref = repo.head().unwrap();
  if head_ref == reference {
    info!("Already on target branch");
    return;
  }
  let has_changes = check_has_changes(repo);
  if has_changes {
    panic!("Cannot change branch due to active changes");
  }
  let branch_ref_name = reference.name().unwrap();
  let obj = repo.revparse_single(branch_ref_name).unwrap();
  repo.checkout_tree(
    &obj,
    None
  ).unwrap();
  repo.set_head(branch_ref_name).unwrap();
  info!("Branch set to {}", name);
}

fn clean_up(_ctx: &Context) {
  let target_repository = env::current_dir().unwrap();
  info!("Clean-up command {:?}", target_repository);
  let repo = match Repository::open(target_repository) {
    Ok(repo) => repo,
    Err(e) => panic!("failed to open: {}", e),
  };
  set_branch(&repo, "master");
  for branch_it in repo.branches(Some(BranchType::Local)).unwrap() {
    let (mut branch, _) = branch_it.unwrap();
    let branch_name = String::from(branch.name().unwrap().unwrap());
    if branch_name == "master" {
      continue;
    }
    let should_be_removed = Confirm::new(&format!("Do you want to remove {}?", branch_name))
      .with_default(false)
      .prompt()
      .unwrap();
    if should_be_removed {
      branch.delete().unwrap();
      println!("Branch name {} removed", branch_name);
    }
  }
}

pub fn generate_clean_up_command() -> Command {
  Command::new("clean-up").action(clean_up)
}
