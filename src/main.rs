use std::io::{BufWriter, stdout};
use clap::Parser;
use crate::terminal::Terminal;

mod terminal;
/// Click ESC to exist
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 0 - 100 (float)
    #[arg(short='d', long, default_value="20")]
    distribution: f64,

    /// Speed in milliseconds (Smaller -> faster)
    #[arg(short='z', long, default_value="1000")]
    speed: u64,

    /// Particle character
    #[arg(short= 'p', long, default_value="*")]
    particle: char,

    /// Space character
    #[arg(short='s', long, default_value=" ")]
    space: char,

    /// Space between particles
    #[arg(short= 'B', long)]
    space_between: bool,

    /// Particle color (ANSI)
    #[arg(short= 'P', long)]
    particle_color: Option<u8>,

    /// Background color (ANSI)
    #[arg(short= 'c', long)]
    background_color: Option<u8>,

    /// Space color (ANSI)
    #[arg(short= 'S', long)]
    space_color: Option<u8>
}

fn main()  {
    let args = Args::parse();
    let out = BufWriter::new(stdout());
    let (width, height) = crossterm::terminal::size().expect("Failed to get terminal size");
    let mut terminal = Terminal::new(out, args, height, width);

    loop {
        terminal.event_loop();
        terminal.main_loop();
    }
}


