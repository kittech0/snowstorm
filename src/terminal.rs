use std::io::Write;
use std::process::exit;
use std::thread;
use std::time::Duration;
use crossterm::event::{Event, KeyCode, poll, read};
use crossterm::{cursor, execute, queue};
use crossterm::cursor::{MoveLeft, MoveToRow};
use crossterm::style::Print;
use crossterm::terminal::{BeginSynchronizedUpdate, Clear, ClearType, disable_raw_mode, DisableLineWrap, enable_raw_mode, EnableLineWrap, EndSynchronizedUpdate, EnterAlternateScreen, LeaveAlternateScreen, ScrollDown, SetTitle};
use rand::{Rng, thread_rng};
use crate::Args;

pub struct Terminal<T: Write> {
    out: T,
    height: usize,
    width: usize,
    args: Args,
    chance: f64,
}

impl<T: Write> Terminal<T> {
    pub fn new(out: T, args: Args, height: usize, width: usize) -> Self {
        let chance = (args.distribution as f64) / 255.0;
        Terminal { out, height, width, args, chance}
    }

    pub fn setup(&mut self) {
        enable_raw_mode().expect("Failed to enable raw mode");
        queue!(self.out,EnterAlternateScreen).expect("Failed to enter alternate screen");
        queue!(self.out,DisableLineWrap).expect("Failed to disable line wrap");
        queue!(self.out,Clear(ClearType::All)).expect("Failed to clear screen");
        queue!(self.out,cursor::Hide).expect("Failed to hide cursor");
        queue!(self.out,SetTitle("Snowstorm")).expect("Failed to set title");
        queue!(self.out,MoveToRow(0)).expect("Failed to move cursor");
        queue!(self.out,BeginSynchronizedUpdate).expect("Failed to begin synchronized update");
        self.out.flush().expect("Failed to flush");
    }

    pub fn main_loop(&mut self) {
        let line = self.generate_line();
        let size = line.len();
        queue!(self.out, ScrollDown(1)).expect("Failed to move cursor");
        queue!(self.out, Print(line)).expect("Failed to write");
        queue!(self.out, MoveLeft(size as u16)).expect("Failed to move cursor");
        self.out.flush().expect("Failed to flush");
        thread::sleep(Duration::from_millis(self.args.speed));
    }

    pub fn event_loop(&mut self) {
        if let Ok(true) = poll(Duration::from_millis(5)) {
            match read().expect("Failed to read event") {
                Event::Resize(width, height) => {
                    if self.width < width as usize {
                        execute!(self.out,Clear(ClearType::All)).expect("Failed to clear screen");
                    }
                    self.height = height as usize;
                    self.width = width as usize;
                }
                Event::Key(event) => {
                    if event.code == KeyCode::Esc {
                        self.restore();
                        exit(0);
                    }
                }
                _ => {}
            }
        }
    }

    fn restore(&mut self) {
        disable_raw_mode().expect("Failed to disable raw mode");
        queue!(self.out,EnableLineWrap).expect("Failed to enable line wrap");
        queue!(self.out,cursor::Show).expect("Failed to show cursor");
        queue!(self.out,SetTitle("")).expect("Failed to set title");
        queue!(self.out,LeaveAlternateScreen).expect("Failed to leave alternate screen");
        queue!(self.out,EndSynchronizedUpdate).expect("Failed to begin synchronized update");
        self.out.flush().expect("Failed to flush");
    }

    fn generate_line(&self) -> String {
        let mut builder = String::with_capacity(self.width);
        for _ in 0..self.width {
            let last_char = builder.chars().last().unwrap_or(' ');
            builder.push(self.gen_char(last_char));
        }
        builder
    }

    fn gen_char(&self, last_char: char) -> char {
        let random = thread_rng().gen_bool(self.chance);
        if random {
            if self.args.space_between && last_char == self.args.snowflake {
                self.args.air
            } else {
                self.args.snowflake
            }
        } else {
            self.args.air
        }
    }
}

impl<T: Write> Drop for Terminal<T> {
    fn drop(&mut self) {
        self.restore()
    }
}