use std::{thread};
use std::collections::VecDeque;
use std::io::{stdout, Write};
use std::time::Duration;
use clap::{Parser};
use crossterm::{QueueableCommand, terminal};
use crossterm::terminal::{Clear, ClearType};
use itertools::Itertools;
use rand::{Rng, thread_rng};

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

fn main() {
    let args = Args::parse();

    clear_screen().expect("Unsupported terminal");

    let chance = args.distribution as f64 / u8::MAX as f64;
    let (width,height) = terminal::size().expect("Unsupported terminal");
    let mut vec_print = VecDeque::from(vec![" ".repeat(width as usize);height as usize]);

    loop {
        printing(width as usize,chance, &args, &mut vec_print)
    }
}

fn printing(width: usize,chance: f64, args: &Args, vec_print: &mut VecDeque<String>) {
    let line = generate_line(width, chance, args);
    vec_print.pop_back();
    vec_print.push_front(line);

    let print_string =  vec_print.iter().join("\n");
    print!("{}", print_string);

    thread::sleep(Duration::from_millis(args.speed));
    clear_screen().expect("Unsupported terminal");
}

fn generate_line(width: usize, chance: f64, args: &Args) -> String {
    let mut builder = String::with_capacity(width);
    for _ in 0..width {
        let last_char = builder.chars().last().unwrap();
        builder.push(gen_char(chance, args,last_char));
    }
    builder
}

fn gen_char(chance: f64, args: &Args, last_char: char) -> char {
    let random = thread_rng().gen_bool(chance);
    if random {
        if args.space_between && last_char == args.snowflake {
            args.air
        } else {
            args.snowflake
        }
    } else {
        args.air
    }
}

fn clear_screen() -> Result<(), std::io::Error> {
    let mut out = stdout();
    out.queue(Clear(ClearType::All))?;
    out.flush()?;
    Ok(())
}


