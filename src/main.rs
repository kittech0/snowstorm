use std::io::{BufWriter, stdout};
use clap::Parser;
use crate::terminal::Terminal;

mod terminal;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // 0 - 255
    #[arg(short, long, default_value="50")]
    distribution: u8,

    // Speed in milliseconds (Smaller -> faster)
    #[arg(short='S', long, default_value="1000")]
    speed: u64,

    #[arg(short, long, default_value="*")]
    snowflake: char,

    #[arg(short, long, default_value=" ")]
    air: char,

    #[arg(short = 'B', long)]
    space_between: bool,
}

fn main()  {
    let args = Args::parse();
    let out = BufWriter::new(stdout());
    let (width, height) = crossterm::terminal::size().expect("Failed to get terminal size");
    let mut terminal = Terminal::new(out, args, height as usize, width as usize);

    terminal.setup();
    loop {
        terminal.event_loop();
        terminal.main_loop();
    }
}


