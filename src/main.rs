// List all to-level modules here
mod cli;
mod core;

/// The main function
fn main() {
    println!("Copyright 2020 b42-sneak; All rights reserved.\n");

    cli::handle_commands::exec_cli();
}
