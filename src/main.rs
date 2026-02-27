use clap::Parser;
use watchlist::Cli;

fn main() {
    let cli = Cli::parse();
    dbg!(cli);
}
