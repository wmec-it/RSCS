use crossterm: :{
    cursor: :{MoveTo, RestorePosition, SavePosition},
    execute,
    style: :{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, Clear, ClearType, ScrollUp, SetSize},
};
use std::io::{stdout, Write};
use std::sync: :{Arc, Mutex};

use crate::conf::vars::MAIN_THEME;

/// Represents a sticky progress bar that stays at the top of the terminal
pub struct StickyProgressBar {
    current:  usize,
    total: usize,
    title: String,
    bar_width: u16,
}

impl StickyProgressBar {
    /// Creates a new progress bar
    pub fn new(total: usize, title: &str) -> Self {
        Self {
            current: 0,
            total,
            title: title.to_string(),
            bar_width: 40,
        }
    }

    /// Initializes the progress bar display at the top of the terminal
    pub fn init(&self) {
        let mut stdout = stdout();
        
        // Save cursor position
        execute!(stdout, SavePosition).unwrap();
        
        // Move to top and clear the first two lines for progress bar
        execute!(stdout, MoveTo(0, 0)).unwrap();
        execute!(stdout, Clear(ClearType::CurrentLine)).unwrap();
        
        // Draw initial progress bar
        self.draw_bar();
        
        // Scroll content down to make room
        execute!(stdout, ScrollUp(2)).unwrap();
        
        // Move cursor below the progress bar area
        execute!(stdout, MoveTo(0, 2)).unwrap();
        
        stdout.flush().unwrap();
    }

    /// Increments progress by 1 and redraws the bar
    pub fn increment(&mut self) {
        if self.current < self.total {
            self.current += 1;
            self.update();
        }
    }

    /// Sets progress to a specific value
    pub fn set_progress(&mut self, value: usize) {
        self.current = value. min(self.total);
        self.update();
    }

    /// Updates the progress bar display
    fn update(&self) {
        let mut stdout = stdout();
        
        // Save current cursor position
        execute!(stdout, SavePosition).unwrap();
        
        // Move to top and redraw
        execute!(stdout, MoveTo(0, 0)).unwrap();
        self.draw_bar();
        
        // Restore cursor position
        execute!(stdout, RestorePosition).unwrap();
        stdout.flush().unwrap();
    }

    /// Draws the progress bar
    fn draw_bar(&self) {
        let mut stdout = stdout();
        
        let percentage = if self.total > 0 {
            (self.current as f64 / self.total as f64 * 100.0) as u8
        } else {
            0
        };
        
        let filled = (self.bar_width as f64 * self.current as f64 / self.total as f64) as u16;
        let empty = self. bar_width - filled;
        
        // Clear the line
        execute!(stdout, Clear(ClearType::CurrentLine)).unwrap();
        
        // Print title and progress info
        execute!(
            stdout,
            SetForegroundColor(Color:: Rgb {
                r: u8::from_str_radix(&MAIN_THEME. primary[0..2], 16).unwrap_or(245),
                g:  u8::from_str_radix(&MAIN_THEME.primary[2.. 4], 16).unwrap_or(126),
                b:  u8::from_str_radix(&MAIN_THEME.primary[4.. 6], 16).unwrap_or(32),
            }),
            Print(format!("{} ", self.title)),
            ResetColor,
            Print("["),
            SetForegroundColor(Color::Rgb {
                r: u8::from_str_radix(&MAIN_THEME.success[0..2], 16).unwrap_or(105),
                g:  u8::from_str_radix(&MAIN_THEME.success[2.. 4], 16).unwrap_or(255),
                b:  u8::from_str_radix(&MAIN_THEME.success[4.. 6], 16).unwrap_or(144),
            }),
            Print("█". repeat(filled as usize)),
            ResetColor,
            Print("░". repeat(empty as usize)),
            Print("] "),
            Print(format!("{}/{} ({}%)", self.current, self.total, percentage)),
        ).unwrap();
        
        // Move to next line for separator
        execute!(stdout, MoveTo(0, 1)).unwrap();
        execute!(stdout, Clear(ClearType:: CurrentLine)).unwrap();
        execute!(
            stdout,
            SetForegroundColor(Color::Rgb {
                r:  u8::from_str_radix(&MAIN_THEME.primary[0..2], 16).unwrap_or(245),
                g: u8::from_str_radix(&MAIN_THEME. primary[2..4], 16).unwrap_or(126),
                b:  u8::from_str_radix(&MAIN_THEME.primary[4.. 6], 16).unwrap_or(32),
            }),
            Print("─".repeat(60)),
            ResetColor,
        ).unwrap();
        
        stdout.flush().unwrap();
    }

    /// Completes the progress bar
    pub fn finish(&mut self) {
        self.current = self.total;
        self.update();
        
        let mut stdout = stdout();
        execute!(stdout, MoveTo(0, 2)).unwrap();
        stdout.flush().unwrap();
    }
}

/// Thread-safe wrapper for progress bar
pub type SharedProgressBar = Arc<Mutex<StickyProgressBar>>;

/// Creates a new shared progress bar
pub fn create_shared_progress_bar(total: usize, title: &str) -> SharedProgressBar {
    Arc::new(Mutex::new(StickyProgressBar::new(total, title)))
}
