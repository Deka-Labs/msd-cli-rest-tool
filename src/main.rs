use clap::StructOpt;

mod cli;

fn main() {
    let _ = cli::MainCliArgs::parse();
}
