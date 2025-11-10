mod data_io;
mod structures;
mod logic;
mod frontend;

use color_eyre::Result;
use frontend::cards::Cards;
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <path to file>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let mut card_rendering = Cards::new(file_path.to_string());
    card_rendering.execute()
}
