use std::fmt;

struct FrameBufferWriter {
    width: usize,
    height: usize,
    buffer: Vec<Vec<char>>,
    cursor_x: usize,
    cursor_y: usize,
}

impl FrameBufferWriter {
    /// Creates a new FrameBufferWriter with the specified width and height.
    fn new(width: usize, height: usize) -> Self {
        let buffer = vec![vec![' '; width]; height];
        FrameBufferWriter {
            width,
            height,
            buffer,
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    /// Sets the cursor position if it's within valid bounds.
    fn set_cursor(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.cursor_x = x;
            self.cursor_y = y;
        } else {
            self.handle_invalid_position(x, y);
        }
    }

    /// Handles invalid cursor positions gracefully.
    fn handle_invalid_position(&mut self, x: usize, y: usize) {
        eprintln!("Error: Invalid cursor position ({}, {})", x, y);
        self.cursor_x = x.clamp(0, self.width - 1);
        self.cursor_y = y.clamp(0, self.height - 1);
    }

    /// Writes a character at the current cursor position.
    fn write_char(&mut self, ch: char) {
        if ch == '\n' {
            self.cursor_x = 0;
            self.cursor_y += 1;
        } else {
            if self.cursor_x < self.width && self.cursor_y < self.height {
                self.buffer[self.cursor_y][self.cursor_x] = ch;
                self.cursor_x += 1;
            }
        }

        // Handle wrapping across rows
        if self.cursor_x >= self.width {
            self.cursor_x = 0;
            self.cursor_y += 1;
        }

        // Handle scrolling
        if self.cursor_y >= self.height {
            self.scroll_screen();
        }
    }

    /// Scrolls the screen up by one row.
    fn scroll_screen(&mut self) {
        self.buffer.remove(0); // Remove the top row
        self.buffer.push(vec![' '; self.width]); // Add a blank row at the bottom
        self.cursor_y = self.height - 1;
    }

    /// Renders the framebuffer content to the terminal.
    fn render(&self) {
        for row in &self.buffer {
            let row_str: String = row.iter().collect();
            println!("{}", row_str);
        }
    }
}

/// Allows the FrameBufferWriter to be displayed using `println!`.
impl fmt::Display for FrameBufferWriter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.buffer {
            let row_str: String = row.iter().collect();
            writeln!(f, "{}", row_str)?;
        }
        Ok(())
    }
}

fn main() {
    let mut fbw = FrameBufferWriter::new(10, 5);

    let text = "Hello\nWorld! This is a FrameBufferWriter test.";

    for ch in text.chars() {
        fbw.write_char(ch);
    }

    println!("{}", fbw);
}
