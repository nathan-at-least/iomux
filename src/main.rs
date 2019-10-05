#![deny(warnings)]

mod argparse;

fn main() {
    use argparse::build_commands;
    use std::env::args;

    println!("{:?}", build_commands(args()));
}
