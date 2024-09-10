use seahorse::Command;

pub fn recreate_branch_command() -> Command {
  Command::new("recreate-branch").alias("rb")
}