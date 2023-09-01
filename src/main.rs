use clap::Parser;
use mine_some_numbers_example::{args::Args, run};

fn main() {
    run(Args::parse());
}
