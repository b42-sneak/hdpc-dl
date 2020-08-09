mod cli;
mod core;

fn main() {
    println!("Copyright 2020 b42-sneak; All rights reserved.\n");

    // TODO maybe move the CLI module here and remove this
    cli::exec_cli();
}
