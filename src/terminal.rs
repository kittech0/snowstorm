use std::io::Write;
use std::process::exit;
use std::thread;
use std::time::Duration;
use crossterm::event::{Event, KeyCode, poll, read};
use crossterm::{cursor, execute, queue};
use crossterm::cursor::{MoveToColumn, MoveToRow};
use crossterm::style::Color::Reset;
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, StyledContent, Stylize};
use crossterm::terminal::{BeginSynchronizedUpdate, Clear, ClearType, disable_raw_mode, DisableLineWrap, enable_raw_mode, EnableLineWrap, EndSynchronizedUpdate, EnterAlternateScreen, LeaveAlternateScreen, ScrollDown, SetTitle};
use rand::{Rng, thread_rng};
use crate::Args;

pub struct Terminal<T: Write> {
    out: T,
    height: u16,
    width: u16,
    args: Args,
    chance: f64,
}

impl<T: Write> Terminal<T> {
    pub fn new(out: T, args: Args, height: u16, width: u16) -> Self {
        let chance = args.distribution / 100.0;
        let mut terminal = Terminal { out, height, width, args, chance};
        terminal.setup();
        terminal
    }

    pub fn main_loop(&mut self) {
        queue!(self.out, ScrollDown(1)).expect("Failed to move cursor");
        self.queue_chars();
        queue!(self.out, MoveToColumn(0)).expect("Failed to move cursor");
        self.out.flush().expect("Failed to flush");
        thread::sleep(Duration::from_millis(self.args.speed));
    }

    pub fn event_loop(&mut self) {
        if let Ok(true) = poll(Duration::from_millis(5)) {
            match read().expect("Failed to read event") {
                Event::Resize(width, height) => {
                    if self.width < width {
                        execute!(self.out,Clear(ClearType::All)).expect("Failed to clear screen");
                    }
                    self.height = height;
                    self.width = width;
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

    fn setup(&mut self) {
        enable_raw_mode().expect("Failed to enable raw mode");
        queue!(self.out,EnterAlternateScreen).expect("Failed to enter alternate screen");
        queue!(self.out,DisableLineWrap).expect("Failed to disable line wrap");
        queue!(self.out,Clear(ClearType::All)).expect("Failed to clear screen");
        queue!(self.out,cursor::Hide).expect("Failed to hide cursor");
        queue!(self.out,SetTitle("Snowstorm")).expect("Failed to set title");
        queue!(self.out,MoveToRow(0)).expect("Failed to move cursor");
        queue!(self.out,BeginSynchronizedUpdate).expect("Failed to begin synchronized update");
        if let Some(color) = self.args.background_color {
            queue!(self.out,SetBackgroundColor(Color::AnsiValue(color))).expect("Failed to set color");
        }
        self.out.flush().expect("Failed to flush");
    }

    fn restore(&mut self) {
        disable_raw_mode().expect("Failed to disable raw mode");
        queue!(self.out,EnableLineWrap).expect("Failed to enable line wrap");
        queue!(self.out,cursor::Show).expect("Failed to show cursor");
        queue!(self.out,SetTitle("")).expect("Failed to set title");
        queue!(self.out,LeaveAlternateScreen).expect("Failed to leave alternate screen");
        queue!(self.out,EndSynchronizedUpdate).expect("Failed to begin synchronized update");
        queue!(self.out,ResetColor).expect("Failed to reset color");
        self.out.flush().expect("Failed to flush");
    }

    fn queue_chars(&mut self) {
        let mut last_char = self.args.space;
        for _ in 0..self.width {
            let new_char = self.generate_char(last_char);
            last_char = *new_char.content();
            queue!(self.out, Print(&new_char)).expect("Failed to write");
        }
    }

    fn generate_char(&self, last_char: char) -> StyledContent<char> {
        let random = thread_rng().gen_bool(self.chance);
        let space_color = if let Some(color) = self.args.space_color {
            Color::AnsiValue(color)
        } else {
            Reset
        };
        let particle_color = if let Some(color) = self.args.particle_color {
            Color::AnsiValue(color)
        } else {
            Reset
        };

        if random {
            if self.args.space_between && last_char == self.args.particle {
                self.args.space.with(space_color)
            } else {
                self.args.particle.with(particle_color)
            }
        } else {
            self.args.space.with(space_color)
        }
    }
}

impl<T: Write> Drop for Terminal<T> {
    fn drop(&mut self) {
        self.restore()
    }
}