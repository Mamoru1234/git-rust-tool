extern crate dotenv;
use dotenv::dotenv;
use git_rust_tool::commands;
use std::env;
use seahorse::App;

fn main() {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let args: Vec<String> = env::args().collect();
    let app = App::new("git-rust-tool")
        .version(env!("CARGO_PKG_VERSION"))
        .command(commands::generate_clean_up_command());
    app.run(args);
}
